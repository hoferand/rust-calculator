use std::collections::HashMap;
use std::f32::consts::{E, PI};

pub struct Environment {
	values: HashMap<String, f32>,
}

impl Environment {
	pub fn assign(&mut self, key: String, value: f32) -> Result<f32, String> {
		self.values.insert(key, value);
		Ok(value)
	}

	pub fn get(&mut self, key: String) -> Result<f32, String> {
		let value = self.values.get(&key);
		match value {
			Some(value) => Ok(*value),
			None => Err(format!("")),
		}
	}
}

pub fn new() -> Environment {
	let mut env = Environment {
		values: HashMap::new(),
	};
	env.assign(String::from("pi"), PI).expect("");
	env.assign(String::from("e"), E).expect("");
	return env;
}
