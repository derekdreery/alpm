extern crate printf;
extern crate libc;

use libc::{c_char, c_int, c_void};

// Test function takes a format string, and a variadic list
type Callback = extern "C" fn(*const c_char, *mut c_void) -> *mut c_void;

#[link(name="printf_test_helper")]
extern "C" {
    fn dispatch(test_no: c_int, cb: Callback) -> *mut c_void;
}

// This should match Callback signature
extern "C" fn test_cb(format: *const c_char, args: *mut c_void) -> *mut c_void {
    unsafe {
        let out = Box::new(printf::printf(format, args));
        println!("{:?}", out);
        Box::into_raw(out) as *mut c_void
    }
}

#[test]
fn simple() {

    let tests = vec![
        (1, "testing printf format: 1\n"),
        (2, "Characters: a A"),
        (3, "Decimals: 1977 650000"),
        (4, "Preceding with blanks:       1977"),
        (5, "Preceding with zeros: 0000001977"),
        (6, "Some different radices: 100 64 144 0x64 0144"),
        (7, "floats: 3.14 +3e+00 3.141600E+00"),
        (8, "Width trick:    10"),
        (9, "A string")
    ];
    for (num, cstr) in tests {
        unsafe {
            let out: Box<String> = Box::from_raw(dispatch(num, test_cb) as *mut String);
            assert_eq!(cstr, *out);
        }
    }
}
