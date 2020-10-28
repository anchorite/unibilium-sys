//! # Overview
//!
//! This is wrapper around C unibilium library. Use instead the higher level unibilium library,
//! which provided Rust friendly interface around this one.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CStr;
use std::fmt;

impl fmt::Display for unibi_boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cstr = unsafe { CStr::from_ptr(unibi_name_bool(*self)) };
        write!(
            f,
            "{}",
            cstr.to_str().expect("Failed to parse UTF-8 string")
        )
    }
}

impl fmt::Display for unibi_numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cstr = unsafe { CStr::from_ptr(unibi_name_num(*self)) };
        write!(
            f,
            "{}",
            cstr.to_str().expect("Failed to parse UTF-8 string")
        )
    }
}

impl fmt::Display for unibi_string {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cstr = unsafe { CStr::from_ptr(unibi_name_str(*self)) };
        write!(
            f,
            "{}",
            cstr.to_str().expect("Failed to parse UTF-8 string")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_display() {
        assert_eq!(
            format!("{}", unibi_boolean::unibi_return_does_clr_eol),
            "return_does_clr_eol"
        );
    }
}
