pub mod error;
use error::Error;
pub mod environment;
use environment::Environment;
mod lexer;
mod parser;
mod token;

pub fn calculate(input: &String, env: &mut Environment) -> Result<f32, Error> {
	let mut tokens = lexer::tokenize(input)?;
	return parser::evaluate(&mut tokens, env);
}
