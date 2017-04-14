#![feature(trace_macros)]
//! Useful utility functions that complement libalpm, but are not in the C library.
//!
//! VERY MUCH WIP
#[macro_use] extern crate nom;
extern crate libalpm;

pub mod ini;
