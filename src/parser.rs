use crate::{
	AddOperator, Arguments, Cursor, Environment, Error, ExpOperator, MulOperator, TokenValue,
	Variable,
};

pub(crate) struct Parser<'e> {
	tokens: Cursor,
	env: &'e mut Environment,
}

impl<'e> Parser<'e> {
	pub(crate) fn new(tokens: Cursor, env: &'e mut Environment) -> Self {
		Self { tokens, env }
	}

	pub(crate) fn evaluate(&mut self) -> Result<f32, Error> {
		let result = self.evaluate_statement()?;

		// check if all tokens are consumed
		self.tokens.expect(&TokenValue::Eof)?;
		self.env.set_last_result(result);

		Ok(result)
	}

	fn evaluate_statement(&mut self) -> Result<f32, Error> {
		match (
			self.tokens
				.current()
				.ok_or(Error::UnexpectedEndOfInput)?
				.value,
			self.tokens.next().ok_or(Error::UnexpectedEndOfInput)?.value,
		) {
			(TokenValue::Identifier(_), TokenValue::Equals) => self.evaluate_assignment(),
			_ => self.evaluate_additive(),
		}
	}

	fn evaluate_assignment(&mut self) -> Result<f32, Error> {
		let id = self.tokens.consume().ok_or(Error::UnexpectedEndOfInput)?;
		match id.value {
			TokenValue::Identifier(id) => {
				self.tokens.expect(&TokenValue::Equals)?;
				let value = self.evaluate_statement()?;
				Ok(self.env.assign(id, value))
			}
			_ => Err(Error::UnexpectedToken {
				token: id.src.clone(),
				start: id.start,
				end: id.end,
			}),
		}
	}

	fn evaluate_additive(&mut self) -> Result<f32, Error> {
		let mut left = self.evaluate_multiplicative()?;

		while let Some(op) = self.tokens.get_add_op()? {
			let right = self.evaluate_multiplicative()?;
			match op {
				AddOperator::Add => left += right,
				AddOperator::Sub => left -= right,
			}
		}

		Ok(left)
	}

	fn evaluate_multiplicative(&mut self) -> Result<f32, Error> {
		let mut left = self.evaluate_exponential()?;

		while let Some(op) = self.tokens.get_mul_op()? {
			let right = self.evaluate_exponential()?;
			match op {
				MulOperator::Mul => left *= right,
				MulOperator::Div => {
					if right == 0.0 {
						return Err(Error::Runtime("Division by 0!"));
					}
					left /= right
				}
				MulOperator::Mod => {
					if right == 0.0 {
						return Err(Error::Runtime("Division by 0!"));
					}
					left %= right
				}
			}
		}

		Ok(left)
	}

	fn evaluate_exponential(&mut self) -> Result<f32, Error> {
		let mut left = self.evaluate_atomic()?;

		while let Some(op) = self.tokens.get_exp_op()? {
			let right = self.evaluate_atomic()?;
			match op {
				ExpOperator::Power => left = left.powf(right),
				ExpOperator::Root => {
					if right == 0.0 {
						return Err(Error::Runtime("Division by 0!"));
					}
					left = left.powf(1.0 / right)
				}
			}
		}

		Ok(left)
	}

	fn evaluate_atomic(&mut self) -> Result<f32, Error> {
		let token = self.tokens.consume().ok_or(Error::UnexpectedEndOfInput)?;
		match token.value {
			TokenValue::Number(val) => Ok(val),
			TokenValue::Identifier(id) => match self.env.get(&id) {
				Some(var) => match var {
					Variable::Var(var) => Ok(*var),
					Variable::Fn(fun) => Ok(fun(self.evaluate_atomic()?)),
					Variable::Custom(fun) => fun(self),
				},
				_ => Err(Error::VariableNotFound {
					var: id,
					start: token.start,
					end: token.end,
				}),
			},
			TokenValue::LastResult => match self.env.get_last_result() {
				Some(var) => Ok(var),
				_ => Err(Error::VariableNotFound {
					var: token.src,
					start: token.start,
					end: token.end,
				}),
			},
			TokenValue::AddOperator(op) => match op {
				AddOperator::Add => self.evaluate_atomic(),
				AddOperator::Sub => Ok(-(self.evaluate_atomic()?)),
			},
			TokenValue::OpenBracket => {
				let value = self.evaluate_additive();
				self.tokens.expect(&TokenValue::CloseBracket)?;
				value
			}
			TokenValue::Eof => Err(Error::UnexpectedEndOfInput),
			_ => Err(Error::UnexpectedToken {
				token: token.src.clone(),
				start: token.start,
				end: token.end,
			}),
		}
	}
}

impl<'e> Arguments for Parser<'e> {
	fn get_next_arg(&mut self) -> Result<f32, Error> {
		self.evaluate_atomic()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Token;

	fn new_t(value: TokenValue) -> Token {
		Token::new(value, "".to_owned(), 0, 0)
	}

	fn new_p<'e>(env: &'e mut Environment, tokens: Vec<Token>) -> Parser<'e> {
		Parser {
			tokens: Cursor::new(tokens),
			env,
		}
	}

	#[test]
	fn test_01_evaluate_atomic_simple() {
		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![new_t(TokenValue::Number(45.56)), new_t(TokenValue::Eof),]
			)
			.evaluate_atomic()
			.unwrap(),
			45.56
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(45.56)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_atomic()
			.unwrap(),
			-45.56
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(45.56)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_atomic()
			.unwrap(),
			-45.56
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(45.56)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_atomic()
			.unwrap(),
			45.56
		);
	}

	#[test]
	fn test_02_evaluate_mul() {
		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_multiplicative()
			.unwrap(),
			12.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(12.0)),
					new_t(TokenValue::MulOperator(MulOperator::Div)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_multiplicative()
			.unwrap(),
			3.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(12.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mod)),
					new_t(TokenValue::Number(7.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_multiplicative()
			.unwrap(),
			5.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_multiplicative()
			.unwrap(),
			-12.0
		);
	}

	#[test]
	fn test_03_evaluate_add() {
		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_additive()
			.unwrap(),
			7.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_additive()
			.unwrap(),
			-1.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_additive()
			.unwrap(),
			7.0
		);
	}

	#[test]
	fn test_04_evaluate_operation_order() {
		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate()
			.unwrap(),
			19.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate()
			.unwrap(),
			16.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate()
			.unwrap(),
			-8.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::ExpOperator(ExpOperator::Power)),
					new_t(TokenValue::Number(2.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate()
			.unwrap(),
			48.0
		);
	}

	#[test]
	fn test_05_blank_input() {
		match new_p(&mut Environment::new(), vec![new_t(TokenValue::Eof)]).evaluate() {
			Err(_) => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_06_only_operator() {
		match new_p(
			&mut Environment::new(),
			vec![
				new_t(TokenValue::AddOperator(AddOperator::Add)),
				new_t(TokenValue::Eof),
			],
		)
		.evaluate()
		{
			Err(_) => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_07_only_numbers() {
		match new_p(
			&mut Environment::new(),
			vec![
				new_t(TokenValue::Number(4.0)),
				new_t(TokenValue::Number(5.0)),
				new_t(TokenValue::Eof),
			],
		)
		.evaluate()
		{
			Err(_) => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_08_wrong_bracket() {
		match new_p(
			&mut Environment::new(),
			vec![
				new_t(TokenValue::OpenBracket),
				new_t(TokenValue::Number(5.0)),
				new_t(TokenValue::OpenBracket),
				new_t(TokenValue::Eof),
			],
		)
		.evaluate()
		{
			Err(_) => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_09_division_by_0() {
		match new_p(
			&mut Environment::new(),
			vec![
				new_t(TokenValue::Number(4.0)),
				new_t(TokenValue::MulOperator(MulOperator::Div)),
				new_t(TokenValue::Number(0.0)),
				new_t(TokenValue::Eof),
			],
		)
		.evaluate()
		{
			Err(_) => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_10_variable_assigment_get() {
		let mut env = Environment::new();

		let mut parser = Parser {
			tokens: Cursor::new(vec![
				new_t(TokenValue::Identifier("a".to_owned())),
				new_t(TokenValue::Equals),
				new_t(TokenValue::Number(34.5)),
				new_t(TokenValue::Eof),
			]),
			env: &mut env,
		};
		// assign
		assert_eq!(parser.evaluate().unwrap(), 34.5);

		let mut parser = Parser {
			tokens: Cursor::new(vec![
				new_t(TokenValue::Identifier("a".to_owned())),
				new_t(TokenValue::AddOperator(AddOperator::Add)),
				new_t(TokenValue::Number(2.0)),
				new_t(TokenValue::Eof),
			]),
			env: &mut env,
		};
		// get
		assert_eq!(parser.evaluate().unwrap(), 36.5);

		let mut parser = Parser {
			tokens: Cursor::new(vec![
				new_t(TokenValue::Identifier("a".to_owned())),
				new_t(TokenValue::Equals),
				new_t(TokenValue::Number(5.4)),
				new_t(TokenValue::Eof),
			]),
			env: &mut env,
		};
		// reassign
		assert_eq!(parser.evaluate().unwrap(), 5.4);

		let mut parser = Parser {
			tokens: Cursor::new(vec![
				new_t(TokenValue::Identifier("a".to_owned())),
				new_t(TokenValue::AddOperator(AddOperator::Add)),
				new_t(TokenValue::Number(2.0)),
				new_t(TokenValue::Eof),
			]),
			env: &mut env,
		};
		// get
		assert_eq!(parser.evaluate().unwrap(), 7.4);
	}

	#[test]
	fn test_11_get_undefined_variable() {
		match new_p(
			&mut Environment::new(),
			vec![
				new_t(TokenValue::Identifier("xyz".to_owned())),
				new_t(TokenValue::Eof),
			],
		)
		.evaluate()
		{
			Err(_) => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_12_function_call() {
		let mut env = Environment::new();
		env.init_std();
		let mut parser = Parser {
			tokens: Cursor::new(vec![
				new_t(TokenValue::Identifier("test".to_owned())),
				new_t(TokenValue::Number(4.0)),
				new_t(TokenValue::Eof),
			]),
			env: &mut env,
		};
		assert_eq!(parser.evaluate().unwrap(), 2.0);

		let mut parser = Parser {
			tokens: Cursor::new(vec![
				new_t(TokenValue::Identifier("test".to_owned())),
				new_t(TokenValue::Number(4.0)),
				new_t(TokenValue::AddOperator(AddOperator::Add)),
				new_t(TokenValue::Number(4.0)),
				new_t(TokenValue::Eof),
			]),
			env: &mut env,
		};
		assert_eq!(parser.evaluate().unwrap(), 6.0);
	}

	#[test]
	fn test_13_last_result() {
		let mut env = Environment::new();
		env.init_std();
		let mut parser = Parser {
			tokens: Cursor::new(vec![new_t(TokenValue::LastResult), new_t(TokenValue::Eof)]),
			env: &mut env,
		};

		// not assigned yet
		match parser.evaluate() {
			Err(_) => (),
			_ => panic!(),
		}

		let mut parser = Parser {
			tokens: Cursor::new(vec![new_t(TokenValue::Number(4.0)), new_t(TokenValue::Eof)]),
			env: &mut env,
		};
		// assign last result
		assert_eq!(parser.evaluate().unwrap(), 4.0);

		let mut parser = Parser {
			tokens: Cursor::new(vec![new_t(TokenValue::LastResult), new_t(TokenValue::Eof)]),
			env: &mut env,
		};
		// use last result
		assert_eq!(parser.evaluate().unwrap(), 4.0);
	}

	#[test]
	fn test_14_evaluate_exp() {
		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::ExpOperator(ExpOperator::Power)),
					new_t(TokenValue::Number(2.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_exponential()
			.unwrap(),
			9.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(8.0)),
					new_t(TokenValue::ExpOperator(ExpOperator::Root)),
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_exponential()
			.unwrap(),
			2.0
		);

		assert_eq!(
			new_p(
				&mut Environment::new(),
				vec![
					new_t(TokenValue::Number(2.0)),
					new_t(TokenValue::ExpOperator(ExpOperator::Power)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(1.0)),
					new_t(TokenValue::Eof),
				]
			)
			.evaluate_exponential()
			.unwrap(),
			0.5
		);
	}
}
