use std::iter::Peekable;

use crate::calculator::environment::{Environment, Variable};
use crate::calculator::error::Error;
use crate::calculator::token::{Token, TokenType};

pub fn evaluate(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	env: &mut Environment,
) -> Result<f32, Error> {
	let result = evaluate_statement(tokens, env);

	// check if all tokens are consumed
	if let Ok(value) = result {
		consume(tokens, TokenType::Eof)?;
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
		match token.token_type {
			TokenType::Let => result = evaluate_assignment(tokens, env),
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
	consume(tokens, TokenType::Let)?;
	let id = consume(tokens, TokenType::Identifier)?;
	consume(tokens, TokenType::Equals)?;
	let value = evaluate_statement(tokens, env)?;

	Ok(env.assign(id.value, value))
}

fn evaluate_additive(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	env: &mut Environment,
) -> Result<f32, Error> {
	let mut left = evaluate_multiplicative(tokens, env)?;

	while let Some(token) = tokens.peek() {
		if token.token_type == TokenType::AddOperator {
			let operator = consume(tokens, TokenType::AddOperator)?;
			let right = evaluate_multiplicative(tokens, env)?;
			match operator.value.as_str() {
				"+" => left += right,
				"-" => left -= right,
				_ => return invalid_operator(operator), // should never happen
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

	while let Some(token) = tokens.peek() {
		if token.token_type == TokenType::MulOperator {
			let operator = consume(tokens, TokenType::MulOperator)?;
			let right = evaluate_atomic(tokens, env)?;
			match operator.value.as_str() {
				"*" => left *= right,
				"/" => {
					if right == 0.0 {
						return Err(Error::Fatal(String::from("Division by 0!")));
					}
					left /= right
				}
				"%" => left %= right,
				_ => return invalid_operator(operator), // should never happen
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
		match token.token_type {
			TokenType::Number => Ok(token.value.parse().unwrap()),
			TokenType::Identifier => match env.get(token.value.parse().unwrap()) {
				Some(var) => match var {
					Variable::Var(var) => Ok(*var),
					Variable::Fn(var) => Ok(var(evaluate_atomic(tokens, env)?)),
				},
				_ => Err(Error::VariableNotFound(token.value, token.start, token.end)),
			},
			TokenType::LastResult => match env.get_last_result() {
				Some(var) => Ok(var),
				_ => Err(Error::VariableNotFound(
					String::from("$"),
					token.start,
					token.end,
				)),
			},
			TokenType::AddOperator => match token.value.as_str() {
				"+" => evaluate_atomic(tokens, env),
				"-" => Ok(-(evaluate_atomic(tokens, env)?)),
				_ => invalid_operator(token), // should never happen
			},
			TokenType::OpenBracket => {
				let value = evaluate_additive(tokens, env);
				consume(tokens, TokenType::CloseBracket)?;
				value
			}
			TokenType::Eof => unexpected_end_of_input(),
			_ => unexpected_token(token),
		}
	} else {
		unexpected_end_of_input()
	}
}

fn consume(
	tokens: &mut Peekable<impl Iterator<Item = Token>>,
	token_type: TokenType,
) -> Result<Token, Error> {
	if let Some(token) = tokens.next() {
		if token.token_type == token_type {
			Ok(token)
		} else {
			Err(Error::UnexpectedToken(token.value, token.start, token.end))
		}
	} else {
		Err(Error::Fatal(String::from("Unexpected end of input!")))
	}
}

fn invalid_operator(token: Token) -> Result<f32, Error> {
	Err(Error::InvalidOperator(token.value, token.start, token.end))
}

fn unexpected_token(token: Token) -> Result<f32, Error> {
	Err(Error::UnexpectedToken(token.value, token.start, token.end))
}

fn unexpected_end_of_input() -> Result<f32, Error> {
	Err(Error::Fatal(String::from("Unexpected end of input!")))
}

#[cfg(test)]
mod tests {
	use super::*;

	// only needed for testing
	fn new_t(token_type: TokenType, value: String) -> Token {
		Token {
			token_type,
			value,
			start: 0,
			end: 0,
		}
	}

	#[test]
	fn test_01_evaluate_atomic_simple() {
		assert_eq!(
			evaluate_atomic(
				&mut vec![
					new_t(TokenType::Number, String::from("45.56")),
					new_t(TokenType::Eof, String::from("Eof")),
				]
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
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::Number, String::from("45.56")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::AddOperator, String::from("+")),
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::Number, String::from("45.56")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::Number, String::from("45.56")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("3")),
					new_t(TokenType::MulOperator, String::from("*")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("12")),
					new_t(TokenType::MulOperator, String::from("/")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("12")),
					new_t(TokenType::MulOperator, String::from("%")),
					new_t(TokenType::Number, String::from("7")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("3")),
					new_t(TokenType::MulOperator, String::from("*")),
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("3")),
					new_t(TokenType::AddOperator, String::from("+")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("3")),
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("3")),
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("3")),
					new_t(TokenType::AddOperator, String::from("+")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::MulOperator, String::from("*")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("3")),
					new_t(TokenType::MulOperator, String::from("*")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::AddOperator, String::from("+")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Number, String::from("3")),
					new_t(TokenType::MulOperator, String::from("*")),
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::AddOperator, String::from("+")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
			&mut vec![new_t(TokenType::Eof, String::from("Eof"))]
				.into_iter()
				.peekable(),
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
				new_t(TokenType::AddOperator, String::from("+")),
				new_t(TokenType::Eof, String::from("Eof")),
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
				new_t(TokenType::Number, String::from("4")),
				new_t(TokenType::Number, String::from("5")),
				new_t(TokenType::Eof, String::from("Eof")),
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
				new_t(TokenType::OpenBracket, String::from("(")),
				new_t(TokenType::Number, String::from("5")),
				new_t(TokenType::OpenBracket, String::from("(")),
				new_t(TokenType::Eof, String::from("Eof")),
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
				new_t(TokenType::Number, String::from("4")),
				new_t(TokenType::MulOperator, String::from("/")),
				new_t(TokenType::Number, String::from("0")),
				new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Let, String::from("let")),
					new_t(TokenType::Identifier, String::from("a")),
					new_t(TokenType::Equals, String::from("=")),
					new_t(TokenType::Number, String::from("34.5")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Identifier, String::from("a")),
					new_t(TokenType::AddOperator, String::from("+")),
					new_t(TokenType::Number, String::from("2")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Let, String::from("let")),
					new_t(TokenType::Identifier, String::from("a")),
					new_t(TokenType::Equals, String::from("=")),
					new_t(TokenType::Number, String::from("5.4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Identifier, String::from("a")),
					new_t(TokenType::AddOperator, String::from("+")),
					new_t(TokenType::Number, String::from("2")),
					new_t(TokenType::Eof, String::from("Eof"))
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
				new_t(TokenType::Identifier, String::from("xyz")),
				new_t(TokenType::Eof, String::from("Eof")),
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
					new_t(TokenType::Identifier, String::from("sqrt")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof"))
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
					new_t(TokenType::Identifier, String::from("sqrt")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::AddOperator, String::from("+")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
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
			&mut vec![
				new_t(TokenType::LastResult, String::from("$")),
				new_t(TokenType::Eof, String::from("Eof")),
			]
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
				&mut vec![
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::Eof, String::from("Eof")),
				]
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
				&mut vec![
					new_t(TokenType::LastResult, String::from("$")),
					new_t(TokenType::Eof, String::from("Eof")),
				]
				.into_iter()
				.peekable(),
				&mut env,
			)
			.unwrap(),
			4.0
		);
	}
}
