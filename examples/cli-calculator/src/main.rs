use std::io::{self, stdout, BufRead, Write};

use colored::Colorize;

use calculator::*;

/// A simple demo application for demonstrating the calculator lib.
fn main() {
	println!("Simple calculator in Rust");
	print!("> ");
	stdout().flush().expect("");

	let mut calculator = Calculator::new();

	// initialize std lib
	calculator.init_std();

	// initialize predefined variables
	calculator.add_var("foo", 40.0);

	// initialize predefined functions
	calculator.add_fn("double", double);
	calculator.add_fn("min", min);

	// read expressions
	for input in io::stdin().lock().lines() {
		if let Ok(input) = input {
			if input.is_empty() {
				break;
			}

			// evaluate line
			match calculator.calculate(&input) {
				Ok(result) => println!("= {}", result),
				Err(e) => {
					eprintln!("{}: {}", "ERROR".red(), e);
					match e {
						Error::Fatal(_) | Error::Runtime(_) | Error::UnexpectedEndOfInput => (),
						Error::InvalidCharacter(_, pos) => {
							print_error_position(&input, pos, pos);
						}
						Error::UnexpectedToken {
							token: _,
							start,
							end,
						} => {
							print_error_position(&input, start, end);
						}
						Error::VariableNotFound { var: _, start, end } => {
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
	let indent: String = input[0..start]
		.chars()
		.map(|c| if c.is_ascii_whitespace() { c } else { ' ' }) // needed for tabs, etc. to be printed correctly
		.collect();
	eprintln!(
		" {} {}\n {} {}{}",
		"|".red().bold(),
		&input,
		"|".red().bold(),
		indent,
		"^".repeat(end - start + 1).red().bold(),
	);
}

fn double(arg: f32) -> f32 {
	arg * 2.0
}

fn min(arg1: f32, arg2: f32) -> Result<f32, Error> {
	Ok(arg1.min(arg2))
}
