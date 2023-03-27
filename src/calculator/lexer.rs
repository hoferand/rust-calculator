#[derive(Debug)]
pub enum TokenType {
	Number,
	AddOperator,
	MulOperator,
	OpenBracket,
	CloseBracket,
	EOF,
}

#[derive(Debug)]
pub struct Token {
	pub token_type: TokenType,
	pub value: String,
}

pub fn tokenize(input: String) -> Vec<Token> {
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
