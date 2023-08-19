use super::Error;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
	pub value: TokenValue,
	pub src: String,
	pub start: usize,
	pub end: usize,
}

impl Token {
	pub(crate) fn new(value: TokenValue, src: String, start: usize, end: usize) -> Token {
		Token {
			value,
			src,
			start,
			end,
		}
	}

	pub(crate) fn is_add_op(&self) -> bool {
		matches!(self.value, TokenValue::AddOperator(_))
	}

	pub(crate) fn is_mul_op(&self) -> bool {
		matches!(self.value, TokenValue::MulOperator(_))
	}

	pub(crate) fn is_exp_op(&self) -> bool {
		matches!(self.value, TokenValue::ExpOperator(_))
	}

	pub(crate) fn unexpected(&self) -> Error {
		match self.value {
			TokenValue::Eof => Error::Fatal("Unexpected end of input!".to_owned()),
			_ => Error::UnexpectedToken(self.src.clone(), self.start, self.end),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenValue {
	Number(f32),
	AddOperator(AddOperator),
	MulOperator(MulOperator),
	ExpOperator(ExpOperator),
	OpenBracket,
	CloseBracket,
	Identifier(String),
	Let,
	Equals,
	LastResult,
	Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum AddOperator {
	Add,
	Sub,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum MulOperator {
	Mul,
	Div,
	Mod,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ExpOperator {
	Power,
	Root,
}
