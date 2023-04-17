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
			tokens.push(Token::new(TokenValue::OpenBracket, start, start));
		} else if char == ')' {
			tokens.push(Token::new(TokenValue::CloseBracket, start, start));
		} else if char == '+' {
			tokens.push(Token::new(
				TokenValue::AddOperator(AddOperator::Add),
				start,
				start,
			));
		} else if char == '-' {
			tokens.push(Token::new(
				TokenValue::AddOperator(AddOperator::Sub),
				start,
				start,
			));
		} else if char == '*' {
			tokens.push(Token::new(
				TokenValue::MulOperator(MulOperator::Mul),
				start,
				start,
			));
		} else if char == '/' {
			tokens.push(Token::new(
				TokenValue::MulOperator(MulOperator::Div),
				start,
				start,
			));
		} else if char == '%' {
			tokens.push(Token::new(
				TokenValue::MulOperator(MulOperator::Mod),
				start,
				start,
			));
		} else if char == '=' {
			tokens.push(Token::new(TokenValue::Equals, start, start));
		} else if char == '$' {
			tokens.push(Token::new(TokenValue::LastResult, start, start));
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
				Ok(number) => tokens.push(Token::new(TokenValue::Number(number), start, end)),
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
			if value == "let" {
				tokens.push(Token::new(TokenValue::Let, start, end))
			} else {
				tokens.push(Token::new(TokenValue::Identifier(value), start, end))
			}
			start = end;
		} else {
			return Err(Error::InvalidCharacter(char, start));
		}

		start += 1;
	}
	tokens.push(Token::new(TokenValue::Eof, start, start));

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
			vec![Token::new(TokenValue::Eof, 12, 12)]
		);
	}

	#[test]
	fn test_02_numerical_literal() {
		assert_eq!(
			tokenize("9 44.4").unwrap().collect::<Vec<Token>>(),
			vec![
				Token::new(TokenValue::Number(9.0), 0, 0),
				Token::new(TokenValue::Number(44.4), 2, 5),
				Token::new(TokenValue::Eof, 6, 6)
			]
		);
	}

	#[test]
	fn test_03_add_operator_literal() {
		assert_eq!(
			tokenize("+-").unwrap().collect::<Vec<Token>>(),
			vec![
				Token::new(TokenValue::AddOperator(AddOperator::Add), 0, 0),
				Token::new(TokenValue::AddOperator(AddOperator::Sub), 1, 1),
				Token::new(TokenValue::Eof, 2, 2)
			]
		);
	}

	#[test]
	fn test_04_mul_operator_literal() {
		assert_eq!(
			tokenize("*/%").unwrap().collect::<Vec<Token>>(),
			vec![
				Token::new(TokenValue::MulOperator(MulOperator::Mul), 0, 0),
				Token::new(TokenValue::MulOperator(MulOperator::Div), 1, 1),
				Token::new(TokenValue::MulOperator(MulOperator::Mod), 2, 2),
				Token::new(TokenValue::Eof, 3, 3)
			]
		);
	}

	#[test]
	fn test_05_bracket_literal() {
		assert_eq!(
			tokenize("()").unwrap().collect::<Vec<Token>>(),
			vec![
				Token::new(TokenValue::OpenBracket, 0, 0),
				Token::new(TokenValue::CloseBracket, 1, 1),
				Token::new(TokenValue::Eof, 2, 2)
			]
		);
	}

	#[test]
	fn test_06_equals_character() {
		assert_eq!(
			tokenize("= 4").unwrap().collect::<Vec<Token>>(),
			vec![
				Token::new(TokenValue::Equals, 0, 0),
				Token::new(TokenValue::Number(4.0), 2, 2),
				Token::new(TokenValue::Eof, 3, 3)
			]
		);
	}

	#[test]
	fn test_07_identifier() {
		assert_eq!(
			tokenize("Id id123").unwrap().collect::<Vec<Token>>(),
			vec![
				Token::new(TokenValue::Identifier("Id".to_owned()), 0, 1),
				Token::new(TokenValue::Identifier("id123".to_owned()), 3, 7),
				Token::new(TokenValue::Eof, 8, 8)
			]
		);

		assert_eq!(
			tokenize("4id").unwrap().collect::<Vec<Token>>(),
			vec![
				Token::new(TokenValue::Number(4.0), 0, 0),
				Token::new(TokenValue::Identifier("id".to_owned()), 1, 2),
				Token::new(TokenValue::Eof, 3, 3)
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
				Token::new(TokenValue::Identifier("a".to_owned()), 0, 0),
				Token::new(TokenValue::LastResult, 1, 1),
				Token::new(TokenValue::Number(4.0), 2, 2),
				Token::new(TokenValue::Eof, 3, 3)
			]
		);
	}

	#[test]
	fn test_10_let() {
		assert_eq!(
			tokenize("let a").unwrap().collect::<Vec<Token>>(),
			vec![
				Token::new(TokenValue::Let, 0, 2),
				Token::new(TokenValue::Identifier("a".to_owned()), 4, 4),
				Token::new(TokenValue::Eof, 5, 5)
			]
		);
	}
}
