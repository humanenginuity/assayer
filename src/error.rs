// Copyright 2017 Human Enginuity. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use super::consts::*;

#[derive(Fail, Debug, PartialEq)]
pub enum Error {
    NoValue,
    HasValue,
    EmptyValue,
    MissingValue,
    ExtraValue,
    InvalidValue,
    ValueFormatInvalid,
    ValueOutOfRange,
    ValueTypeMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::NoValue => MSG_DISP_ERR_VALIDATION_NO_VALUE,
            Error::HasValue => MSG_DISP_ERR_VALIDATION_HAS_VALUE,
            Error::EmptyValue => MSG_DISP_ERR_VALIDATION_NULL_VALUE,
            Error::MissingValue => MSG_DISP_ERR_VALIDATION_MISSING_VALUE,
            Error::ExtraValue => MSG_DISP_ERR_VALIDATION_VALUE_EXTRA,
            Error::InvalidValue => MSG_DISP_ERR_VALIDATION_VALUE_INVALID,
            Error:: ValueFormatInvalid => MSG_DISP_ERR_VALIDATION_VALUE_FORMAT_INVALID,
            Error::ValueOutOfRange => MSG_DISP_ERR_VALIDATION_VALUE_OUT_OF_RANGE,
            Error::ValueTypeMismatch => MSG_DISP_ERR_VALIDATION_VALUE_TYPE_MISMATCH,
        })
    }
}
