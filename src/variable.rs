use crate::{Arguments, Error};

pub(crate) enum Variable {
	Var(f32),
	Fn(fn(f32) -> f32),
	Custom(fn(&mut dyn Arguments) -> Result<f32, Error>),
}
