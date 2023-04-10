#[derive(Debug, PartialEq)]
pub enum TokenType {
	Number,
	AddOperator,
	MulOperator,
	OpenBracket,
	CloseBracket,
	Identifier,
	Equals,
	LastResult,
	EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
	pub token_type: TokenType,
	pub value: String,
	pub start: usize,
	pub end: usize,
}