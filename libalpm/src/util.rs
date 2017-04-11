//! Some helper functions not directly related to the API

use libc;
use std::mem;
use std::ptr;
use std::ffi::{CStr, CString};
use alpm_sys::*;

/// A wrapper around a libc::utsname struct, holding information on the current computer and os.
pub struct UtsName(libc::utsname);

impl UtsName {
    /// The system name
    pub fn sysname(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.sysname.as_ptr())
                .to_str().expect("sysname not valid utf8 (should never happen)")
        }
    }

    /// The node name
    pub fn nodename(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.nodename.as_ptr())
                .to_str().expect("nodename not valid utf8 (should never happen)")
        }
    }

    /// The release
    pub fn release(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.release.as_ptr())
                .to_str().expect("release not valid utf8 (should never happen)")
        }
    }

    /// The version
    pub fn version(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.version.as_ptr())
                .to_str().expect("version not valid utf8 (should never happen)")
        }
    }

    /// Useful as value for "arch"
    pub fn machine(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.0.machine.as_ptr())
                .to_str().expect("machine not valid utf8 (should never happen)")
        }
    }
}

/// A simple safe wrapper round libc::uname.
pub fn uname() -> UtsName {
    unsafe {
        let mut raw: libc::utsname = mem::zeroed();
        let res = libc::uname(&mut raw as *mut libc::utsname);
        if res == 0 { // success
            UtsName(raw)
        } else {
            panic!("uname function errored (this shouldn't happen)")
        }
    }
}

// NOTE the linked list in alpm_list_t is actually a doubly linked list, where you can't rely on
// the first item->prev = null

/// Convert an alpm_list_t to a rust Vec.
///
/// It's important not to take ownership of the data pointer (raw->data). It should be cleared
/// using the original data structure. If you do this you can let the vec go out of scope and not
/// touch the original data. The vec must not outlive the underlying struct, this bound can be
/// enforced by the type T having a lifetime parameter.
pub(crate) unsafe fn alpm_list_to_vec<T, F>(raw: *const alpm_list_t, f: F) -> Vec<T>
    where F: Fn(*const libc::c_void) -> T
{
    let mut vec = Vec::new();
    if raw.is_null() {
        return vec;
    }
    // get first node
    let first = raw;
    let mut raw = raw;

    // copy list (not data)
    vec.push(f((*raw).data as *const libc::c_void));
    while ! (*raw).next.is_null() {
        raw = (*raw).next;
        vec.push(f((*raw).data as *const libc::c_void));
    }
    vec
}

/// Convert a rust_vec to an alpm_list_t.
///
/// This function passes ownership of the contained data to alpm - so it must be allocated with
/// libc::malloc and friends
pub(crate) unsafe fn vec_to_alpm_list<T, F>(v: Vec<T>, f: F) -> *const alpm_list_t
    where F: Fn(&T) -> *const libc::c_void
{
    if v.len() == 0 {
        return ptr::null();
    }

    // init everything to null
    let list = libc::calloc(1, mem::size_of::<alpm_list_t>()) as *const alpm_list_t;
    let mut list_prev = list as *mut alpm_list_t;
    // convert first element and put it in
    (*list_prev).data = f(&v[0]);
    // for each remaining element
    for el in v.iter().skip(1) {
        // create null alpm_list_t
        let mut list_inner = libc::calloc(1, mem::size_of::<alpm_list_t>()) as *mut alpm_list_t;
        // add in value
        (*list_inner).data = f(el);
        // hook up to prev alpm_list_t
        (*list_inner).prev = list_prev;
        (*list_prev).next = list_inner;

        list_prev = list_inner;
    }
    list
}

/// Borrow a rust_vec as an alpm_list_t. This list must be freed by the user using `alpm_list_free`.
///
/// Borrowed vec must live longer than the c library needs it for (this can't be checked)
pub(crate) unsafe fn vec_as_alpm_list<T, F>(v: &Vec<T>, f: F) -> *const alpm_list_t
    where F: Fn(&T) -> *const libc::c_void
{
    if v.len() == 0 {
        return ptr::null();
    }

    // init everything to null
    let list = libc::calloc(1, mem::size_of::<alpm_list_t>()) as *const alpm_list_t;
    let mut list_prev = list as *mut alpm_list_t;
    // convert first element and put it in
    (*list_prev).data = f(&v[0]);
    // for each remaining element
    for el in v.iter().skip(1) {
        // create null alpm_list_t
        let mut list_inner = libc::calloc(1, mem::size_of::<alpm_list_t>()) as *mut alpm_list_t;
        // add in value
        (*list_inner).data = f(el);
        // hook up to prev alpm_list_t
        (*list_inner).prev = list_prev;
        (*list_prev).next = list_inner;

        list_prev = list_inner;
    }
    list
}

/// Convert a str to unowned raw mem allocated with libc::malloc
pub(crate) unsafe fn str_to_unowned_char_array(s: *const &str) -> *const libc::c_void {
    let len = (*s).len();
    // remember extra byte for '\0' (will be zero since calloc)
    let mut p = libc::calloc(len + 1, mem::size_of::<libc::c_char>()) as *mut libc::c_char;
    ptr::copy::<libc::c_char>((*s).as_ptr() as *const i8, p, len);
    p as *const libc::c_void
}

/// Convert a str to owned raw mem allocated (and deallocated) in rust
pub(crate) fn cstring_to_owned_char_array(s: &CString) -> *const libc::c_void {
    s.as_ptr() as *const libc::c_void
}

/// A TEMPORARY helper function to extract server urls from a pacman conf file (until I've
/// implemented `Options::from_ini`).
#[deprecated]
pub fn get_servers(path: &str, repo: &str, arch: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let mut servers = Vec::new();
    let file = BufReader::new(File::open(path).unwrap());
    for line in file.lines() {
        let line = line.unwrap();
        if line.starts_with("Server = ") {
            let server = line
                .replace("$repo", repo)
                .replace("$arch", arch)
                .replace("Server = ", "");
            servers.push(server);
        }
    }
    servers
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_uname() {
        let info = uname();
        assert_eq!(info.sysname(), "Linux");
    }
}
