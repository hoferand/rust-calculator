/// This type represents all possible errors that can occur when evaluating an expression.
#[derive(Debug)]
pub enum Error {
	Fatal(/* message: */ &'static str),
	InvalidCharacter(/* character: */ char, /* position: */ usize),
	UnexpectedToken(
		/* token: */ String,
		/* start: */ usize,
		/* end: */ usize,
	),
	Runtime(/* message: */ &'static str),
	VariableNotFound(
		/* variable: */ String,
		/* start: */ usize,
		/* end: */ usize,
	),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Fatal(msg) => write!(f, "{}", msg),
			Self::InvalidCharacter(ch, _) => write!(f, "Invalid character `{}` found!", ch),
			Self::UnexpectedToken(tk, _, _) => {
				write!(f, "Unexpected token `{}` found!", tk)
			}
			Self::Runtime(msg) => write!(f, "{}", msg),
			Self::VariableNotFound(var, _, _) => write!(f, "Variable `{}` not found!", var),
		}
	}
}

impl std::error::Error for Error {}
