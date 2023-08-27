use crate::{Arguments, Error};

pub(crate) enum Variable {
	Var(f32),
	Fn(fn(&mut dyn Arguments) -> Result<f32, Error>),
}
