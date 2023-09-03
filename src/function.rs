use crate::{Arguments, Error};

/// Function
pub(crate) trait Function {
	fn clone_box(&self) -> Box<dyn Function>;
	fn call_with_args(&self, args: &mut dyn Arguments) -> Result<f32, Error>;
}

/// HandlerFunction
pub(crate) struct HandlerFunction<H: Clone> {
	pub handler: H,
	pub call: fn(H, &mut dyn Arguments) -> Result<f32, Error>,
}

impl<H: Clone> Clone for HandlerFunction<H> {
	fn clone(&self) -> Self {
		Self {
			handler: self.handler.clone(),
			call: self.call,
		}
	}
}

impl<H> Function for HandlerFunction<H>
where
	H: Clone + 'static,
{
	fn clone_box(&self) -> Box<dyn Function> {
		Box::new(self.clone())
	}

	fn call_with_args(&self, args: &mut dyn Arguments) -> Result<f32, Error> {
		(self.call)(self.handler.clone(), args)
	}
}
