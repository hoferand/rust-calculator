use crate::{AddOperator, Error, ExpOperator, MulOperator, Token, TokenValue};

pub(crate) struct Cursor {
	tokens: Vec<Token>,
	pointer: usize,
}

impl Cursor {
	pub(crate) fn new(tokens: Vec<Token>) -> Cursor {
		Cursor { tokens, pointer: 0 }
	}

	pub(crate) fn current(&self) -> Option<Token> {
		self.tokens.get(self.pointer).cloned()
	}

	pub(crate) fn consume(&mut self) -> Option<Token> {
		if let Some(token) = self.tokens.get(self.pointer) {
			self.pointer += 1;
			Some(token.clone())
		} else {
			None
		}
	}

	pub(crate) fn next(&self) -> Option<Token> {
		self.tokens.get(self.pointer + 1).cloned()
	}

	pub(crate) fn expect(&mut self, expected: &TokenValue) -> Result<Token, Error> {
		let token = self.consume().ok_or(Error::UnexpectedEndOfInput)?;
		match &token.value {
			value if value == expected => Ok(token),
			TokenValue::Eof => Err(Error::UnexpectedEndOfInput),
			_ => Err(Error::UnexpectedToken {
				token: token.src,
				start: token.start,
				end: token.end,
			}),
		}
	}

	pub(crate) fn get_add_op(&mut self) -> Result<Option<AddOperator>, Error> {
		if let TokenValue::AddOperator(op) =
			self.consume().ok_or(Error::UnexpectedEndOfInput)?.value
		{
			Ok(Some(op))
		} else {
			self.pointer -= 1;
			Ok(None)
		}
	}

	pub(crate) fn get_mul_op(&mut self) -> Result<Option<MulOperator>, Error> {
		if let TokenValue::MulOperator(op) =
			self.consume().ok_or(Error::UnexpectedEndOfInput)?.value
		{
			Ok(Some(op))
		} else {
			self.pointer -= 1;
			Ok(None)
		}
	}

	pub(crate) fn get_exp_op(&mut self) -> Result<Option<ExpOperator>, Error> {
		if let TokenValue::ExpOperator(op) =
			self.consume().ok_or(Error::UnexpectedEndOfInput)?.value
		{
			Ok(Some(op))
		} else {
			self.pointer -= 1;
			Ok(None)
		}
	}
}
