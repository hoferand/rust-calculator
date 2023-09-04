use crate::{Arguments, Error};

pub(crate) trait FromArguments: Sized {
	fn from_args(args: &mut dyn Arguments) -> Result<Self, Error>;
}

impl FromArguments for f32 {
	fn from_args(args: &mut dyn Arguments) -> Result<Self, Error> {
		args.get_next_arg()
	}
}
