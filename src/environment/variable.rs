use super::Function;

pub enum Variable {
	Var(f32),
	Fn(Box<dyn Function>),
}
