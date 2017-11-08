// Copyright 2017 Human Enginuity. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]
#![deny(unused_must_use, overflowing_literals)]
#![feature(plugin)]
#![plugin(error_def)]

extern crate failure;
#[macro_use] extern crate derive_fail;

#[cfg(test)] mod unit_tests;
pub mod consts;
mod error;

pub use error::Error;

type Result<T> = std::result::Result<T, self::error::Error>;

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

// By-Value (Consuming) Validators
pub trait Validator<T> {
    fn validate(value: T) -> Result<T>;
}

pub trait MethodSyntaxValidator: Sized {
    fn validate<T: Validator<Self>>(self) -> Result<Self>;
}

impl<T> MethodSyntaxValidator for T {
    fn validate<U: Validator<Self>>(self) -> Result<Self> { U::validate(self) }
}
