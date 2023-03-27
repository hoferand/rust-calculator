use std::io::{self, stdout, BufRead, Write};
mod calculator;

fn main() {
	println!("Simple calculator in Rust");
	print!("> ");
	stdout().flush().expect("");

	// read expressions
	for input in io::stdin().lock().lines() {
		if let Ok(input) = input {
			if input.is_empty() {
				break;
			}

			// evaluate line
			print!("= {}\n\n", calculator::calculate(input));
		}

		print!("> ");
		stdout().flush().expect("");
	}
}
