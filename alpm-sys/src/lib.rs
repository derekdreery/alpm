#![feature(untagged_unions)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;
extern crate libarchive3_sys;

mod ffi;
pub use ffi::*;

