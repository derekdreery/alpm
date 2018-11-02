extern crate libc;

use libc::{c_int, c_char, c_void};

// Test function takes a format string, and a variadic list
pub type Callback = extern "C" fn(*const c_char, *mut c_void) -> *mut c_void;

#[link(name = "printf-helper", kind = "static")]
extern "C" {
    pub fn dispatch(test_no: c_int, cb: Callback) -> *mut c_void;
}
