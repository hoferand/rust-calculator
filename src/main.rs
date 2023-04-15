use std::io::{self, stdout, BufRead, Write};
mod calculator;
use calculator::{environment::Environment, error::Error};
use colored::Colorize;

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
			match calculator::calculate(&input, &mut env) {
				Ok(result) => print!("= {}\n\n", result),
				Err(e) => match e {
					Error::Fatal(msg) => eprint!("Error: {}\n\n", msg),
					Error::InvalidCharacter(ch, pos) => {
						eprintln!("Error: Invalid character `{}` found!", ch);
						print_error_position(&input, &pos, &pos);
					}
					Error::InvalidOperator(op, start, end) => {
						eprintln!("Error: Invalid operator `{}` found !", op);
						print_error_position(&input, &start, &end);
					}
					Error::UnexpectedToken(token, start, end) => {
						eprintln!("Error: Unexpected token `{}` found!", token);
						print_error_position(&input, &start, &end);
					}
					Error::VariableNotFound(var, start, end) => {
						eprintln!("Error: Variable `{}` not found!", var);
						print_error_position(&input, &start, &end);
					}
				},
			}
		}

		print!("> ");
		stdout().flush().expect("");
	}
}

fn print_error_position(input: &String, start: &usize, end: &usize) {
	eprint!(
		" {} {}\n {} {}{}\n\n",
		"|".red().bold(),
		&input,
		"|".red().bold(),
		" ".repeat(*start),
		"^".repeat(end - start + 1).red().bold(),
	);
}
