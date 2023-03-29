pub mod token;
pub use token::{Token, TokenType};

pub fn tokenize(input: String) -> Result<Vec<Token>, String> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars: Vec<char> = input.chars().collect();
	while !chars.is_empty() {
		if [' ', '\n', '\t'].contains(&chars.first().unwrap()) {
			// ignore spaces, new lines and tabs
			chars.remove(0);
			continue;
		}

		if chars.first().unwrap() == &'(' {
			tokens.push(Token {
				token_type: TokenType::OpenBracket,
				value: String::from(chars.remove(0)),
			});
		} else if chars.first().unwrap() == &')' {
			tokens.push(Token {
				token_type: TokenType::CloseBracket,
				value: String::from(chars.remove(0)),
			});
		} else if ['+', '-'].contains(&chars.first().unwrap()) {
			tokens.push(Token {
				token_type: TokenType::AddOperator,
				value: String::from(chars.remove(0)),
			});
		} else if ['*', '/', '%'].contains(&chars.first().unwrap()) {
			tokens.push(Token {
				token_type: TokenType::MulOperator,
				value: String::from(chars.remove(0)),
			});
		} else if chars.first().unwrap().is_numeric() {
			let mut value = String::from(chars.remove(0));
			while !chars.is_empty()
				&& (chars.first().unwrap().is_numeric() || chars.first().unwrap() == &'.')
			{
				value.push(chars.remove(0));
			}
			tokens.push(Token {
				token_type: TokenType::Number,
				value: value,
			});
		} else {
			return Err(format!(
				"Unexpected character found: {}!",
				chars.first().unwrap()
			));
		}
	}
	tokens.push(Token {
		token_type: TokenType::EOF,
		value: String::from("EOF"),
	});

	return Ok(tokens);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_blank_input() {
		assert_eq!(
			tokenize(String::from("   \n\n   \t\t		")).unwrap(),
			vec![Token {
				token_type: TokenType::EOF,
				value: String::from("EOF")
			}]
		);
	}

	#[test]
	fn test_02_numerical_literal() {
		assert_eq!(
			tokenize(String::from("9 44.4")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("9")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("44.4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			]
		);
	}

	#[test]
	fn test_03_add_operator_literal() {
		assert_eq!(
			tokenize(String::from("+-")).unwrap(),
			vec![
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("+")
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			]
		);
	}

	#[test]
	fn test_04_mul_operator_literal() {
		assert_eq!(
			tokenize(String::from("*/%")).unwrap(),
			vec![
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("*")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("/")
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("%")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			]
		);
	}

	#[test]
	fn test_05_bracket_literal() {
		assert_eq!(
			tokenize(String::from("()")).unwrap(),
			vec![
				Token {
					token_type: TokenType::OpenBracket,
					value: String::from("(")
				},
				Token {
					token_type: TokenType::CloseBracket,
					value: String::from(")")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			]
		);
	}

	#[test]
	fn test_06_error() {
		match tokenize(String::from("a")) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}
}
