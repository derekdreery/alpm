#![feature(trace_macros)]
//! Useful utility functions that complement libalpm, but are not in the C library.
//!
//! VERY MUCH WIP
extern crate libalpm;
#[macro_use]
extern crate nom;

pub mod ini;
