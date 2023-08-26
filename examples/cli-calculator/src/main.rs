use std::io::{self, stdout, BufRead, Write};

use colored::Colorize;

use calculator::*;

/// A simple demo application for demonstrating the calculator lib.
fn main() {
	println!("Simple calculator in Rust");
	print!("> ");
	stdout().flush().expect("");

	// initialize environment
	let mut calculator = Calculator::new();
	calculator.init_std();
	calculator.add_custom("twice", twice);
	calculator.add_custom("min", min);

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

fn twice(args: &mut dyn Arguments) -> Result<f32, Error> {
	let first_arg = args.get_next_arg()?;
	Ok(first_arg * 2.0)
}

fn min(args: &mut dyn Arguments) -> Result<f32, Error> {
	let fst_arg = args.get_next_arg()?;
	let sec_arg = args.get_next_arg()?;
	Ok(fst_arg.min(sec_arg))
}
