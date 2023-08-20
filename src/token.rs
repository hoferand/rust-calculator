use crate::Error;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
	pub value: TokenValue,
	pub src: String,
	pub start: usize,
	pub end: usize,
}

impl Default for Token {
	fn default() -> Self {
		Self {
			value: TokenValue::Eof,
			src: "EOF".to_owned(),
			start: 0,
			end: 0,
		}
	}
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

	pub(crate) fn unexpected(&self) -> Error {
		match self.value {
			TokenValue::Eof => Error::Runtime("Unexpected end of input!"),
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
