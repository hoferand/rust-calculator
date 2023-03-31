use std::io::{self, stdout, BufRead, Write};
mod calculator;
use calculator::environment;

fn main() {
	println!("Simple calculator in Rust");
	print!("> ");
	stdout().flush().expect("");

	// initialize environment
	let mut env = environment::new();
	env.init();

	// read expressions
	for input in io::stdin().lock().lines() {
		if let Ok(input) = input {
			if input.is_empty() {
				break;
			}

			// evaluate line
			match calculator::calculate(input, &mut env) {
				Ok(result) => print!("= {}\n\n", result),
				Err(e) => print!("Error: {}\n\n", e),
			}
		}

		print!("> ");
		stdout().flush().expect("");
	}
}
