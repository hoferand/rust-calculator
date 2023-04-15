#[derive(Debug, PartialEq)]
pub enum TokenType {
	Number,
	AddOperator,
	MulOperator,
	OpenBracket,
	CloseBracket,
	Identifier,
	Let,
	Equals,
	LastResult,
	Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
	pub token_type: TokenType,
	pub value: String,
	pub start: usize,
	pub end: usize,
}
