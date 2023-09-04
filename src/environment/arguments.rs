use crate::Error;

/// Used for getting arguments for function calls.
pub trait Arguments {
	fn get_next_arg(&mut self) -> Result<f32, Error>;
}
