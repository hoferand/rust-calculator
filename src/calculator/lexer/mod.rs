pub mod token;
pub use token::{Token, TokenType};

pub fn tokenize(input: String) -> Result<Vec<Token>, String> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars: Vec<char> = input.chars().collect();
	while !chars.is_empty() {
		let char = chars.first().unwrap();
		if [' ', '\n', '\t'].contains(&char) {
			// ignore spaces, new lines and tabs
			chars.remove(0);
			continue;
		} else if char == &'(' {
			tokens.push(Token {
				token_type: TokenType::OpenBracket,
				value: String::from(chars.remove(0)),
			});
		} else if char == &')' {
			tokens.push(Token {
				token_type: TokenType::CloseBracket,
				value: String::from(chars.remove(0)),
			});
		} else if ['+', '-'].contains(&char) {
			tokens.push(Token {
				token_type: TokenType::AddOperator,
				value: String::from(chars.remove(0)),
			});
		} else if ['*', '/', '%'].contains(&char) {
			tokens.push(Token {
				token_type: TokenType::MulOperator,
				value: String::from(chars.remove(0)),
			});
		} else if char == &'=' {
			tokens.push(Token {
				token_type: TokenType::Equals,
				value: String::from(chars.remove(0)),
			});
		} else if char.is_ascii_digit() {
			let mut value = String::from(chars.remove(0));
			let mut point_cnt = 0;
			while !chars.is_empty()
				&& (chars.first().unwrap().is_numeric()
					|| (chars.first().unwrap() == &'.' && point_cnt == 0))
			{
				if chars.first().unwrap() == &'.' {
					point_cnt += 1;
				}
				value.push(chars.remove(0));
			}
			tokens.push(Token {
				token_type: TokenType::Number,
				value: value,
			});
		} else if char.is_ascii_alphanumeric() {
			let mut value = String::from(chars.remove(0));
			while !chars.is_empty() && chars.first().unwrap().is_ascii_alphanumeric() {
				value.push(chars.remove(0));
			}
			tokens.push(Token {
				token_type: TokenType::Identifier,
				value: value,
			});
		} else {
			return Err(format!("Unexpected character found [{}]!", char));
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
	fn test_06_equals_character() {
		assert_eq!(
			tokenize(String::from("= 4")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Equals,
					value: String::from("=")
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			]
		);
	}

	#[test]
	fn test_07_identifier() {
		assert_eq!(
			tokenize(String::from("Id id123")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Identifier,
					value: String::from("Id")
				},
				Token {
					token_type: TokenType::Identifier,
					value: String::from("id123")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			]
		);

		assert_eq!(
			tokenize(String::from("4id")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("4")
				},
				Token {
					token_type: TokenType::Identifier,
					value: String::from("id")
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF")
				}
			]
		);
	}

	#[test]
	fn test_08_error() {
		match tokenize(String::from("<")) {
			Ok(_) => assert!(false),
			Err(_) => assert!(true),
		}
	}
}
