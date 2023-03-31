mod lexer;
use lexer::{Token, TokenType};
pub mod environment;
use environment::Environment;

pub fn calculate(input: String, env: &mut Environment) -> Result<f32, String> {
	let mut tokens = lexer::tokenize(input)?;
	return evaluate(&mut tokens, env);
}

fn evaluate(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, String> {
	let result = evaluate_statement(tokens, env);

	// check if all tokens are consumed
	if matches!(result, Ok(_)) {
		let token = tokens.remove(0);
		if !matches!(token.token_type, TokenType::EOF) {
			return unexpected_token(token.value);
		}
	}

	return result;
}

fn evaluate_statement(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, String> {
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

fn evaluate_assignment(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, String> {
	let key = tokens.remove(0).value;
	tokens.remove(0); // remove =
	let value = evaluate(tokens, env)?;
	return env.assign(key, value);
}

fn evaluate_additive(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, String> {
	let mut left = evaluate_multiplicative(tokens, env)?;

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::AddOperator)
	{
		let operator = tokens.remove(0); // remove: +, -
		let right = evaluate_multiplicative(tokens, env)?;
		match operator.value.as_str() {
			"+" => left += right,
			"-" => left -= right,
			_ => return invalid_operator(operator.value), // should never happen
		}
	}

	return Ok(left);
}

fn evaluate_multiplicative(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, String> {
	let mut left = evaluate_atomic(tokens, env)?;

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::MulOperator)
	{
		let operator = tokens.remove(0); // remove: *, /, %
		let right = evaluate_atomic(tokens, env)?;
		match operator.value.as_str() {
			"*" => left *= right,
			"/" => {
				if right == 0.0 {
					return Err(String::from("Division by 0!"));
				}
				left /= right
			}
			"%" => left %= right,
			_ => return invalid_operator(operator.value), // should never happen
		}
	}

	return Ok(left);
}

fn evaluate_atomic(tokens: &mut Vec<Token>, env: &mut Environment) -> Result<f32, String> {
	let token = tokens.remove(0);
	match token.token_type {
		TokenType::Number => Ok(token.value.parse().unwrap()),
		TokenType::Identifier => Ok(env.get(token.value.parse().unwrap())?),
		TokenType::AddOperator => {
			let operator = token;
			match operator.value.as_str() {
				"+" => evaluate_atomic(tokens, env),
				"-" => Ok(-(evaluate_atomic(tokens, env)?)),
				_ => invalid_operator(operator.value), // should never happen
			}
		}
		TokenType::OpenBracket => {
			let value = evaluate_additive(tokens, env);
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
			evaluate_atomic(
				&mut vec![
					Token {
						token_type: TokenType::Number,
						value: String::from("45.56")
					},
					Token {
						token_type: TokenType::EOF,
						value: String::from("EOF")
					}
				],
				&mut environment::new()
			)
			.unwrap(),
			45.56
		);

		assert_eq!(
			evaluate_atomic(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			-45.56
		);

		assert_eq!(
			evaluate_atomic(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			-45.56
		);

		assert_eq!(
			evaluate_atomic(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			12.0
		);

		assert_eq!(
			evaluate_multiplicative(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			3.0
		);

		assert_eq!(
			evaluate_multiplicative(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			5.0
		);

		assert_eq!(
			evaluate_multiplicative(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			7.0
		);

		assert_eq!(
			evaluate_additive(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			-1.0
		);

		assert_eq!(
			evaluate_additive(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			19.0
		);

		assert_eq!(
			evaluate(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			16.0
		);

		assert_eq!(
			evaluate(
				&mut vec![
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
				],
				&mut environment::new()
			)
			.unwrap(),
			-8.0
		);
	}

	#[test]
	fn test_05_error() {
		match evaluate(
			&mut vec![Token {
				token_type: TokenType::EOF,
				value: String::from("EOF"),
			}],
			&mut environment::new(),
		) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_06_error() {
		match evaluate(
			&mut vec![
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("+"),
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
				},
			],
			&mut environment::new(),
		) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_07_error() {
		match evaluate(
			&mut vec![
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
			],
			&mut environment::new(),
		) {
			Ok(a) => print!("HALLO {a}"),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_08_error() {
		match evaluate(
			&mut vec![
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
			],
			&mut environment::new(),
		) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_09_error() {
		match evaluate(
			&mut vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("4"),
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("/"),
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("0"),
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
				},
			],
			&mut environment::new(),
		) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}
}
