pub mod error;
pub use error::*;
pub mod arguments;
pub use arguments::Arguments;
mod cursor;
use cursor::*;
mod environment;
use environment::Environment;
mod lexer;
mod parser;
use parser::Parser;
mod token;
use token::*;
mod variable;
use variable::*;

/// Representation of a calculator instance.
pub struct Calculator {
	env: Environment,
}

impl Calculator {
	/// This creates a new empty instance of Calculator.
	pub fn new() -> Self {
		Self {
			env: Environment::new(),
		}
	}

	/// Initialize the std lib on this calculator instance.
	pub fn init_std(&mut self) {
		self.env.init_std()
	}

	/// Adds a custom function/constant to this calculator instance.
	/// This overrides any existing variable/function with this name without any warning.
	///
	/// # Example
	///
	/// ```
	/// use calculator::*;
	///
	/// fn twice(args: &mut dyn Arguments) -> Result<f32, Error> {
	///     let arg = args.get_next_arg()?;
	///     Ok(arg * 2.0)
	/// }
	///
	/// let mut calculator = Calculator::new();
	/// calculator.add_custom("twice", twice);
	///
	/// let val = calculator.calculate("twice 4").unwrap();
	/// assert_eq!(val, 8.0);
	/// ```
	pub fn add_custom(&mut self, id: &str, fun: fn(&mut dyn Arguments) -> Result<f32, Error>) {
		self.env.assign_custom(id.to_owned(), fun);
	}

	/// calc.calculates the result of the given expression
	///
	/// # Example
	///
	/// ```
	/// use calculator::*;
	///
	/// let mut calculator = Calculator::new();
	///
	/// let val = calculator.calculate("3 * -(4 + 5)").unwrap();
	/// assert_eq!(val, -27.0);
	/// ```
	///
	/// # Errors
	///
	/// This evaluation can fail if the structure of the input is not valid.
	/// For example if the input contains invalid characters or have bad syntax.
	pub fn calculate(&mut self, input: &str) -> Result<f32, Error> {
		let tokens = Cursor::new(lexer::tokenize(input)?);
		Parser::new(tokens, &mut self.env).evaluate()
	}
}

impl Default for Calculator {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_numerical_literal() {
		let mut calc = Calculator::new();

		assert_eq!(calc.calculate("0").unwrap(), 0.0);
		assert_eq!(calc.calculate("4").unwrap(), 4.0);

		assert_eq!(calc.calculate("0.0").unwrap(), 0.0);
		assert_eq!(calc.calculate("4.5").unwrap(), 4.5);
		assert_eq!(calc.calculate("455.555").unwrap(), 455.555);
	}

	#[test]
	fn test_02_sign() {
		let mut calc = Calculator::new();

		assert_eq!(calc.calculate("-0").unwrap(), 0.0);
		assert_eq!(calc.calculate("-4").unwrap(), -4.0);
		assert_eq!(calc.calculate("-0.0").unwrap(), 0.0);
		assert_eq!(calc.calculate("-4.5").unwrap(), -4.5);
		assert_eq!(calc.calculate("-455.555").unwrap(), -455.555);

		assert_eq!(calc.calculate("+0").unwrap(), 0.0);
		assert_eq!(calc.calculate("+4").unwrap(), 4.0);
		assert_eq!(calc.calculate("+4.5").unwrap(), 4.5);

		assert_eq!(calc.calculate("--4").unwrap(), 4.0);
		assert_eq!(calc.calculate("+-4").unwrap(), -4.0);
	}

	#[test]
	fn test_03_additive() {
		let mut calc = Calculator::new();

		assert_eq!(calc.calculate("0 + 0").unwrap(), 0.0);
		assert_eq!(calc.calculate("4 + 3").unwrap(), 7.0);
		assert_eq!(calc.calculate("4.5 + 3").unwrap(), 7.5);

		assert_eq!(calc.calculate("0 - 0").unwrap(), 0.0);
		assert_eq!(calc.calculate("4 - 7").unwrap(), -3.0);
		assert_eq!(calc.calculate("4.5 - 3").unwrap(), 1.5);
	}

	#[test]
	fn test_04_multiplicative() {
		let mut calc = Calculator::new();

		assert_eq!(calc.calculate("0 * 0").unwrap(), 0.0);
		assert_eq!(calc.calculate("4 * 3").unwrap(), 12.0);
		assert_eq!(calc.calculate("4.5 * 3").unwrap(), 13.5);

		assert_eq!(calc.calculate("0 / 1").unwrap(), 0.0);
		assert_eq!(calc.calculate("12 / 3").unwrap(), 4.0);
		assert_eq!(calc.calculate("4.5 / 3").unwrap(), 1.5);

		assert_eq!(calc.calculate("0 % 1").unwrap(), 0.0);
		assert_eq!(calc.calculate("11 % 3").unwrap(), 2.0);
		assert_eq!(calc.calculate("4.5 % 3").unwrap(), 1.5);
	}

	#[test]
	fn test_05_operation_order() {
		let mut calc = Calculator::new();

		assert_eq!(calc.calculate("3 + 4 * 5").unwrap(), 23.0);
		assert_eq!(calc.calculate("3 * 4 + 5").unwrap(), 17.0);

		assert_eq!(calc.calculate("3 + -4 * 5").unwrap(), -17.0);
		assert_eq!(calc.calculate("3 + -4 * -5").unwrap(), 23.0);
	}

	#[test]
	fn test_06_brackets() {
		let mut calc = Calculator::new();

		assert_eq!(calc.calculate("(3 + 4) * 5").unwrap(), 35.0);
		assert_eq!(calc.calculate("3 * (4 + 5)").unwrap(), 27.0);

		assert_eq!(calc.calculate("3 + -(4 * 5)").unwrap(), -17.0);
		assert_eq!(calc.calculate("3 + -(4 * -5)").unwrap(), 23.0);
		assert_eq!(calc.calculate("(3 + -4) * 5").unwrap(), -5.0);
		assert_eq!(calc.calculate("(3 + -4) * -5").unwrap(), 5.0);
	}

	#[test]
	fn test_07_variables() {
		let mut calc = Calculator::new();

		assert_eq!(calc.calculate("a = 5").unwrap(), 5.0);
		assert_eq!(calc.calculate("a * 10").unwrap(), 50.0);
	}

	#[test]
	fn test_08_function_calls() {
		let mut calc = Calculator::new();
		calc.init_std();

		assert_eq!(calc.calculate("test 16").unwrap(), 8.0);
		assert_eq!(calc.calculate("test 16 * 5").unwrap(), 40.0);
	}

	#[test]
	fn test_09_last_result() {
		let mut calc = Calculator::new();

		match calc.calculate("$") {
			Err(_) => (),
			_ => panic!(),
		}
		assert_eq!(calc.calculate("4 + 5").unwrap(), 9.0);
		assert_eq!(calc.calculate("$ + 3").unwrap(), 12.0);
	}
}
