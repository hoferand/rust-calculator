mod lexer;
use lexer::{Token, TokenType};

pub fn evaluate(input: String) -> f32 {
	return evaluate_additive(&mut lexer::tokenize(input));
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
		TokenType::AddOperator => {
			match token.value.as_str() {
				"+" => evaluate_atomic(tokens),
				"-" => -evaluate_atomic(tokens),
				_ => panic!("Unexpected operator found!"), // should never happen
			}
		}
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
