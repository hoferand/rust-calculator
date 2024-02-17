use colored::Colorize;
use rustyline::DefaultEditor;

use calculator::*;

/// A simple demo application for demonstrating the calculator lib.
fn main() {
	println!("Simple calculator in Rust");

	let mut calculator = Calculator::new();

	// initialize std lib
	calculator.init_std();

	// initialize predefined variables
	calculator.add_var("foo", 40.0);

	// initialize predefined functions
	calculator.add_fn("double", |arg: f32| arg * 2.0);
	fn div(a: f32, b: f32) -> Result<f32, Error> {
		if b == 0.0 {
			Err(Error::Fatal("Division by zero!"))
		} else {
			Ok(a / b)
		}
	}
	calculator.add_fn("div", div);

	// read expressions
	let mut rl = DefaultEditor::new().expect("Failed to create new editor!");
	while let Ok(input) = rl.readline("> ") {
		if input.is_empty() {
			break;
		}

		rl.add_history_entry(&input)
			.expect("Failed to add history entry!");

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
					Error::UnexpectedToken { start, end, .. } => {
						print_error_position(&input, start, end);
					}
					Error::VariableNotFound { start, end, .. } => {
						print_error_position(&input, start, end);
					}
				}
			}
		}
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
