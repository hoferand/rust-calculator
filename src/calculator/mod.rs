pub mod error;
pub use error::*;
pub mod environment;
pub use environment::*;
mod cursor;
use cursor::*;
mod lexer;
mod parser;
mod token;
use token::*;

pub fn calculate(input: &str, env: &mut Environment) -> Result<f32, Error> {
	let mut tokens = Cursor::new(lexer::tokenize(input)?);
	parser::evaluate(&mut tokens, env)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_numerical_literal() {
		assert_eq!(calculate("0", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("4", &mut Environment::new()).unwrap(), 4.0);

		assert_eq!(calculate("0.0", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("4.5", &mut Environment::new()).unwrap(), 4.5);
		assert_eq!(
			calculate("455.555", &mut Environment::new()).unwrap(),
			455.555
		);
	}

	#[test]
	fn test_02_sign() {
		assert_eq!(calculate("-0", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("-4", &mut Environment::new()).unwrap(), -4.0);
		assert_eq!(calculate("-0.0", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("-4.5", &mut Environment::new()).unwrap(), -4.5);
		assert_eq!(
			calculate("-455.555", &mut Environment::new()).unwrap(),
			-455.555
		);

		assert_eq!(calculate("+0", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("+4", &mut Environment::new()).unwrap(), 4.0);
		assert_eq!(calculate("+4.5", &mut Environment::new()).unwrap(), 4.5);

		assert_eq!(calculate("--4", &mut Environment::new()).unwrap(), 4.0);
		assert_eq!(calculate("+-4", &mut Environment::new()).unwrap(), -4.0);
	}

	#[test]
	fn test_03_additive() {
		assert_eq!(calculate("0 + 0", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("4 + 3", &mut Environment::new()).unwrap(), 7.0);
		assert_eq!(calculate("4.5 + 3", &mut Environment::new()).unwrap(), 7.5);

		assert_eq!(calculate("0 - 0", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("4 - 7", &mut Environment::new()).unwrap(), -3.0);
		assert_eq!(calculate("4.5 - 3", &mut Environment::new()).unwrap(), 1.5);
	}

	#[test]
	fn test_04_multiplicative() {
		assert_eq!(calculate("0 * 0", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("4 * 3", &mut Environment::new()).unwrap(), 12.0);
		assert_eq!(calculate("4.5 * 3", &mut Environment::new()).unwrap(), 13.5);

		assert_eq!(calculate("0 / 1", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("12 / 3", &mut Environment::new()).unwrap(), 4.0);
		assert_eq!(calculate("4.5 / 3", &mut Environment::new()).unwrap(), 1.5);

		assert_eq!(calculate("0 % 1", &mut Environment::new()).unwrap(), 0.0);
		assert_eq!(calculate("11 % 3", &mut Environment::new()).unwrap(), 2.0);
		assert_eq!(calculate("4.5 % 3", &mut Environment::new()).unwrap(), 1.5);
	}

	#[test]
	fn test_05_operation_order() {
		assert_eq!(
			calculate("3 + 4 * 5", &mut Environment::new()).unwrap(),
			23.0
		);
		assert_eq!(
			calculate("3 * 4 + 5", &mut Environment::new()).unwrap(),
			17.0
		);

		assert_eq!(
			calculate("3 + -4 * 5", &mut Environment::new()).unwrap(),
			-17.0
		);
		assert_eq!(
			calculate("3 + -4 * -5", &mut Environment::new()).unwrap(),
			23.0
		);
	}

	#[test]
	fn test_06_brackets() {
		assert_eq!(
			calculate("(3 + 4) * 5", &mut Environment::new()).unwrap(),
			35.0
		);
		assert_eq!(
			calculate("3 * (4 + 5)", &mut Environment::new()).unwrap(),
			27.0
		);

		assert_eq!(
			calculate("3 + -(4 * 5)", &mut Environment::new()).unwrap(),
			-17.0
		);
		assert_eq!(
			calculate("3 + -(4 * -5)", &mut Environment::new()).unwrap(),
			23.0
		);
		assert_eq!(
			calculate("(3 + -4) * 5", &mut Environment::new()).unwrap(),
			-5.0
		);
		assert_eq!(
			calculate("(3 + -4) * -5", &mut Environment::new()).unwrap(),
			5.0
		);
	}

	#[test]
	fn test_07_variables() {
		let mut env = Environment::new();
		assert_eq!(calculate("a = 5", &mut env).unwrap(), 5.0);
		assert_eq!(calculate("a * 10", &mut env).unwrap(), 50.0);
	}

	#[test]
	fn test_08_function_calls() {
		let mut env = Environment::new();
		env.init();
		assert_eq!(calculate("test 16", &mut env).unwrap(), 8.0);
		assert_eq!(calculate("test 16 * 5", &mut env).unwrap(), 40.0);
	}

	#[test]
	fn test_09_last_result() {
		let mut env = Environment::new();
		env.init();
		match calculate("$", &mut env) {
			Err(_) => (),
			_ => panic!(),
		}
		assert_eq!(calculate("4 + 5", &mut env).unwrap(), 9.0);
		assert_eq!(calculate("$ + 3", &mut env).unwrap(), 12.0);
	}
}
