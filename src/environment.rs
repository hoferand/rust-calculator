use std::collections::HashMap;
use std::f32::consts::{E, PI};

use crate::{Arguments, Error, Variable};

pub(crate) struct Environment {
	variables: HashMap<String, Variable>,
	last_result: Option<f32>,
}

impl Default for Environment {
	fn default() -> Self {
		Self::new()
	}
}

impl Environment {
	pub(crate) fn new() -> Environment {
		Environment {
			variables: HashMap::new(),
			last_result: None,
		}
	}

	pub(crate) fn assign(&mut self, key: String, value: f32) -> f32 {
		self.variables.insert(key, Variable::Var(value));
		value
	}

	pub(crate) fn assign_custom(
		&mut self,
		key: String,
		fun: fn(&mut dyn Arguments) -> Result<f32, Error>,
	) {
		self.variables.insert(key, Variable::Custom(fun));
	}

	pub(crate) fn get(&self, key: &str) -> Option<&Variable> {
		return self.variables.get(key);
	}

	pub(crate) fn get_last_result(&self) -> Option<f32> {
		self.last_result
	}

	pub(crate) fn set_last_result(&mut self, value: f32) -> f32 {
		self.last_result = Some(value);
		value
	}

	pub(crate) fn init_std(&mut self) {
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
		match env.get("var1") {
			Some(Variable::Var(val)) => assert_eq!(*val, 34.5),
			_ => panic!(),
		}
	}

	#[test]
	fn test_02_get_undefined() {
		let env = Environment::new();
		match env.get("xyz") {
			None => (),
			_ => panic!(),
		}
	}

	#[test]
	fn test_03_init() {
		let mut env = Environment::new();
		env.init_std();
		match env.get("pi") {
			Some(Variable::Var(val)) => assert_eq!(*val, PI),
			_ => panic!(),
		}

		match env.get("test") {
			Some(Variable::Fn(var)) => assert_eq!(var(4.0), 2.0),
			_ => panic!(),
		}
	}
}
