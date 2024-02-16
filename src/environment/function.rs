use crate::{Arguments, Error};

pub trait Function {
	fn clone_box(&self) -> Box<dyn Function>;
	fn call_with_args(&self, args: &mut dyn Arguments) -> Result<f32, Error>;
}
