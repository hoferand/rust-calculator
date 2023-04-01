#[derive(Debug)]
pub enum Error {
	// error message
	Error(String),

	// character, position
	InvalidCharacter(char, usize),

	// operator, start, end
	InvalidOperator(String, usize, usize),

	// token value, start, end
	UnexpectedToken(String, usize, usize),

	// variable name, start, end
	VariableNotFound(String, usize, usize),
}
