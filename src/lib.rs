#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

mod consts;
mod error;

#[cfg(test)] mod unit_tests;

pub use self::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

// Ref Validators
pub trait ValidatorRef<T> {
    fn validate_ref(value: &T) -> Result<&T>;
}

pub trait MethodSyntaxValidatorRef: Sized {
    fn validate_ref<T: ValidatorRef<Self>>(&self) -> Result<&Self>;
}

impl<T> MethodSyntaxValidatorRef for T {
    fn validate_ref<U: ValidatorRef<Self>>(&self) -> Result<&Self> { U::validate_ref(self) }
}

// MutRef Validators
pub trait ValidatorMutRef<T> {
    fn validate_mut_ref(value: &mut T) -> Result<&mut T>;
}

pub trait MethodSyntaxValidatorMutRef: Sized {
    fn validate_mut_ref<T: ValidatorMutRef<Self>>(&mut self) -> Result<&mut Self>;
}
impl<T> MethodSyntaxValidatorMutRef for T {
    fn validate_mut_ref<U: ValidatorMutRef<Self>>(&mut self) -> Result<&mut Self> { U::validate_mut_ref(self) }
}

// Consuming Validators
pub trait Validator<T> {
    fn validate(value: T) -> Result<T>;
}

pub trait MethodSyntaxValidator: Sized {
    fn validate<T: Validator<Self>>(self) -> Result<Self>;
}

impl<T> MethodSyntaxValidator for T {
    fn validate<U: Validator<Self>>(self) -> Result<Self> { U::validate(self) }
}
