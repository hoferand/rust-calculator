use crate::{Arguments, Error};

/// CustomFunction
pub trait Handler<T> {
	fn call(&self, args: &mut dyn Arguments) -> Result<f32, Error>;
}

impl<F, T, R> Handler<T> for F
where
	F: Fn(T) -> R,
	T: FromArguments,
	R: IntoResult,
{
	fn call(&self, args: &mut dyn Arguments) -> Result<f32, Error> {
		(self)(T::from_args(args)?).into_result()
	}
}

impl<F, T1, T2, R> Handler<(T1, T2)> for F
where
	F: Fn(T1, T2) -> R,
	T1: FromArguments,
	T2: FromArguments,
	R: IntoResult,
{
	fn call(&self, args: &mut dyn Arguments) -> Result<f32, Error> {
		(self)(T1::from_args(args)?, T2::from_args(args)?).into_result()
	}
}

impl<F, T1, T2, T3, R> Handler<(T1, T2, T3)> for F
where
	F: Fn(T1, T2, T3) -> R,
	T1: FromArguments,
	T2: FromArguments,
	T3: FromArguments,
	R: IntoResult,
{
	fn call(&self, args: &mut dyn Arguments) -> Result<f32, Error> {
		(self)(
			T1::from_args(args)?,
			T2::from_args(args)?,
			T3::from_args(args)?,
		)
		.into_result()
	}
}

/// FromArguments
pub(crate) trait FromArguments: Sized {
	fn from_args(args: &mut dyn Arguments) -> Result<Self, Error>;
}

impl FromArguments for f32 {
	fn from_args(args: &mut dyn Arguments) -> Result<Self, Error> {
		args.get_next_arg()
	}
}

/// IntoResult
pub(crate) trait IntoResult {
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
