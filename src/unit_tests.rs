#[allow(unused_imports)]
use super::*;
use consts::*;

#[derive(Debug, PartialEq, Eq)]
struct NonEmptyStringValidator {
    value: String,
}

//Example DRY implementation of ref, mut_ref and consuming validator
impl ValidatorRef<String> for NonEmptyStringValidator {
    fn validate_ref(v: &String) -> Result<&String> {
       Ok(v)
           .and_then(|v| match !v.is_empty() {
                true => Ok(v),
                false => Err(Error::ValueEmpty(MSG_ERR_VALIDATION_VALUE_EMPTY.to_string())),
            })
    }
}

impl ValidatorMutRef<String> for NonEmptyStringValidator {
    fn validate_mut_ref(v: &mut String) -> Result<&mut String> {
        Ok(v)
            .and_then(|v| match (&*v).validate_ref::<NonEmptyStringValidator>() {
                Ok(_) => Ok(v),
                Err(err) => Err(err),
            })
    }
}

impl Validator<String> for NonEmptyStringValidator {
    fn validate(v: String) -> Result<String> {
        Ok(v)
            .and_then(|v| match (&v).validate_ref::<NonEmptyStringValidator>() {
                Ok(_) => Ok(v),
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
    let expected_result = Some(Error::ValueEmpty(MSG_ERR_VALIDATION_VALUE_EMPTY.to_string()));

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
    let expected_result = Some(Error::ValueEmpty(MSG_ERR_VALIDATION_VALUE_EMPTY.to_string()));

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
    let expected_result = Some(Error::ValueEmpty(MSG_ERR_VALIDATION_VALUE_EMPTY.to_string()));

    assert_eq!(input.validate::<NonEmptyStringValidator>().err(), expected_result);
}
