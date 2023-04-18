use std::iter::Peekable;

use crate::calculator::environment::{Environment, Variable};
use crate::calculator::error::Error;
use crate::calculator::token::{AddOperator, MulOperator, Token, TokenValue};

pub fn evaluate(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	env: &mut Environment,
) -> Result<f32, Error> {
	let result = evaluate_statement(tokens, env);

	// check if all tokens are consumed
	if let Ok(value) = result {
		consume(tokens, TokenValue::Eof)?;
		env.set_last_result(value);

		return Ok(value);
	}

	result
}

fn evaluate_statement(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	env: &mut Environment,
) -> Result<f32, Error> {
	let result;
	if let Some(token) = tokens.peek() {
		match token.value {
			TokenValue::Let => result = evaluate_assignment(tokens, env),
			_ => result = evaluate_additive(tokens, env),
		}
	} else {
		return unexpected_end_of_input();
	}

	result
}

fn evaluate_assignment(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	env: &mut Environment,
) -> Result<f32, Error> {
	consume(tokens, TokenValue::Let)?;
	if let Some(token) = tokens.next() {
		match token.value {
			TokenValue::Identifier(id) => {
				consume(tokens, TokenValue::Equals)?;
				let value = evaluate_statement(tokens, env)?;
				return Ok(env.assign(id, value));
			}
			_ => return unexpected_token(token),
		}
	}

	unexpected_end_of_input()
}

fn evaluate_additive(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	env: &mut Environment,
) -> Result<f32, Error> {
	let mut left = evaluate_multiplicative(tokens, env)?;

	while let Some(Token {
		value: TokenValue::AddOperator(_),
		start: _,
		end: _,
	}) = tokens.peek()
	{
		if let Some(op) = tokens.next() {
			let right = evaluate_multiplicative(tokens, env)?;
			match op.value {
				TokenValue::AddOperator(AddOperator::Add) => left += right,
				TokenValue::AddOperator(AddOperator::Sub) => left -= right,
				_ => return unexpected_token(op),
			}
		} else {
			break;
		}
	}

	Ok(left)
}

fn evaluate_multiplicative(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	env: &mut Environment,
) -> Result<f32, Error> {
	let mut left = evaluate_atomic(tokens, env)?;

	while let Some(Token {
		value: TokenValue::MulOperator(_),
		start: _,
		end: _,
	}) = tokens.peek()
	{
		if let Some(op) = tokens.next() {
			let right = evaluate_multiplicative(tokens, env)?;
			match op.value {
				TokenValue::MulOperator(MulOperator::Mul) => left *= right,
				TokenValue::MulOperator(MulOperator::Div) => {
					if right == 0.0 {
						return Err(Error::Fatal("Division by 0!".to_owned()));
					}
					left /= right
				}
				TokenValue::MulOperator(MulOperator::Mod) => {
					if right == 0.0 {
						return Err(Error::Fatal("Division by 0!".to_owned()));
					}
					left %= right
				}
				_ => return unexpected_token(op),
			}
		} else {
			break;
		}
	}

	Ok(left)
}

fn evaluate_atomic(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	env: &mut Environment,
) -> Result<f32, Error> {
	if let Some(token) = tokens.next() {
		match token.value {
			TokenValue::Number(val) => Ok(val),
			TokenValue::Identifier(id) => match env.get(id.to_owned()) {
				Some(var) => match var {
					Variable::Var(var) => Ok(*var),
					Variable::Fn(var) => Ok(var(evaluate_atomic(tokens, env)?)),
				},
				_ => Err(Error::VariableNotFound(id, token.start, token.end)),
			},
			TokenValue::LastResult => match env.get_last_result() {
				Some(var) => Ok(var),
				_ => Err(Error::VariableNotFound(
					token.value.to_string(),
					token.start,
					token.end,
				)),
			},
			TokenValue::AddOperator(op) => match op {
				AddOperator::Add => evaluate_atomic(tokens, env),
				AddOperator::Sub => Ok(-(evaluate_atomic(tokens, env)?)),
			},
			TokenValue::OpenBracket => {
				let value = evaluate_additive(tokens, env);
				consume(tokens, TokenValue::CloseBracket)?;
				value
			}
			TokenValue::Eof => unexpected_end_of_input(),
			_ => unexpected_token(token),
		}
	} else {
		unexpected_end_of_input()
	}
}

fn consume(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	token_type: TokenValue,
) -> Result<Token, Error> {
	if let Some(token) = tokens.next() {
		if token.value == token_type {
			Ok(token)
		} else {
			Err(Error::UnexpectedToken(
				token.value.to_string(),
				token.start,
				token.end,
			))
		}
	} else {
		Err(Error::Fatal("Unexpected end of input!".to_owned()))
	}
}

fn unexpected_token(token: Token) -> Result<f32, Error> {
	Err(Error::UnexpectedToken(
		token.value.to_string(),
		token.start,
		token.end,
	))
}

fn unexpected_end_of_input() -> Result<f32, Error> {
	Err(Error::Fatal("Unexpected end of input!".to_owned()))
}

#[cfg(test)]
mod tests {
	use super::*;

	// only needed for testing
	fn new_t(value: TokenValue) -> Token {
		Token::new(value, 0, 0)
	}

	#[test]
	fn test_01_evaluate_atomic_simple() {
		assert_eq!(
			evaluate_atomic(
				&mut vec![new_t(TokenValue::Number(45.56)), new_t(TokenValue::Eof),]
					.into_iter()
					.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			45.56
		);

		assert_eq!(
			evaluate_atomic(
				&mut vec![
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(45.56)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			-45.56
		);

		assert_eq!(
			evaluate_atomic(
				&mut vec![
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(45.56)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			-45.56
		);

		assert_eq!(
			evaluate_atomic(
				&mut vec![
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(45.56)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			45.56
		);
	}

	#[test]
	fn test_02_evaluate_mul() {
		assert_eq!(
			evaluate_multiplicative(
				&mut vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			12.0
		);

		assert_eq!(
			evaluate_multiplicative(
				&mut vec![
					new_t(TokenValue::Number(12.0)),
					new_t(TokenValue::MulOperator(MulOperator::Div)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			3.0
		);

		assert_eq!(
			evaluate_multiplicative(
				&mut vec![
					new_t(TokenValue::Number(12.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mod)),
					new_t(TokenValue::Number(7.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			5.0
		);

		assert_eq!(
			evaluate_multiplicative(
				&mut vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			-12.0
		);
	}

	#[test]
	fn test_03_evaluate_add() {
		assert_eq!(
			evaluate_additive(
				&mut vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			7.0
		);

		assert_eq!(
			evaluate_additive(
				&mut vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			-1.0
		);

		assert_eq!(
			evaluate_additive(
				&mut vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			7.0
		);
	}

	#[test]
	fn test_04_evaluate_operation_order() {
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			19.0
		);

		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			16.0
		);

		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Number(3.0)),
					new_t(TokenValue::MulOperator(MulOperator::Mul)),
					new_t(TokenValue::AddOperator(AddOperator::Sub)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut Environment::new()
			)
			.unwrap(),
			-8.0
		);
	}

	#[test]
	fn test_05_blank_input() {
		match evaluate(
			&mut vec![new_t(TokenValue::Eof)].into_iter().peekable(),
			&mut Environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_06_only_operator() {
		match evaluate(
			&mut vec![
				new_t(TokenValue::AddOperator(AddOperator::Add)),
				new_t(TokenValue::Eof),
			]
			.into_iter()
			.peekable(),
			&mut Environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_07_only_numbers() {
		match evaluate(
			&mut vec![
				new_t(TokenValue::Number(4.0)),
				new_t(TokenValue::Number(5.0)),
				new_t(TokenValue::Eof),
			]
			.into_iter()
			.peekable(),
			&mut Environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_08_wrong_bracket() {
		match evaluate(
			&mut vec![
				new_t(TokenValue::OpenBracket),
				new_t(TokenValue::Number(5.0)),
				new_t(TokenValue::OpenBracket),
				new_t(TokenValue::Eof),
			]
			.into_iter()
			.peekable(),
			&mut Environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_09_division_by_0() {
		match evaluate(
			&mut vec![
				new_t(TokenValue::Number(4.0)),
				new_t(TokenValue::MulOperator(MulOperator::Div)),
				new_t(TokenValue::Number(0.0)),
				new_t(TokenValue::Eof),
			]
			.into_iter()
			.peekable(),
			&mut Environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_10_variable_assigment_get() {
		let mut env = Environment::new();

		// assign
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Let),
					new_t(TokenValue::Identifier("a".to_owned())),
					new_t(TokenValue::Equals),
					new_t(TokenValue::Number(34.5)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut env,
			)
			.unwrap(),
			34.5
		);

		// get
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Identifier("a".to_owned())),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(2.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut env,
			)
			.unwrap(),
			36.5
		);

		// reassign
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Let),
					new_t(TokenValue::Identifier("a".to_owned())),
					new_t(TokenValue::Equals),
					new_t(TokenValue::Number(5.4)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut env,
			)
			.unwrap(),
			5.4
		);

		// get
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Identifier("a".to_owned())),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(2.0)),
					new_t(TokenValue::Eof)
				]
				.into_iter()
				.peekable(),
				&mut env,
			)
			.unwrap(),
			7.4
		);
	}

	#[test]
	fn test_11_get_undefined_variable() {
		match evaluate(
			&mut vec![
				new_t(TokenValue::Identifier("xyz".to_owned())),
				new_t(TokenValue::Eof),
			]
			.into_iter()
			.peekable(),
			&mut Environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_12_function_call() {
		let mut env = Environment::new();
		env.init();
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Identifier("sqrt".to_owned())),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof)
				]
				.into_iter()
				.peekable(),
				&mut env,
			)
			.unwrap(),
			2.0
		);

		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenValue::Identifier("sqrt".to_owned())),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::AddOperator(AddOperator::Add)),
					new_t(TokenValue::Number(4.0)),
					new_t(TokenValue::Eof),
				]
				.into_iter()
				.peekable(),
				&mut env,
			)
			.unwrap(),
			6.0
		);
	}

	#[test]
	fn test_13_last_result() {
		let mut env = Environment::new();
		env.init();

		// not assigned yet
		match evaluate(
			&mut vec![new_t(TokenValue::LastResult), new_t(TokenValue::Eof)]
				.into_iter()
				.peekable(),
			&mut Environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}

		// assign last result
		assert_eq!(
			evaluate(
				&mut vec![new_t(TokenValue::Number(4.0)), new_t(TokenValue::Eof),]
					.into_iter()
					.peekable(),
				&mut env,
			)
			.unwrap(),
			4.0
		);

		// use last result
		assert_eq!(
			evaluate(
				&mut vec![new_t(TokenValue::LastResult), new_t(TokenValue::Eof),]
					.into_iter()
					.peekable(),
				&mut env,
			)
			.unwrap(),
			4.0
		);
	}
}
