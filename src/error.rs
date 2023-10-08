/// This type represents all possible errors that can occur when evaluating an expression.
#[derive(Debug, PartialEq)]
pub enum Error {
	Fatal(/* message: */ &'static str),
	InvalidCharacter(/* character: */ char, /* position: */ usize),
	UnexpectedToken {
		token: String,
		start: usize,
		end: usize,
	},
	UnexpectedEndOfInput,
	Runtime(/* message: */ &'static str),
	VariableNotFound {
		var: String,
		start: usize,
		end: usize,
	},
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Fatal(msg) => write!(f, "{}", msg),
			Self::InvalidCharacter(ch, _) => write!(f, "Invalid character `{}` found!", ch),
			Self::UnexpectedToken {
				token,
				start: _,
				end: _,
			} => {
				write!(f, "Unexpected token `{}` found!", token)
			}
			Self::Runtime(msg) => write!(f, "{}", msg),
			Self::VariableNotFound {
				var,
				start: _,
				end: _,
			} => write!(f, "Variable `{}` not found!", var),
			Self::UnexpectedEndOfInput => write!(f, "Unexpected end of input!"),
		}
	}
}

impl std::error::Error for Error {}
