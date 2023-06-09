use std::collections::HashMap;
use std::f32;
use std::f32::consts::{E, PI};

pub enum Variable {
	Var(f32),
	Fn(fn(f32) -> f32),
}

pub struct Environment {
	variables: HashMap<String, Variable>,
	last_result: Option<f32>,
}

impl Environment {
	pub fn new() -> Environment {
		Environment {
			variables: HashMap::new(),
			last_result: None,
		}
	}

	pub fn assign(&mut self, key: String, value: f32) -> f32 {
		self.variables.insert(key, Variable::Var(value));
		value
	}

	pub fn get(&mut self, key: String) -> Option<&Variable> {
		return self.variables.get(&key);
	}

	pub fn get_last_result(&mut self) -> Option<f32> {
		self.last_result
	}

	pub fn set_last_result(&mut self, value: f32) -> f32 {
		self.last_result = Some(value);
		value
	}

	pub fn init(&mut self) {
		self.variables.insert("pi".to_owned(), Variable::Var(PI));
		self.variables.insert("e".to_owned(), Variable::Var(E));

		self.variables
			.insert("sin".to_owned(), Variable::Fn(|x| x.sin()));
		self.variables
			.insert("asin".to_owned(), Variable::Fn(|x| x.asin()));
		self.variables
			.insert("cos".to_owned(), Variable::Fn(|x| x.cos()));
		self.variables
			.insert("acos".to_owned(), Variable::Fn(|x| x.acos()));
		self.variables
			.insert("tan".to_owned(), Variable::Fn(|x| x.tan()));
		self.variables
			.insert("atan".to_owned(), Variable::Fn(|x| x.atan()));
		self.variables
			.insert("r2d".to_owned(), Variable::Fn(|x| (x * 180.0) / PI));
		self.variables
			.insert("d2r".to_owned(), Variable::Fn(|x| (x * PI) / 180.0));

		#[cfg(test)]
		self.variables
			.insert("test".to_owned(), Variable::Fn(|x| x / 2.0));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_assignment() {
		let mut env = Environment::new();
		assert_eq!(env.assign("var1".to_owned(), 34.5), 34.5);
		match env.get("var1".to_owned()) {
			Some(Variable::Var(val)) => assert_eq!(*val, 34.5),
			_ => panic!(),
		}
	}

	#[test]
	fn test_02_get_undefined() {
		let mut env = Environment::new();
		match env.get("xyz".to_owned()) {
			None => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_03_init() {
		let mut env = Environment::new();
		env.init();
		match env.get("pi".to_owned()) {
			Some(Variable::Var(val)) => assert_eq!(*val, PI),
			_ => panic!(),
		}

		match env.get("test".to_owned()) {
			Some(Variable::Fn(var)) => assert_eq!(var(4.0), 2.0),
			_ => panic!(),
		}
	}
}
