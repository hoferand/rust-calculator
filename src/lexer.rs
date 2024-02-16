use crate::{AddOperator, Error, ExpOperator, MulOperator, Token, TokenValue};

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars = input.chars().peekable();
	let mut start = 0;
	while let Some(char) = chars.next() {
		let value;
		let mut src = char.to_string();
		match char {
			' ' | '\n' | '\t' | '\r' => {
				// ignore whitespaces
				start += 1;
				continue;
			}
			'(' => value = TokenValue::OpenBracket,
			')' => value = TokenValue::CloseBracket,
			'+' => value = TokenValue::AddOperator(AddOperator::Add),
			'-' => value = TokenValue::AddOperator(AddOperator::Sub),
			'*' => match chars.peek() {
				Some('*') => {
					src.push(chars.next().unwrap());
					value = TokenValue::ExpOperator(ExpOperator::Power);
				}
				_ => value = TokenValue::MulOperator(MulOperator::Mul),
			},
			'/' => match chars.peek() {
				Some('/') => {
					src.push(chars.next().unwrap());
					value = TokenValue::ExpOperator(ExpOperator::Root);
				}
				_ => value = TokenValue::MulOperator(MulOperator::Div),
			},
			'%' => value = TokenValue::MulOperator(MulOperator::Mod),
			'=' => value = TokenValue::Equals,
			'$' => value = TokenValue::LastResult,
			c if c.is_ascii_digit() => {
				let mut point = false;
				while let Some(n_char) = chars.peek() {
					if n_char.is_numeric() || (*n_char == '.' && !point) {
						if let Some(char) = chars.next() {
							if char == '.' {
								point = true;
							}
							src.push(char);
							continue;
						}
					}
					break;
				}
				match src.parse() {
					Ok(number) => value = TokenValue::Number(number),
					Err(_) => return Err(Error::Fatal("Cannot parse number!")), // should never happen
				}
			}
			c if c.is_ascii_alphabetic() || c == '_' => {
				while let Some(n_char) = chars.peek() {
					if n_char.is_ascii_alphanumeric() || *n_char == '_' {
						if let Some(char) = chars.next() {
							src.push(char);
							continue;
						}
					}
					break;
				}
				if src == "let" {
					value = TokenValue::Let;
				} else {
					value = TokenValue::Identifier(src.clone());
				}
			}
			c => return Err(Error::InvalidCharacter(c, start)),
		}

		let len = src.len();
		tokens.push(Token {
			value,
			src,
			start,
			end: start + len - 1,
		});
		start += len;
	}
	tokens.push(Token::new(TokenValue::Eof, "EOF".to_owned(), start, start));

	Ok(tokens)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_blank_input() {
		assert_eq!(
			tokenize("   \n\n \r \t\t		").unwrap(),
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
			Err(_) => (),
			_ => panic!(),
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

	#[test]
	fn test_10_exp_operator_literal() {
		assert_eq!(
			tokenize("**//").unwrap(),
			vec![
				Token::new(
					TokenValue::ExpOperator(ExpOperator::Power),
					"**".to_owned(),
					0,
					1
				),
				Token::new(
					TokenValue::ExpOperator(ExpOperator::Root),
					"//".to_owned(),
					2,
					3
				),
				Token::new(TokenValue::Eof, "EOF".to_owned(), 4, 4)
			]
		);
	}
}
