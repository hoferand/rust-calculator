use crate::Function;

pub(crate) enum Variable {
	Var(f32),
	Fn(Box<dyn Function>),
}
