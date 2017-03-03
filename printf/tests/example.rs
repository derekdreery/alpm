extern crate printf;
extern crate va_list;

use va_list::VaList;
use std::os::raw::{c_char, c_int};
use std::ffi::CString;

// Test function takes a format string, and a variadic list
type Callback = extern "C" fn(*const c_char, VaList) -> *mut c_char;

#[link(name="printf_test_helper")]
extern "C" {
    fn dispatch(test_no: c_int, cb: Callback) -> *mut c_char;
}

// This should match Callback signature
extern "C" fn test_cb(format: *const c_char, args: VaList) -> *mut c_char {
    unsafe {
        let mut out: Vec<u8> = Vec::new();
        printf::printf(&mut out, format, args);
        match CString::new(out) {
            Ok(out) => out.into_raw(),
            _ => 0 as *mut c_char
        }
    }
}

#[test]
fn simple() {

    let tests = vec![
        (1, &b"testing printf format: 01\n"[..]),
        (2, b"Characters: a A"),
        (3, b"Decimals: 1977 650000"),
        (4, b"Preceding with blanks:       1977"),
        (5, b"Preceding with zeros: 0000001977"),
        (6, b"Some different radices: 100 64 144 0x64 0144"),
        (7, b"floats: 3.14 +3e+000 3.141600E+000"),
        (8, b"Width trick:    10"),
        (9, b"A string")
    ];
    for (num, cstr) in tests {
        unsafe {
            let out = dispatch(num, test_cb);
            let out = CString::from_raw(out);
            assert_eq!(CString::new(cstr).unwrap(), out);
        }
    }
}
