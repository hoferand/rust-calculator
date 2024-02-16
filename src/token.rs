#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
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
pub enum AddOperator {
	Add,
	Sub,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MulOperator {
	Mul,
	Div,
	Mod,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpOperator {
	Power,
	Root,
}
