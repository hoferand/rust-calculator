use crate::{AddOperator, Error, ExpOperator, MulOperator, Token, TokenValue};

pub(crate) struct Cursor {
	tokens: Vec<Token>,
	pointer: usize,
}

impl Cursor {
	pub(crate) fn new(tokens: Vec<Token>) -> Cursor {
		Cursor { tokens, pointer: 0 }
	}

	pub(crate) fn current(&mut self) -> Option<Token> {
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

	pub(crate) fn next(&mut self) -> Option<Token> {
		self.tokens.get(self.pointer + 1).cloned()
	}

	pub(crate) fn expect(&mut self, expected: &TokenValue) -> Result<Token, Error> {
		let token = self.consume().unwrap_or_default();
		match &token.value {
			value if value == expected => Ok(token),
			_ => Err(token.unexpected()),
		}
	}

	pub(crate) fn get_add_op(&mut self) -> Option<AddOperator> {
		if let TokenValue::AddOperator(op) = self.consume().unwrap_or_default().value {
			Some(op)
		} else {
			self.pointer -= 1;
			None
		}
	}

	pub(crate) fn get_mul_op(&mut self) -> Option<MulOperator> {
		if let TokenValue::MulOperator(op) = self.consume().unwrap_or_default().value {
			Some(op)
		} else {
			self.pointer -= 1;
			None
		}
	}

	pub(crate) fn get_exp_op(&mut self) -> Option<ExpOperator> {
		if let TokenValue::ExpOperator(op) = self.consume().unwrap_or_default().value {
			Some(op)
		} else {
			self.pointer -= 1;
			None
		}
	}
}
