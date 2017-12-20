#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(untagged_unions)]

extern crate libarchive3_sys;
extern crate libc;

mod ffi;
pub use ffi::*;
