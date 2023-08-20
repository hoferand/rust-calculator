use std::io::{self, stdout, BufRead, Write};

use colored::Colorize;

use calculator::*;

/// A simple demo application for demonstrating the calculator lib.
fn main() {
	println!("Simple calculator in Rust");
	print!("> ");
	stdout().flush().expect("");

	// initialize environment
	let mut env = Environment::new();
	env.init();

	// read expressions
	for input in io::stdin().lock().lines() {
		if let Ok(input) = input {
			if input.is_empty() {
				break;
			}

			// evaluate line
			match calculate(&input, &mut env) {
				Ok(result) => println!("= {}", result),
				Err(e) => {
					eprintln!("Error: {}", e);
					match e {
						Error::Fatal(_) | Error::Runtime(_) => (),
						Error::InvalidCharacter(_, pos) => {
							print_error_position(&input, pos, pos);
						}
						Error::UnexpectedToken(_, start, end) => {
							print_error_position(&input, start, end);
						}
						Error::VariableNotFound(_, start, end) => {
							print_error_position(&input, start, end);
						}
					}
				}
			}
		}

		print!("\n> ");
		stdout().flush().expect("");
	}
}

fn print_error_position(input: &str, start: usize, end: usize) {
	eprintln!(
		" {} {}\n {} {}{}",
		"|".red().bold(),
		&input,
		"|".red().bold(),
		" ".repeat(start),
		"^".repeat(end - start + 1).red().bold(),
	);
}
