// Copyright 2017 Human Enginuity. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//error.rs
pub const MSG_ERR: &'static str = "Error";
pub const MSG_CAUSED_BY: &'static str = "caused by";

pub const MSG_DISP_ERR_VALIDATION_NO_VALUE: &'static str = "Found `None` value but expected a `Some()` value";
pub const MSG_DISP_ERR_VALIDATION_HAS_VALUE: &'static str = "Found `Some()` value but expected a `None` value";
pub const MSG_DISP_ERR_VALIDATION_NULL_VALUE: &'static str = "Found `null` value but expected a non-null value";
pub const MSG_DISP_ERR_VALIDATION_MISSING_VALUE: &'static str = "Missing one or more expected values";
pub const MSG_DISP_ERR_VALIDATION_VALUE_EXTRA: &'static str = "Found one or more unexpected extra values";
pub const MSG_DISP_ERR_VALIDATION_VALUE_INVALID: &'static str = "An invalid value was found";
pub const MSG_DISP_ERR_VALIDATION_VALUE_FORMAT_INVALID: &'static str = "An invalid format was found";
pub const MSG_DISP_ERR_VALIDATION_VALUE_OUT_OF_RANGE: &'static str = "Value is outside the acceptable range of values";
pub const MSG_DISP_ERR_VALIDATION_VALUE_TYPE_MISMATCH: &'static str = "Value is not of the expected type";

//General
