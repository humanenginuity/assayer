#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate fluent_validator;
use fluent_validator::Args;

pub fn main() {
    match fluent_validator::run(Args::from(std::env::args_os())) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
