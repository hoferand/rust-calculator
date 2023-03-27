mod lexer;
use lexer::{Token, TokenType};

pub fn calculate(input: String) -> f32 {
	let mut tokens = lexer::tokenize(input);
	return evaluate(&mut tokens);
}

fn evaluate(tokens: &mut Vec<Token>) -> f32 {
	let result = evaluate_additive(tokens);

	if !matches!(tokens.remove(0).token_type, TokenType::EOF) {
		panic!("Unexpected token found!");
	}

	return result;
}

fn evaluate_additive(tokens: &mut Vec<Token>) -> f32 {
	let mut left = evaluate_multiplicative(tokens);

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::AddOperator)
	{
		let operator = tokens.remove(0); // remove: +, -
		let right = evaluate_multiplicative(tokens);
		match operator.value.as_str() {
			"+" => left += right,
			"-" => left -= right,
			_ => panic!("Unexpected operator found!"), // should never happen
		}
	}

	return left;
}

fn evaluate_multiplicative(tokens: &mut Vec<Token>) -> f32 {
	let mut left = evaluate_atomic(tokens);

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::MulOperator)
	{
		let operator = tokens.remove(0); // remove: *, /, %
		let right = evaluate_atomic(tokens);
		match operator.value.as_str() {
			"*" => left *= right,
			"/" => left /= right,
			"%" => left %= right,
			_ => panic!("Unexpected operator found!"), // should never happen
		}
	}

	return left;
}

fn evaluate_atomic(tokens: &mut Vec<Token>) -> f32 {
	let token = tokens.remove(0);
	match token.token_type {
		TokenType::Number => token.value.parse().unwrap(),
		TokenType::AddOperator => {
			match token.value.as_str() {
				"+" => evaluate_atomic(tokens),
				"-" => -evaluate_atomic(tokens),
				_ => panic!("Unexpected operator found!"), // should never happen
			}
		}
		TokenType::OpenBracket => {
			let value = evaluate_additive(tokens);
			let bracket = tokens.remove(0); // remove )
			if !matches!(bracket.token_type, TokenType::CloseBracket) {
				panic!("Unexpected token found!");
			}
			value
		}
		_ => panic!("Unexpected token found!"),
	}
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
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
			]),
			-8.0
		);
	}

	#[test]
	#[should_panic]
	fn test_06_panic() {
		evaluate(&mut vec![Token {
			token_type: TokenType::EOF,
			value: String::from("EOF"),
		}]);
	}

	#[test]
	#[should_panic]
	fn test_07_panic() {
		evaluate(&mut vec![
			Token {
				token_type: TokenType::MulOperator,
				value: String::from("+"),
			},
			Token {
				token_type: TokenType::EOF,
				value: String::from("EOF"),
			},
		]);
	}

	#[test]
	#[should_panic]
	fn test_08_panic() {
		evaluate(&mut vec![
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
		]);
	}

	#[test]
	#[should_panic]
	fn test_09_panic() {
		evaluate(&mut vec![
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
		]);
	}
}
