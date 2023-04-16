#[derive(Debug)]
pub enum Error {
	// error message
	Fatal(String),

	// character, position
	InvalidCharacter(char, usize),

	// operator, start, end
	InvalidOperator(String, usize, usize),

	// token value, start, end
	UnexpectedToken(String, usize, usize),

	// variable name, start, end
	VariableNotFound(String, usize, usize),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Fatal(msg) => write!(f, "{}", msg),
			Self::InvalidCharacter(ch, _) => write!(f, "Invalid character `{}` found!", ch),
			Self::InvalidOperator(op, _, _) => write!(f, "Invalid operator `{}` found!", op),
			Self::UnexpectedToken(tk, _, _) => {
				write!(f, "Unexpected token `{}` found!", tk)
			}
			Self::VariableNotFound(var, _, _) => write!(f, "Variable `{}` not found!", var),
		}
	}
}

impl std::error::Error for Error {}
