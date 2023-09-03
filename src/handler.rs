use crate::{Arguments, Error, FromArguments, IntoResult};

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
