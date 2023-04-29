use crate::calculator::error::Error;
use crate::calculator::token::{Token, TokenValue};

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

	pub fn expect(&mut self, value: TokenValue) -> Result<Token, Error> {
		let token = self.consume();
		match &token.value {
			t_value if value == *t_value => Ok(token),
			TokenValue::Eof => Err(Error::Fatal("Unexpected end of input!".to_owned())),
			_ => Err(Error::UnexpectedToken(token.src, token.start, token.end)),
		}
	}
}
