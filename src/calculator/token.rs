#[derive(Debug, PartialEq)]
pub struct Token {
	pub value: TokenValue,
	pub start: usize,
	pub end: usize,
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

impl ToString for TokenValue {
	fn to_string(&self) -> String {
		match self {
			Self::Number(v) => v.to_string(),
			Self::AddOperator(o) => o.to_string(),
			Self::MulOperator(o) => o.to_string(),
			Self::OpenBracket => "(".to_owned(),
			Self::CloseBracket => ")".to_owned(),
			Self::Identifier(i) => i.to_owned(),
			Self::Let => "let".to_owned(),
			Self::Equals => "=".to_owned(),
			Self::LastResult => "$".to_owned(),
			Self::Eof => "EOF".to_owned(),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum AddOperator {
	Add,
	Sub,
}

impl ToString for AddOperator {
	fn to_string(&self) -> String {
		match self {
			Self::Add => "+".to_owned(),
			Self::Sub => "-".to_owned(),
		}
	}
}

#[derive(Debug, PartialEq)]
pub enum MulOperator {
	Mul,
	Div,
	Mod,
}

impl ToString for MulOperator {
	fn to_string(&self) -> String {
		match self {
			Self::Mul => "*".to_owned(),
			Self::Div => "/".to_owned(),
			Self::Mod => "%".to_owned(),
		}
	}
}
