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
    ValueNone,
    ValueSome,
    ValueNull,
    ValueMissing,
    ValueExtra,
    ValueInvalid,
    ValueFormatInvalid,
    ValueOutOfRange,
    ValueTypeMismatch,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ValueNone => MSG_DISP_ERR_VALIDATION_VALUE_NONE,
            ValueSome => MSG_DISP_ERR_VALIDATION_VALUE_SOME,
            ValueNull => MSG_DISP_ERR_VALIDATION_VALUE_NULL,
            ValueMissing => MSG_DISP_ERR_VALIDATION_VALUE_MISSING,
            ValueExtra => MSG_DISP_ERR_VALIDATION_VALUE_EXTRA,
            ValueInvalid => MSG_DISP_ERR_VALIDATION_VALUE_INVALID,
            ValueFormatInvalid => MSG_DISP_ERR_VALIDATION_VALUE_FORMAT_INVALID,
            ValueOutOfRange => MSG_DISP_ERR_VALIDATION_VALUE_OUT_OF_RANGE,
            ValueTypeMismatch => MSG_DISP_ERR_VALIDATION_VALUE_TYPE_MISMATCH,
        })
    }
}
