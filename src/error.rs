use super::consts::*;
use super::fluent_validator;
use std::error::Error as StdError;
use std::fmt;

pub enum Error {
    ValidatorError(fluent_validator::Error),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ValidatorError(_) => ERR_FLUENT_VALIDATION,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::ValidatorError(ref inner_err) => Some(inner_err),
        }
    }
}

#[allow(unused_variables)]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ValidatorError(ref msg) => f.write_str(&err_msg(ERR_VALIDATION, self.description())),
        }
    }
}

//#[allow(dead_code)]
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self, f)
    }
}

fn err_msg(err_name: &str, err_detail: &str)-> String {
    format!("{}: {}!", err_name, err_detail)
}

impl From<fluent_validator::Error> for Error {
    fn from(error: fluent_validator::Error) -> Self { Error::ValidatorError(error) }
}
