use crate::calculator::error::Error;
use crate::calculator::token::{AddOperator, MulOperator, Token, TokenValue};

pub fn tokenize(input: &str) -> Result<impl Iterator<Item = Token>, Error> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars = input.chars().peekable();
	let mut start = 0;
	while let Some(char) = chars.next() {
		if [' ', '\n', '\t'].contains(&char) {
			// ignore spaces, line breaks and tabs
			start += 1;
			continue;
		} else if char == '(' {
			tokens.push(Token {
				value: TokenValue::OpenBracket,
				start,
				end: start,
			});
		} else if char == ')' {
			tokens.push(Token {
				value: TokenValue::CloseBracket,
				start,
				end: start,
			});
		} else if char == '+' {
			tokens.push(Token {
				value: TokenValue::AddOperator(AddOperator::Add),
				start,
				end: start,
			});
		} else if char == '-' {
			tokens.push(Token {
				value: TokenValue::AddOperator(AddOperator::Sub),
				start,
				end: start,
			});
		} else if char == '*' {
			tokens.push(Token {
				value: TokenValue::MulOperator(MulOperator::Mul),
				start,
				end: start,
			});
		} else if char == '/' {
			tokens.push(Token {
				value: TokenValue::MulOperator(MulOperator::Div),
				start,
				end: start,
			});
		} else if char == '%' {
			tokens.push(Token {
				value: TokenValue::MulOperator(MulOperator::Mod),
				start,
				end: start,
			});
		} else if char == '=' {
			tokens.push(Token {
				value: TokenValue::Equals,
				start,
				end: start,
			});
		} else if char == '$' {
			tokens.push(Token {
				value: TokenValue::LastResult,
				start,
				end: start,
			});
		} else if char.is_ascii_digit() {
			let mut value = char.to_string();
			let mut point_cnt = 0;
			let mut end = start;
			while let Some(n_char) = chars.peek() {
				if n_char.is_numeric() || (*n_char == '.' && point_cnt == 0) {
					if let Some(char) = chars.next() {
						if char == '.' {
							point_cnt += 1;
						}
						value.push(char);
						end += 1;
						continue;
					}
				}
				break;
			}
			match value.parse() {
				Ok(number) => tokens.push(Token {
					value: TokenValue::Number(number),
					start,
					end,
				}),
				Err(_) => return Err(Error::Fatal("Cannot parse number!".to_owned())),
			}
			start = end;
		} else if char.is_ascii_alphabetic() {
			let mut value = char.to_string();
			let mut end = start;
			while let Some(n_char) = chars.peek() {
				if n_char.is_ascii_alphanumeric() {
					if let Some(char) = chars.next() {
						value.push(char);
						end += 1;
						continue;
					}
				}
				break;
			}
			tokens.push(Token {
				value: if value == "let" {
					TokenValue::Let
				} else {
					TokenValue::Identifier(value)
				},
				start,
				end,
			});
			start = end;
		} else {
			return Err(Error::InvalidCharacter(char, start));
		}

		start += 1;
	}
	tokens.push(Token {
		value: TokenValue::Eof,
		start,
		end: start,
	});

	Ok(tokens.into_iter())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::calculator::token::Token;

	#[test]
	fn test_01_blank_input() {
		assert_eq!(
			tokenize("   \n\n   \t\t		").unwrap().collect::<Vec<Token>>(),
			vec![Token {
				value: TokenValue::Eof,
				start: 12,
				end: 12
			}]
		);
	}

	#[test]
	fn test_02_numerical_literal() {
		assert_eq!(
			tokenize("9 44.4").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::Number(9.0),
					start: 0,
					end: 0
				},
				Token {
					value: TokenValue::Number(44.4),
					start: 2,
					end: 5
				},
				Token {
					value: TokenValue::Eof,
					start: 6,
					end: 6
				}
			]
		);
	}

	#[test]
	fn test_03_add_operator_literal() {
		assert_eq!(
			tokenize("+-").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::AddOperator(AddOperator::Add),
					start: 0,
					end: 0
				},
				Token {
					value: TokenValue::AddOperator(AddOperator::Sub),
					start: 1,
					end: 1
				},
				Token {
					value: TokenValue::Eof,
					start: 2,
					end: 2
				}
			]
		);
	}

	#[test]
	fn test_04_mul_operator_literal() {
		assert_eq!(
			tokenize("*/%").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::MulOperator(MulOperator::Mul),
					start: 0,
					end: 0
				},
				Token {
					value: TokenValue::MulOperator(MulOperator::Div),
					start: 1,
					end: 1
				},
				Token {
					value: TokenValue::MulOperator(MulOperator::Mod),
					start: 2,
					end: 2
				},
				Token {
					value: TokenValue::Eof,
					start: 3,
					end: 3
				}
			]
		);
	}

	#[test]
	fn test_05_bracket_literal() {
		assert_eq!(
			tokenize("()").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::OpenBracket,
					start: 0,
					end: 0
				},
				Token {
					value: TokenValue::CloseBracket,
					start: 1,
					end: 1
				},
				Token {
					value: TokenValue::Eof,
					start: 2,
					end: 2
				}
			]
		);
	}

	#[test]
	fn test_06_equals_character() {
		assert_eq!(
			tokenize("= 4").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::Equals,
					start: 0,
					end: 0
				},
				Token {
					value: TokenValue::Number(4.0),
					start: 2,
					end: 2
				},
				Token {
					value: TokenValue::Eof,
					start: 3,
					end: 3
				}
			]
		);
	}

	#[test]
	fn test_07_identifier() {
		assert_eq!(
			tokenize("Id id123").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::Identifier("Id".to_owned()),
					start: 0,
					end: 1
				},
				Token {
					value: TokenValue::Identifier("id123".to_owned()),
					start: 3,
					end: 7
				},
				Token {
					value: TokenValue::Eof,
					start: 8,
					end: 8
				}
			]
		);

		assert_eq!(
			tokenize("4id").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::Number(4.0),
					start: 0,
					end: 0
				},
				Token {
					value: TokenValue::Identifier("id".to_owned()),
					start: 1,
					end: 2
				},
				Token {
					value: TokenValue::Eof,
					start: 3,
					end: 3
				}
			]
		);
	}

	#[test]
	fn test_08_invalid_character() {
		match tokenize("<") {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_09_last_result() {
		assert_eq!(
			tokenize("a$4").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::Identifier("a".to_owned()),
					start: 0,
					end: 0
				},
				Token {
					value: TokenValue::LastResult,
					start: 1,
					end: 1
				},
				Token {
					value: TokenValue::Number(4.0),
					start: 2,
					end: 2
				},
				Token {
					value: TokenValue::Eof,
					start: 3,
					end: 3
				}
			]
		);
	}

	#[test]
	fn test_10_let() {
		assert_eq!(
			tokenize("let a").unwrap().collect::<Vec<Token>>(),
			vec![
				Token {
					value: TokenValue::Let,
					start: 0,
					end: 2
				},
				Token {
					value: TokenValue::Identifier("a".to_owned()),
					start: 4,
					end: 4
				},
				Token {
					value: TokenValue::Eof,
					start: 5,
					end: 5
				}
			]
		);
	}
}
