use crate::calculator::error::Error;
use crate::calculator::token::{AddOperator, MulOperator, Token, TokenValue};

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars = input.chars().peekable();
	let mut start = 0;
	while let Some(char) = chars.next() {
		match char {
			' ' | '\n' | '\t' => (), // ignore spaces, line breaks and tabs
			'(' => tokens.push(Token::new(
				TokenValue::OpenBracket,
				char.to_string(),
				start,
				start,
			)),
			')' => tokens.push(Token::new(
				TokenValue::CloseBracket,
				char.to_string(),
				start,
				start,
			)),
			'+' => tokens.push(Token::new(
				TokenValue::AddOperator(AddOperator::Add),
				char.to_string(),
				start,
				start,
			)),
			'-' => tokens.push(Token::new(
				TokenValue::AddOperator(AddOperator::Sub),
				char.to_string(),
				start,
				start,
			)),
			'*' => tokens.push(Token::new(
				TokenValue::MulOperator(MulOperator::Mul),
				char.to_string(),
				start,
				start,
			)),
			'/' => tokens.push(Token::new(
				TokenValue::MulOperator(MulOperator::Div),
				char.to_string(),
				start,
				start,
			)),
			'%' => tokens.push(Token::new(
				TokenValue::MulOperator(MulOperator::Mod),
				char.to_string(),
				start,
				start,
			)),
			'=' => tokens.push(Token::new(
				TokenValue::Equals,
				char.to_string(),
				start,
				start,
			)),
			'$' => tokens.push(Token::new(
				TokenValue::LastResult,
				char.to_string(),
				start,
				start,
			)),
			c if c.is_ascii_digit() => {
				let mut value = c.to_string();
				let mut point = false;
				let mut end = start;
				while let Some(n_char) = chars.peek() {
					if n_char.is_numeric() || (*n_char == '.' && !point) {
						if let Some(char) = chars.next() {
							if char == '.' {
								point = true;
							}
							value.push(char);
							end += 1;
							continue;
						}
					}
					break;
				}
				match value.parse() {
					Ok(number) => {
						tokens.push(Token::new(TokenValue::Number(number), value, start, end))
					}
					Err(_) => return Err(Error::Fatal("Cannot parse number!".to_owned())),
				}
				start = end;
			}
			c if c.is_ascii_alphabetic() => {
				let mut value = c.to_string();
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
					tokens.push(Token::new(TokenValue::Let, value, start, end))
				} else {
					tokens.push(Token::new(
						TokenValue::Identifier(value.clone()),
						value,
						start,
						end,
					))
				}
				start = end;
			}
			c => return Err(Error::InvalidCharacter(c, start)),
		}

		start += 1;
	}
	tokens.push(Token::new(TokenValue::Eof, "EOF".to_string(), start, start));

	Ok(tokens)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::calculator::token::Token;

	#[test]
	fn test_01_blank_input() {
		assert_eq!(
			tokenize("   \n\n   \t\t		").unwrap(),
			vec![Token::new(TokenValue::Eof, "EOF".to_owned(), 12, 12)]
		);
	}

	#[test]
	fn test_02_numerical_literal() {
		assert_eq!(
			tokenize("9 44.4").unwrap(),
			vec![
				Token::new(TokenValue::Number(9.0), "9".to_owned(), 0, 0),
				Token::new(TokenValue::Number(44.4), "44.4".to_owned(), 2, 5),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 6, 6)
			]
		);
	}

	#[test]
	fn test_03_add_operator_literal() {
		assert_eq!(
			tokenize("+-").unwrap(),
			vec![
				Token::new(
					TokenValue::AddOperator(AddOperator::Add),
					"+".to_owned(),
					0,
					0
				),
				Token::new(
					TokenValue::AddOperator(AddOperator::Sub),
					"-".to_owned(),
					1,
					1
				),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 2, 2)
			]
		);
	}

	#[test]
	fn test_04_mul_operator_literal() {
		assert_eq!(
			tokenize("*/%").unwrap(),
			vec![
				Token::new(
					TokenValue::MulOperator(MulOperator::Mul),
					"*".to_owned(),
					0,
					0
				),
				Token::new(
					TokenValue::MulOperator(MulOperator::Div),
					"/".to_owned(),
					1,
					1
				),
				Token::new(
					TokenValue::MulOperator(MulOperator::Mod),
					"%".to_owned(),
					2,
					2
				),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 3, 3)
			]
		);
	}

	#[test]
	fn test_05_bracket_literal() {
		assert_eq!(
			tokenize("()").unwrap(),
			vec![
				Token::new(TokenValue::OpenBracket, "(".to_owned(), 0, 0),
				Token::new(TokenValue::CloseBracket, ")".to_owned(), 1, 1),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 2, 2)
			]
		);
	}

	#[test]
	fn test_06_equals_character() {
		assert_eq!(
			tokenize("= 4").unwrap(),
			vec![
				Token::new(TokenValue::Equals, "=".to_owned(), 0, 0),
				Token::new(TokenValue::Number(4.0), "4".to_owned(), 2, 2),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 3, 3)
			]
		);
	}

	#[test]
	fn test_07_identifier() {
		assert_eq!(
			tokenize("Id id123").unwrap(),
			vec![
				Token::new(
					TokenValue::Identifier("Id".to_owned()),
					"Id".to_owned(),
					0,
					1
				),
				Token::new(
					TokenValue::Identifier("id123".to_owned()),
					"id123".to_owned(),
					3,
					7
				),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 8, 8)
			]
		);

		assert_eq!(
			tokenize("4id").unwrap(),
			vec![
				Token::new(TokenValue::Number(4.0), "4".to_owned(), 0, 0),
				Token::new(
					TokenValue::Identifier("id".to_owned()),
					"id".to_owned(),
					1,
					2
				),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 3, 3)
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
			tokenize("a$4").unwrap(),
			vec![
				Token::new(TokenValue::Identifier("a".to_owned()), "a".to_owned(), 0, 0),
				Token::new(TokenValue::LastResult, "$".to_owned(), 1, 1),
				Token::new(TokenValue::Number(4.0), "4".to_owned(), 2, 2),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 3, 3)
			]
		);
	}
}
