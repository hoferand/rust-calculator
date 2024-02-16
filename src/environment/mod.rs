use std::collections::HashMap;
use std::f32::consts::{E, PI};

pub mod arguments;
pub use arguments::*;
pub mod handler;
pub use handler::*;
pub mod variable;
pub use variable::*;
mod from_arguments;
use from_arguments::*;
mod function;
use function::*;
mod handler_function;
use handler_function::*;
mod into_result;
use into_result::*;

pub struct Environment {
	variables: HashMap<String, Variable>,
	last_result: Option<f32>,
}

impl Default for Environment {
	fn default() -> Self {
		Self::new()
	}
}

impl Environment {
	pub fn new() -> Environment {
		Environment {
			variables: HashMap::new(),
			last_result: None,
		}
	}

	pub fn assign_var(&mut self, key: impl Into<String>, value: f32) {
		self.variables.insert(key.into(), Variable::Var(value));
	}

	pub fn assign_fn<H, T>(&mut self, id: impl Into<String>, fun: H)
	where
		H: Handler<T> + Clone + 'static,
		T: 'static,
	{
		let hf = HandlerFunction {
			handler: fun,
			call: |h, ctx| h.call(ctx),
		};
		self.variables.insert(id.into(), Variable::Fn(Box::new(hf)));
	}

	pub fn get(&self, key: &str) -> Option<&Variable> {
		return self.variables.get(key);
	}

	pub fn get_last_result(&self) -> Option<f32> {
		self.last_result
	}

	pub fn set_last_result(&mut self, value: f32) -> f32 {
		self.last_result = Some(value);
		value
	}

	pub fn init_std(&mut self) {
		self.assign_var("pi", PI);
		self.assign_var("e", E);

		self.assign_fn("sin", f32::sin);
		self.assign_fn("asin", f32::asin);
		self.assign_fn("cos", f32::cos);
		self.assign_fn("acos", f32::acos);
		self.assign_fn("tan", f32::tan);
		self.assign_fn("atan", f32::atan);
		self.assign_fn("r2d", f32::to_degrees);
		self.assign_fn("d2r", f32::to_radians);

		#[cfg(test)]
		self.assign_fn("test", |a: f32| a / 2.0);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_01_assignment() {
		let mut env = Environment::new();
		env.assign_var("var1", 34.5);
		match env.get("var1") {
			Some(Variable::Var(val)) => assert_eq!(*val, 34.5),
			_ => panic!(),
		}
	}

	#[test]
	fn test_02_get_undefined() {
		let env = Environment::new();
		if env.get("xyz").is_some() {
			panic!();
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
			Some(Variable::Fn(_)) => (),
			_ => panic!(),
		}
	}
}
