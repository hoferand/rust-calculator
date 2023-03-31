mod lexer;
use lexer::{Token, TokenType};

pub fn calculate(input: String) -> Result<f32, String> {
	let mut tokens = lexer::tokenize(input)?;
	return evaluate(&mut tokens);
}

fn evaluate(tokens: &mut Vec<Token>) -> Result<f32, String> {
	let result = evaluate_additive(tokens)?;

	let token = tokens.remove(0);
	if !matches!(token.token_type, TokenType::EOF) {
		return unexpected_token(token.value);
	}

	return Ok(result);
}

fn evaluate_additive(tokens: &mut Vec<Token>) -> Result<f32, String> {
	let mut left = evaluate_multiplicative(tokens)?;

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::AddOperator)
	{
		let operator = tokens.remove(0); // remove: +, -
		let right = evaluate_multiplicative(tokens)?;
		match operator.value.as_str() {
			"+" => left += right,
			"-" => left -= right,
			_ => return invalid_operator(operator.value), // should never happen
		}
	}

	return Ok(left);
}

fn evaluate_multiplicative(tokens: &mut Vec<Token>) -> Result<f32, String> {
	let mut left = evaluate_atomic(tokens)?;

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::MulOperator)
	{
		let operator = tokens.remove(0); // remove: *, /, %
		let right = evaluate_atomic(tokens)?;
		match operator.value.as_str() {
			"*" => left *= right,
			"/" => left /= right,
			"%" => left %= right,
			_ => return invalid_operator(operator.value), // should never happen
		}
	}

	return Ok(left);
}

fn evaluate_atomic(tokens: &mut Vec<Token>) -> Result<f32, String> {
	let token = tokens.remove(0);
	match token.token_type {
		TokenType::Number => Ok(token.value.parse().unwrap()),
		TokenType::AddOperator => {
			let operator = token;
			match operator.value.as_str() {
				"+" => evaluate_atomic(tokens),
				"-" => Ok(-(evaluate_atomic(tokens)?)),
				_ => invalid_operator(operator.value), // should never happen
			}
		}
		TokenType::OpenBracket => {
			let value = evaluate_additive(tokens);
			let bracket = tokens.remove(0); // remove )
			if !matches!(bracket.token_type, TokenType::CloseBracket) {
				return unexpected_token(bracket.value);
			}
			value
		}
		_ => unexpected_token(token.value),
	}
}

fn invalid_operator(value: String) -> Result<f32, String> {
	Err(format!("Invalid operator found [{}]!", value))
}

fn unexpected_token(value: String) -> Result<f32, String> {
	Err(format!("Unexpected token found [{}]!", value))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_evaluate_atomic_simple() {
		assert_eq!(
			evaluate_atomic(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("45.56")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			45.56
		);

		assert_eq!(
			evaluate_atomic(&mut vec![
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("45.56")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			-45.56
		);

		assert_eq!(
			evaluate_atomic(&mut vec![
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("+")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("-45.56")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			-45.56
		);

		assert_eq!(
			evaluate_atomic(&mut vec![
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("45.56")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			45.56
		);
	}

	#[test]
	fn test_02_evaluate_mul() {
		assert_eq!(
			evaluate_multiplicative(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("3")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("*")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			12.0
		);

		assert_eq!(
			evaluate_multiplicative(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("12")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("/")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			3.0
		);

		assert_eq!(
			evaluate_multiplicative(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("12")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("%")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("7")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			5.0
		);

		assert_eq!(
			evaluate_multiplicative(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("3")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("*")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			-12.0
		);
	}

	#[test]
	fn test_03_evaluate_add() {
		assert_eq!(
			evaluate_additive(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("3")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("+")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			7.0
		);

		assert_eq!(
			evaluate_additive(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("3")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			-1.0
		);

		assert_eq!(
			evaluate_additive(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("3")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			7.0
		);
	}

	#[test]
	fn test_04_evaluate_operation_order() {
		assert_eq!(
			evaluate(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("3")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("+")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("*")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			19.0
		);

		assert_eq!(
			evaluate(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("3")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("*")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("+")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			16.0
		);

		assert_eq!(
			evaluate(&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("3")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("*")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("+")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			])
			.unwrap(),
			-8.0
		);
	}

	#[test]
	fn test_05_error() {
		match evaluate(&mut vec![Token {
			token_type: TokenType::EOF,
			value: String::from("EOF"),
		}]) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_06_error() {
		match evaluate(&mut vec![
			Token {
				token_type: TokenType::MulOperator,
				value: String::from("+"),
			},
			Token {
				token_type: TokenType::EOF,
				value: String::from("EOF"),
			},
		]) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_07_error() {
		match evaluate(&mut vec![
			Token {
				token_type: TokenType::Number,
				value: String::from("4"),
			},
			Token {
				token_type: TokenType::Number,
				value: String::from("5"),
			},
			Token {
				token_type: TokenType::EOF,
				value: String::from("EOF"),
			},
		]) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_08_error() {
		match evaluate(&mut vec![
			Token {
				token_type: TokenType::OpenBracket,
				value: String::from("("),
			},
			Token {
				token_type: TokenType::Number,
				value: String::from("5"),
			},
			Token {
				token_type: TokenType::OpenBracket,
				value: String::from("("),
			},
			Token {
				token_type: TokenType::EOF,
				value: String::from("EOF"),
			},
		]) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}
}
