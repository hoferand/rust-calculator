#[derive(Debug, PartialEq)]
pub struct Token {
	pub value: TokenValue,
	pub src: String,
	pub start: usize,
	pub end: usize,
}

impl Token {
	pub fn new(value: TokenValue, src: String, start: usize, end: usize) -> Token {
		Token {
			value,
			src,
			start,
			end,
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
	Number(f32),
	AddOperator(AddOperator),
	MulOperator(MulOperator),
	OpenBracket,
	CloseBracket,
	Identifier(String),
	Let,
	Equals,
	LastResult,
	Eof,
}

#[derive(Debug, PartialEq)]
pub enum AddOperator {
	Add,
	Sub,
}

#[derive(Debug, PartialEq)]
pub enum MulOperator {
	Mul,
	Div,
	Mod,
}
