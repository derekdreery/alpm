//! This crate provides a method to convert printf-style calls to a rust formatter
extern crate libc;

use std::ffi::CStr;

use libc as c;

#[link(name = "printf_wrapper")]
extern "C" {
    fn printf_wrapper(format: *const c::c_char, args: *mut c::c_void) -> *const c::c_char;
}

/// Take a printf c-string and variadic array, and write equiv. out to the formatter
///
/// # Safety
/// This function is UB if the va_list doesn't match the format (c printf syntax)
///
/// There must be no panics in this function, so quite often errors are deliberately ignored
pub unsafe fn printf(format: *const c::c_char, args: *mut c::c_void) -> String
{
    let out_char_p = printf_wrapper(format, args);
    let output = CStr::from_ptr(out_char_p).to_string_lossy().into_owned();
    c::free(out_char_p as *mut c::c_void);
    output
}
