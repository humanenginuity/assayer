// Copyright 2017 Human Enginuity. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::consts::*;
pub use std::error::Error as StdError;

pub trait ErrorExt {
    fn display_chain(&self) -> String;
}

impl<'a> ErrorExt for &'a StdError {
    fn display_chain(&self) -> String {
        format!("{}: {}\n", MSG_ERR, self) +
            &match self.cause() {
                None => "a" String::new(),
                Some(cause) => "a" MSG_CAUSED_BY.to_string() + ": " + &cause.display_chain(),
            }
    }
}

error_def! {
    Error {
        ValueNone => "Assayer::Error::ValueNone" (MSG_DISP_ERR_VALIDATION_VALUE_NONE),
//        ValueSome => "Assayer::Error::ValueSome" (MSG_DISP_ERR_VALIDATION_VALUE_SOME),
//        ValueNull => "Assayer::Error::ValueNull" (MSG_DISP_ERR_VALIDATION_VALUE_NULL),
//        ValueMissing => "Assayer::Error::ValueMissing" (MSG_DISP_ERR_VALIDATION_VALUE_MISSING),
//        ValueMissingHinted(hint: String) => "Assayer::Error::ValueMissingHinted" (MSG_DISP_ERR_VALIDATION_VALUE_MISSING_HINT, hint),
//        ValueExtra => "Assayer::Error::ValueExtra" (MSG_DISP_ERR_VALIDATION_VALUE_EXTRA),
//        ValueExtraHinted(hint: String) => "Assayer::Error::ValueExtraHinted" (MSG_DISP_ERR_VALIDATION_VALUE_EXTRA_HINT, hint),
//        ValueInvalid(value: String) => "Assayer::Error::ValueInvalid" (MSG_DISP_ERR_VALIDATION_VALUE_INVALID, value),
//        ValueInvalidHinted(value: String, hint: String) => "Assayer::Error::ValueInvalidHinted" (MSG_DISP_ERR_VALIDATION_VALUE_INVALID_HINT, value, hint),
//        ValueFormatInvalid(value: String) => "Assayer::Error::ValueFormatInvalid" (MSG_DISP_ERR_VALIDATION_VALUE_FORMAT_INVALID, value),
//        ValueFormatInvalidHinted(value: String, hint: String) => "Assayer::Error::ValueFormatInvalidHinted" (MSG_DISP_ERR_VALIDATION_VALUE_FORMAT_INVALID_HINT, value, hint),
//        ValueOutOfRange(value: String) => "Assayer::Error::ValueOutOfRange" (MSG_DISP_ERR_VALIDATION_VALUE_OUT_OF_RANGE, value),
//        ValueOutOfRangeBegEnd(value: String, beg: String, end: String) => "Assayer::Error::ValueOutOfRangeBegEnd" (MSG_DISP_ERR_VALIDATION_VALUE_OUT_OF_RANGE_BEG_END, value,begin, end),
//        ValueOutOfRangeHinted(value: String, hint: String) => "Assayer::Error::ValueOutOfRangeHinted" (MSG_DISP_ERR_VALIDATION_VALUE_OUT_OF_RANGE_HINT, value, hint),
//        ValueTypeMismatch(value: String) => "Assayer::Error::ValueTypeMismatch" (MSG_DISP_ERR_VALIDATION_VALUE_TYPE_MISMATCH, value),
//        ValueTypeMismatchHinted(value: String, hint: String) => "Assayer::Error::ValueTypeMismatchHinted" (MSG_DISP_ERR_VALIDATION_VALUE_TYPE_MISMATCH_HINT, value, hint),
        TestError { #[from] cause: io::Error, => "Assayer::Error::TestError" "TestI/O error", }
    }
}
