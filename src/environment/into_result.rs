use crate::Error;

pub trait IntoResult {
	fn into_result(self) -> Result<f32, Error>;
}

impl IntoResult for f32 {
	fn into_result(self) -> Result<f32, Error> {
		Ok(self)
	}
}

impl IntoResult for Result<f32, Error> {
	fn into_result(self) -> Result<f32, Error> {
		self
	}
}
