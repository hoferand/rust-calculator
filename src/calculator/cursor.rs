use super::{Error, Token, TokenValue};

pub struct Cursor {
	tokens: Vec<Token>,
	pointer: usize,
}

impl Cursor {
	pub fn new(tokens: Vec<Token>) -> Cursor {
		Cursor { tokens, pointer: 0 }
	}

	pub fn current(&mut self) -> Token {
		if let Some(token) = self.tokens.get(self.pointer) {
			return (*token).clone();
		}

		Token::new(TokenValue::Eof, "EOF".to_owned(), 0, 0)
	}

	pub fn consume(&mut self) -> Token {
		if let Some(token) = self.tokens.get(self.pointer) {
			self.pointer += 1;
			return (*token).clone();
		}

		Token::new(TokenValue::Eof, "EOF".to_owned(), 0, 0)
	}

	pub fn next(&mut self) -> Token {
		if let Some(token) = self.tokens.get(self.pointer + 1) {
			return (*token).clone();
		}

		Token::new(TokenValue::Eof, "EOF".to_owned(), 0, 0)
	}

	pub fn expect(&mut self, expected: TokenValue) -> Result<Token, Error> {
		let token = self.consume();
		match &token.value {
			value if *value == expected => Ok(token),
			_ => Err(token.unexpected()),
		}
	}
}
