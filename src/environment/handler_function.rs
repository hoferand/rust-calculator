use crate::Error;

use super::{Arguments, Function};

pub struct HandlerFunction<H: Clone> {
	pub handler: H,
	pub call: fn(&H, &mut dyn Arguments) -> Result<f32, Error>,
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
		(self.call)(&self.handler, args)
	}
}
