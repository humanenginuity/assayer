// Copyright 2017 Human Enginuity. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::consts::*;
use std::error::Error as StdError;
use std::fmt;

/// This type represents all possible errors that can occur when validating a type.
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    ValueEmpty(String),
    ValueInvalid(String),
    Multiple(Vec<Error>),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ValueEmpty(ref msg) => msg.as_str(),
            Error::ValueInvalid(ref msg) => msg.as_str(),
            Error::Multiple(_) => MSG_ERR_VALIDATION_MULTIPLE,
        }
    }
}

/// Returns the error's `description()` prefixed by the error's type.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ValueEmpty(_) => f.write_str(format!("{}: {}", MSG_ERR_VALIDATION_VALUE_EMPTY, self.description()).as_str()),
            Error::ValueInvalid(_) => f.write_str(format!("{}: {}", MSG_ERR_VALIDATION_VALUE_INVALID, self.description()).as_str()),
            Error::Multiple(ref slice) => {
                let mut msg = format!("{}:", self.description());
                for (i, err) in slice.iter().enumerate() {
                    msg.push_str(format!("\n\t#{}: {}", i + 1, &err.to_string()).as_str());
                }
                f.write_str(msg.as_str())
            },
        }
    }
}
