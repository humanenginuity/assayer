#[allow(unused_imports)]
use super::consts::*;
use std::env::ArgsOs;
use std::ops::{Deref, DerefMut};
pub use assayer::Error as ValidatorError;
use assayer::Validator;

//Newtype Args wrapper around Vec<String> necessary to implement Validator<Args>.
#[derive(Clone, Debug)]
pub struct Args(Vec<String>);

impl Args {
    pub fn new(args: Vec<String>) -> Self {
        Args(args)
    }
}

impl Deref for Args {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Args {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ArgsOs> for Args {
    fn from(args_os: ArgsOs) -> Self {
        Args(args_os.map(|arg| arg.to_string_lossy()    //any non-unicode sequence -> � (U+FFFD replacement character)
                                  .to_string())
                    .collect::<Vec<_>>())
    }
}

impl Validator<Args> for Args {
    fn validate(value: Args) -> Result<Self, ValidatorError> {
        Ok(value)
    }
}
