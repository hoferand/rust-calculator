use std::{
	io::{self, stdout, BufRead, Write},
	panic,
};

#[derive(Debug)]
enum TokenType {
	Number,
	AddOperator,
	MulOperator,
	OpenBracket,
	CloseBracket,
	EOF,
}

#[derive(Debug)]
struct Token {
	token_type: TokenType,
	value: String,
}

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
			let mut tokens = tokenize(input);
			println!("= {}", evaluate_additive(&mut tokens));
		}

		print!("> ");
		stdout().flush().expect("");
	}
}

fn tokenize(input: String) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars: Vec<char> = input.chars().collect();
	while !chars.is_empty() {
		if [' ', '\n', '\t'].contains(&chars.first().unwrap()) {
			// ignore spaces, new lines and tabs
			chars.remove(0);
			continue;
		}

		if chars.first().unwrap() == &'(' {
			tokens.push(Token {
				token_type: TokenType::OpenBracket,
				value: String::from(chars.remove(0)),
			});
		} else if chars.first().unwrap() == &')' {
			tokens.push(Token {
				token_type: TokenType::CloseBracket,
				value: String::from(chars.remove(0)),
			});
		} else if ['+', '-'].contains(&chars.first().unwrap()) {
			tokens.push(Token {
				token_type: TokenType::AddOperator,
				value: String::from(chars.remove(0)),
			});
		} else if ['*', '/', '%'].contains(&chars.first().unwrap()) {
			tokens.push(Token {
				token_type: TokenType::MulOperator,
				value: String::from(chars.remove(0)),
			});
		} else if chars.first().unwrap().is_numeric() {
			let mut value = String::from(chars.remove(0));
			while !chars.is_empty()
				&& (chars.first().unwrap().is_numeric() || chars.first().unwrap() == &'.')
			{
				value.push(chars.remove(0));
			}
			tokens.push(Token {
				token_type: TokenType::Number,
				value: value,
			});
		} else {
			panic!("Unexpected character found: {}!", chars.first().unwrap());
		}
	}
	tokens.push(Token {
		token_type: TokenType::EOF,
		value: String::from("EOF"),
	});

	return tokens;
}

fn evaluate_additive(tokens: &mut Vec<Token>) -> f32 {
	let mut left = evaluate_multiplicative(tokens);

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::AddOperator)
	{
		let operator = tokens.remove(0); // remove: +, -
		let right = evaluate_multiplicative(tokens);
		match operator.value.as_str() {
			"+" => left += right,
			"-" => left -= right,
			_ => panic!("Unexpected operator found!"), // should never happen
		}
	}

	return left;
}

fn evaluate_multiplicative(tokens: &mut Vec<Token>) -> f32 {
	let mut left = evaluate_atomic(tokens);

	while !tokens.is_empty() && matches!(tokens.first().unwrap().token_type, TokenType::MulOperator)
	{
		let operator = tokens.remove(0); // remove: *, /, %
		let right = evaluate_atomic(tokens);
		match operator.value.as_str() {
			"*" => left *= right,
			"/" => left /= right,
			"%" => left %= right,
			_ => panic!("Unexpected operator found!"), // should never happen
		}
	}

	return left;
}

fn evaluate_atomic(tokens: &mut Vec<Token>) -> f32 {
	let token = tokens.remove(0);
	match token.token_type {
		TokenType::Number => token.value.parse().unwrap(),
		TokenType::OpenBracket => {
			let value = evaluate_additive(tokens);
			let bracket = tokens.remove(0); // remove )
			if !matches!(bracket.token_type, TokenType::CloseBracket) {
				panic!("Unexpected token found!");
			}
			value
		}
		_ => panic!("Unexpected token found!"),
	}
}
