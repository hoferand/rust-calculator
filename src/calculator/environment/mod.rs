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
		return Environment {
			variables: HashMap::new(),
			last_result: None,
		};
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
		self.variables.insert(String::from("pi"), Variable::Var(PI));
		self.variables.insert(String::from("e"), Variable::Var(E));

		self.variables
			.insert(String::from("sqrt"), Variable::Fn(|x| x.sqrt().round()));
		self.variables
			.insert(String::from("sin"), Variable::Fn(|x| x.sin()));
		self.variables
			.insert(String::from("asin"), Variable::Fn(|x| x.asin()));
		self.variables
			.insert(String::from("cos"), Variable::Fn(|x| x.cos()));
		self.variables
			.insert(String::from("acos"), Variable::Fn(|x| x.acos()));
		self.variables
			.insert(String::from("tan"), Variable::Fn(|x| x.tan()));
		self.variables
			.insert(String::from("atan"), Variable::Fn(|x| x.atan()));
		self.variables
			.insert(String::from("r2d"), Variable::Fn(|x| (x * 180.0) / PI));
		self.variables
			.insert(String::from("d2r"), Variable::Fn(|x| (x * PI) / 180.0));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_assignment() {
		let mut env = Environment::new();
		assert_eq!(env.assign(String::from("var1"), 34.5), 34.5);
		match env.get(String::from("var1")) {
			Some(val) => match val {
				Variable::Var(val) => assert_eq!(*val, 34.5),
				_ => assert!(false),
			},
			_ => assert!(false),
		}
	}

	#[test]
	fn test_02_get_undefined() {
		let mut env = Environment::new();
		match env.get(String::from("xyz")) {
			None => assert!(true),
			_ => assert!(false),
		}
	}

	#[test]
	fn test_03_init() {
		let mut env = Environment::new();
		env.init();
		match env.get(String::from("pi")) {
			Some(val) => match val {
				Variable::Var(val) => assert_eq!(*val, PI),
				_ => assert!(false),
			},
			_ => assert!(false),
		}

		match env.get(String::from("sqrt")) {
			Some(var) => match var {
				Variable::Fn(var) => assert_eq!(var(4.0), 2.0),
				_ => assert!(false),
			},
			_ => assert!(false),
		}
	}
}
