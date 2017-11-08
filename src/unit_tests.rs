// Copyright 2017 Human Enginuity. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(unused_imports)]
use super::*;
use super::Error;
use consts::*;

#[derive(Debug, PartialEq, Eq)]
struct NonEmptyStringValidator {
    value: String,
}

//Example DRY implementation of ref, mut_ref and consuming validator
impl ValidatorRef<String> for NonEmptyStringValidator {
    fn validate_ref(input: &String) -> Result<&String> {
       Ok(input)
           .and_then(|input| match !input.is_empty() {
                true => Ok(input),
                false => Err(Error::ValueNone),
            })
    }
}

impl ValidatorMutRef<String> for NonEmptyStringValidator {
    fn validate_mut_ref(input: &mut String) -> Result<&mut String> {
        Ok(input)
            .and_then(|input| match (&*input).validate_ref::<NonEmptyStringValidator>() {
                Ok(_) => Ok(input),
                Err(err) => Err(err),
            })
    }
}

impl Validator<String> for NonEmptyStringValidator {
    fn validate(input: String) -> Result<String> {
        Ok(input)
            .and_then(|input| match (&input).validate_ref::<NonEmptyStringValidator>() {
                Ok(_) => Ok(input),
                Err(err) => Err(err),
            })
    }
}

#[test]
fn string_validator_ref_handles_non_empty_input() {
    let input = "non-empty test ref value".to_string();
    let expected_result = Some(&input);

    assert_eq!((&input).validate_ref::<NonEmptyStringValidator>().ok(), expected_result);
}

#[test]
fn string_validator_ref_handles_empty_input() {
    let input = String::new();
    let expected_result = Some(Error::ValueNone);

    assert_eq!((&input).validate_ref::<NonEmptyStringValidator>().err(), expected_result);
}

#[test]
fn string_validator_mut_ref_handles_non_empty_input() {
    let mut input = "non-empty test mut ref value".to_string();
    let mut result_inner = input.clone();
    let expected_result = Some(&mut result_inner);

    assert_eq!((&mut input).validate_mut_ref::<NonEmptyStringValidator>().ok(), expected_result);
}

#[test]
fn string_validator_mut_ref_handles_empty_input() {
    let mut input = String::new();
    let expected_result = Some(Error::ValueNone);

    assert_eq!((&mut input).validate_mut_ref::<NonEmptyStringValidator>().err(), expected_result);
}

#[test]
fn string_validator_handles_non_empty_input() {
    let input = "non-empty test value".to_string();
    let expected_result = Some(input.clone());

    assert_eq!(input.validate::<NonEmptyStringValidator>().ok(), expected_result);
}

#[test]
fn string_validator_handles_empty_input() {
    let input = String::new();
    let expected_result = Some(Error::ValueNone);

    assert_eq!(input.validate::<NonEmptyStringValidator>().err(), expected_result);
}
