pub mod error;
use error::Error;
pub mod environment;
use environment::Environment;
mod lexer;
mod parser;
mod token;

pub fn calculate(input: &String, env: &mut Environment) -> Result<f32, Error> {
	let mut tokens = lexer::tokenize(input)?;
	return parser::evaluate(&mut tokens, env);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_numerical_literal() {
		assert_eq!(
			calculate(&String::from("0"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("4"), &mut environment::new()).unwrap(),
			4.0
		);

		assert_eq!(
			calculate(&String::from("0.0"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("4.5"), &mut environment::new()).unwrap(),
			4.5
		);
		assert_eq!(
			calculate(&String::from("455.555"), &mut environment::new()).unwrap(),
			455.555
		);
	}

	#[test]
	fn test_02_sign() {
		assert_eq!(
			calculate(&String::from("-0"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("-4"), &mut environment::new()).unwrap(),
			-4.0
		);
		assert_eq!(
			calculate(&String::from("-0.0"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("-4.5"), &mut environment::new()).unwrap(),
			-4.5
		);
		assert_eq!(
			calculate(&String::from("-455.555"), &mut environment::new()).unwrap(),
			-455.555
		);

		assert_eq!(
			calculate(&String::from("+0"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("+4"), &mut environment::new()).unwrap(),
			4.0
		);
		assert_eq!(
			calculate(&String::from("+4.5"), &mut environment::new()).unwrap(),
			4.5
		);

		assert_eq!(
			calculate(&String::from("--4"), &mut environment::new()).unwrap(),
			4.0
		);
		assert_eq!(
			calculate(&String::from("+-4"), &mut environment::new()).unwrap(),
			-4.0
		);
	}

	#[test]
	fn test_03_additive() {
		assert_eq!(
			calculate(&String::from("0 + 0"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("4 + 3"), &mut environment::new()).unwrap(),
			7.0
		);
		assert_eq!(
			calculate(&String::from("4.5 + 3"), &mut environment::new()).unwrap(),
			7.5
		);

		assert_eq!(
			calculate(&String::from("0 - 0"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("4 - 7"), &mut environment::new()).unwrap(),
			-3.0
		);
		assert_eq!(
			calculate(&String::from("4.5 - 3"), &mut environment::new()).unwrap(),
			1.5
		);
	}

	#[test]
	fn test_04_multiplicative() {
		assert_eq!(
			calculate(&String::from("0 * 0"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("4 * 3"), &mut environment::new()).unwrap(),
			12.0
		);
		assert_eq!(
			calculate(&String::from("4.5 * 3"), &mut environment::new()).unwrap(),
			13.5
		);

		assert_eq!(
			calculate(&String::from("0 / 1"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("12 / 3"), &mut environment::new()).unwrap(),
			4.0
		);
		assert_eq!(
			calculate(&String::from("4.5 / 3"), &mut environment::new()).unwrap(),
			1.5
		);

		assert_eq!(
			calculate(&String::from("0 % 1"), &mut environment::new()).unwrap(),
			0.0
		);
		assert_eq!(
			calculate(&String::from("11 % 3"), &mut environment::new()).unwrap(),
			2.0
		);
		assert_eq!(
			calculate(&String::from("4.5 % 3"), &mut environment::new()).unwrap(),
			1.5
		);
	}

	#[test]
	fn test_05_operation_order() {
		assert_eq!(
			calculate(&String::from("3 + 4 * 5"), &mut environment::new()).unwrap(),
			23.0
		);
		assert_eq!(
			calculate(&String::from("3 * 4 + 5"), &mut environment::new()).unwrap(),
			17.0
		);

		assert_eq!(
			calculate(&String::from("3 + -4 * 5"), &mut environment::new()).unwrap(),
			-17.0
		);
		assert_eq!(
			calculate(&String::from("3 + -4 * -5"), &mut environment::new()).unwrap(),
			23.0
		);
	}

	#[test]
	fn test_06_brackets() {
		assert_eq!(
			calculate(&String::from("(3 + 4) * 5"), &mut environment::new()).unwrap(),
			35.0
		);
		assert_eq!(
			calculate(&String::from("3 * (4 + 5)"), &mut environment::new()).unwrap(),
			27.0
		);

		assert_eq!(
			calculate(&String::from("3 + -(4 * 5)"), &mut environment::new()).unwrap(),
			-17.0
		);
		assert_eq!(
			calculate(&String::from("3 + -(4 * -5)"), &mut environment::new()).unwrap(),
			23.0
		);
		assert_eq!(
			calculate(&String::from("(3 + -4) * 5"), &mut environment::new()).unwrap(),
			-5.0
		);
		assert_eq!(
			calculate(&String::from("(3 + -4) * -5"), &mut environment::new()).unwrap(),
			5.0
		);
	}

	#[test]
	fn test_07_variables() {
		let mut env = environment::new();
		assert_eq!(calculate(&String::from("a = 5"), &mut env).unwrap(), 5.0);
		assert_eq!(calculate(&String::from("a * 10"), &mut env).unwrap(), 50.0);
	}

	#[test]
	fn test_08_function_calls() {
		let mut env = environment::new();
		env.init();
		assert_eq!(calculate(&String::from("sqrt 16"), &mut env).unwrap(), 4.0);
		assert_eq!(
			calculate(&String::from("sqrt 16 * 5"), &mut env).unwrap(),
			20.0
		);
	}

	#[test]
	fn test_09_last_result() {
		let mut env = environment::new();
		env.init();
		match calculate(&String::from("$"), &mut env) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
		assert_eq!(calculate(&String::from("4 + 5"), &mut env).unwrap(), 9.0);
		assert_eq!(calculate(&String::from("$ + 3"), &mut env).unwrap(), 12.0);
	}
}
