pub(crate) enum Variable {
	Var(f32),
	Fn(fn(f32) -> f32),
}
