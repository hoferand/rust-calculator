#[derive(Debug)]
pub enum Error {
	// error message
	Error(String),

	// invalid character, position
	InvalidCharacter(char, usize),

	// invalid operator, start, end
	InvalidOperator(String, usize, usize),

	// unexpected, start, end
	UnexpectedToken(String, usize, usize),

	// variable name, start, end
	VariableNotFound(String, usize, usize),
}
