#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

extern crate RustQuickstartTemplate;

pub const INVALID_UTF8_ARG: &'static str            = "Invalid UTF-8 argument found.  Processing aborted.";
pub const UNKNOWN_APP_NAME: &'static str            = "<Unknown app name>";
pub const ERROR: &'static str                       = "Error";

pub fn main()
{
    let args = std::env::args_os().map(|arg| arg.into_string().expect(INVALID_UTF8_ARG))
                                  .collect::<Vec<_>>();
    let app_name = match args.first() {
        Some(name) => name.to_string(),
        _ => UNKNOWN_APP_NAME.to_string(),
    };

    match RustQuickstartTemplate::run(args) {
        Err(e) => panic!(format!("{}: {}: {}", app_name, ERROR, e.description())),
        _ => (),
    }
}
