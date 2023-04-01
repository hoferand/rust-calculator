use crate::calculator::error::Error;
use crate::calculator::token::{Token, TokenType};

pub fn tokenize(input: &String) -> Result<Vec<Token>, Error> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars: Vec<char> = input.chars().collect();
	let mut start = 0;
	while !chars.is_empty() {
		let char = chars.first().unwrap();
		if [' ', '\n', '\t'].contains(&char) {
			// ignore spaces, new lines and tabs
			chars.remove(0);
			start += 1;
			continue;
		} else if char == &'(' {
			tokens.push(Token {
				token_type: TokenType::OpenBracket,
				value: String::from(chars.remove(0)),
				start: start,
				end: start,
			});
		} else if char == &')' {
			tokens.push(Token {
				token_type: TokenType::CloseBracket,
				value: String::from(chars.remove(0)),
				start: start,
				end: start,
			});
		} else if ['+', '-'].contains(&char) {
			tokens.push(Token {
				token_type: TokenType::AddOperator,
				value: String::from(chars.remove(0)),
				start: start,
				end: start,
			});
		} else if ['*', '/', '%'].contains(&char) {
			tokens.push(Token {
				token_type: TokenType::MulOperator,
				value: String::from(chars.remove(0)),
				start: start,
				end: start,
			});
		} else if char == &'=' {
			tokens.push(Token {
				token_type: TokenType::Equals,
				value: String::from(chars.remove(0)),
				start: start,
				end: start,
			});
		} else if char == &'$' {
			tokens.push(Token {
				token_type: TokenType::LastResult,
				value: String::from(chars.remove(0)),
				start: start,
				end: start,
			});
		} else if char.is_ascii_digit() {
			let mut value = String::from(chars.remove(0));
			let mut point_cnt = 0;
			let mut end = start;
			while !chars.is_empty()
				&& (chars.first().unwrap().is_numeric()
					|| (chars.first().unwrap() == &'.' && point_cnt == 0))
			{
				if chars.first().unwrap() == &'.' {
					point_cnt += 1;
				}
				value.push(chars.remove(0));
				end += 1;
			}
			tokens.push(Token {
				token_type: TokenType::Number,
				value: value,
				start: start,
				end: end,
			});
			start = end;
		} else if char.is_ascii_alphanumeric() {
			let mut value = String::from(chars.remove(0));
			let mut end = start;
			while !chars.is_empty() && chars.first().unwrap().is_ascii_alphanumeric() {
				value.push(chars.remove(0));
				end += 1;
			}
			tokens.push(Token {
				token_type: TokenType::Identifier,
				value: value,
				start: start,
				end: end,
			});
			start = end;
		} else {
			return Err(Error::InvalidCharacter(*char, start));
		}

		start += 1;
	}
	tokens.push(Token {
		token_type: TokenType::EOF,
		value: String::from("EOF"),
		start: start,
		end: start,
	});

	return Ok(tokens);
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::calculator::token::Token;

	#[test]
	fn test_01_blank_input() {
		assert_eq!(
			tokenize(&String::from("   \n\n   \t\t		")).unwrap(),
			vec![Token {
				token_type: TokenType::EOF,
				value: String::from("EOF"),
				start: 12,
				end: 12
			}]
		);
	}

	#[test]
	fn test_02_numerical_literal() {
		assert_eq!(
			tokenize(&String::from("9 44.4")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("9"),
					start: 0,
					end: 0
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("44.4"),
					start: 2,
					end: 5
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
					start: 6,
					end: 6
				}
			]
		);
	}

	#[test]
	fn test_03_add_operator_literal() {
		assert_eq!(
			tokenize(&String::from("+-")).unwrap(),
			vec![
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("+"),
					start: 0,
					end: 0
				},
				Token {
					token_type: TokenType::AddOperator,
					value: String::from("-"),
					start: 1,
					end: 1
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
					start: 2,
					end: 2
				}
			]
		);
	}

	#[test]
	fn test_04_mul_operator_literal() {
		assert_eq!(
			tokenize(&String::from("*/%")).unwrap(),
			vec![
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("*"),
					start: 0,
					end: 0
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("/"),
					start: 1,
					end: 1
				},
				Token {
					token_type: TokenType::MulOperator,
					value: String::from("%"),
					start: 2,
					end: 2
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
					start: 3,
					end: 3
				}
			]
		);
	}

	#[test]
	fn test_05_bracket_literal() {
		assert_eq!(
			tokenize(&String::from("()")).unwrap(),
			vec![
				Token {
					token_type: TokenType::OpenBracket,
					value: String::from("("),
					start: 0,
					end: 0
				},
				Token {
					token_type: TokenType::CloseBracket,
					value: String::from(")"),
					start: 1,
					end: 1
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
					start: 2,
					end: 2
				}
			]
		);
	}

	#[test]
	fn test_06_equals_character() {
		assert_eq!(
			tokenize(&String::from("= 4")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Equals,
					value: String::from("="),
					start: 0,
					end: 0
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4"),
					start: 2,
					end: 2
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
					start: 3,
					end: 3
				}
			]
		);
	}

	#[test]
	fn test_07_identifier() {
		assert_eq!(
			tokenize(&String::from("Id id123")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Identifier,
					value: String::from("Id"),
					start: 0,
					end: 1
				},
				Token {
					token_type: TokenType::Identifier,
					value: String::from("id123"),
					start: 3,
					end: 7
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
					start: 8,
					end: 8
				}
			]
		);

		assert_eq!(
			tokenize(&String::from("4id")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Number,
					value: String::from("4"),
					start: 0,
					end: 0
				},
				Token {
					token_type: TokenType::Identifier,
					value: String::from("id"),
					start: 1,
					end: 2
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
					start: 3,
					end: 3
				}
			]
		);
	}

	#[test]
	fn test_08_invalid_character() {
		match tokenize(&String::from("<")) {
			Err(_) => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_09_last_result() {
		assert_eq!(
			tokenize(&String::from("a$4")).unwrap(),
			vec![
				Token {
					token_type: TokenType::Identifier,
					value: String::from("a"),
					start: 0,
					end: 0
				},
				Token {
					token_type: TokenType::LastResult,
					value: String::from("$"),
					start: 1,
					end: 1
				},
				Token {
					token_type: TokenType::Number,
					value: String::from("4"),
					start: 2,
					end: 2
				},
				Token {
					token_type: TokenType::EOF,
					value: String::from("EOF"),
					start: 3,
					end: 3
				}
			]
		);
	}
}
