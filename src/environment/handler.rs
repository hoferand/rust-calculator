use crate::{Arguments, Error, FromArguments, IntoResult};

pub trait Handler<T> {
	fn call(&self, args: &mut dyn Arguments) -> Result<f32, Error>;
}

impl_handler!(T1);
impl_handler!(T1, T2);
impl_handler!(T1, T2, T3);

macro_rules! impl_handler {
	($($ty:ident),*) => {
		impl<F, $($ty,)* R> Handler<($($ty,)*)> for F
		where
			F: Fn($($ty,)*) -> R,
			$($ty: FromArguments,)*
			R: IntoResult
		{
			fn call(&self, args: &mut dyn Arguments) -> Result<f32, Error> {
				(self)($($ty::from_args(args)?,)*).into_result()
			}
		}
	}
}

use impl_handler;
