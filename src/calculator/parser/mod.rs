use crate::calculator::environment::{Environment, Variable};
use crate::calculator::error::Error;
use crate::calculator::token::{Token, TokenType};

pub fn evaluate(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, Error> {
	if !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::EOF) {
		return Err(Error::Error(String::from("Input must not be blank!")));
	}

	let result = evaluate_statement(tokens, env);

	// check if all tokens are consumed
	if matches!(result, Ok(_)) {
		let token = tokens.remove(0);
		if !matches!(token.token_type, TokenType::EOF) {
			return unexpected_token(token);
		}
	}

	return result;
}

fn evaluate_statement(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, Error> {
	let result;
	if matches!(tokens.get(0).unwrap().token_type, TokenType::Identifier)
		&& matches!(tokens.get(1).unwrap().token_type, TokenType::Equals)
	{
		result = evaluate_assignment(tokens, env);
	} else {
		result = evaluate_additive(tokens, env);
	}

	return result;
}

fn evaluate_assignment(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, Error> {
	let key = tokens.remove(0).value;
	tokens.remove(0); // remove =
	let value = evaluate_statement(tokens, env)?;
	return Ok(env.assign(key, value));
}

fn evaluate_additive(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, Error> {
	let mut left = evaluate_multiplicative(tokens, env)?;

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::AddOperator)
	{
		let operator = tokens.remove(0); // remove: +, -
		let right = evaluate_multiplicative(tokens, env)?;
		match operator.value.as_str() {
			"+" => left += right,
			"-" => left -= right,
			_ => return invalid_operator(operator), // should never happen
		}
	}

	return Ok(left);
}

fn evaluate_multiplicative(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, Error> {
	let mut left = evaluate_atomic(tokens, env)?;

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::MulOperator)
	{
		let operator = tokens.remove(0); // remove: *, /, %
		let right = evaluate_atomic(tokens, env)?;
		match operator.value.as_str() {
			"*" => left *= right,
			"/" => {
				if right == 0.0 {
					return Err(Error::Error(String::from("Division by 0!")));
				}
				left /= right
			}
			"%" => left %= right,
			_ => return invalid_operator(operator), // should never happen
		}
	}

	return Ok(left);
}

fn evaluate_atomic(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, Error> {
	let token = tokens.remove(0);
	match token.token_type {
		TokenType::Number => Ok(token.value.parse().unwrap()),
		TokenType::Identifier => {
			let key = token;
			match env.get(key.value.parse().unwrap()) {
				Some(var) => match var {
					Variable::Var(var) => Ok(*var),
					Variable::Fn(var) => Ok(var(evaluate_atomic(tokens, env)?)),
				},
				_ => Err(Error::VariableNotFound(key.value, key.start, key.end)),
			}
		}
		TokenType::AddOperator => {
			let operator = token;
			match operator.value.as_str() {
				"+" => evaluate_atomic(tokens, env),
				"-" => Ok(-(evaluate_atomic(tokens, env)?)),
				_ => invalid_operator(operator), // should never happen
			}
		}
		TokenType::OpenBracket => {
			let value = evaluate_additive(tokens, env);
			let bracket = tokens.remove(0); // remove )
			if !matches!(bracket.token_type, TokenType::CloseBracket) {
				return unexpected_token(bracket);
			}
			value
		}
		_ => unexpected_token(token),
	}
}

fn invalid_operator(token: Token) -> Result<f32, Error> {
	Err(Error::InvalidOperator(token.value, token.start, token.end))
}

fn unexpected_token(token: Token) -> Result<f32, Error> {
	Err(Error::UnexpectedToken(token.value, token.start, token.end))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::calculator::environment;

	// only needed for testing
	fn new_t(token_type: TokenType, value: String) -> Token {
		return Token {
			token_type: token_type,
			value: value,
			start: 0,
			end: 0,
		};
	}

	#[test]
	fn test_01_evaluate_atomic_simple() {
		assert_eq!(
			evaluate_atomic(
				&mut vec![
					new_t(TokenType::Number, String::from("45.56")),
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
			)
			.unwrap(),
			45.56
		);

		assert_eq!(
			evaluate_atomic(
				&mut vec![
					new_t(TokenType::AddOperator, String::from("-")),
					new_t(TokenType::Number, String::from("45.56")),
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut environment::new()
			)
			.unwrap(),
			-8.0
		);
	}

	#[test]
	fn test_05_blank_input() {
		match evaluate(
			&mut vec![new_t(TokenType::EOF, String::from("EOF"))],
			&mut environment::new(),
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
				new_t(TokenType::EOF, String::from("EOF")),
			],
			&mut environment::new(),
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
				new_t(TokenType::EOF, String::from("EOF")),
			],
			&mut environment::new(),
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
				new_t(TokenType::EOF, String::from("EOF")),
			],
			&mut environment::new(),
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
				new_t(TokenType::EOF, String::from("EOF")),
			],
			&mut environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_10_variable_assigment_get() {
		let mut env = environment::new();

		// assign
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenType::Identifier, String::from("a")),
					new_t(TokenType::Equals, String::from("=")),
					new_t(TokenType::Number, String::from("34.5")),
					new_t(TokenType::EOF, String::from("EOF")),
				],
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut env,
			)
			.unwrap(),
			36.5
		);

		// reassign
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenType::Identifier, String::from("a")),
					new_t(TokenType::Equals, String::from("=")),
					new_t(TokenType::Number, String::from("5.4")),
					new_t(TokenType::EOF, String::from("EOF")),
				],
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
					new_t(TokenType::EOF, String::from("EOF"))
				],
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
				new_t(TokenType::EOF, String::from("EOF")),
			],
			&mut environment::new(),
		) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_12_function_call() {
		let mut env = environment::new();
		env.init();
		assert_eq!(
			evaluate(
				&mut vec![
					new_t(TokenType::Identifier, String::from("sqrt")),
					new_t(TokenType::Number, String::from("4")),
					new_t(TokenType::EOF, String::from("EOF"))
				],
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
					new_t(TokenType::EOF, String::from("EOF")),
				],
				&mut env,
			)
			.unwrap(),
			6.0
		);
	}
}
