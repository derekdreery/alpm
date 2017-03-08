//! The alpm_list type wrapper

use std::collections::LinkedList;

use libc::c_void;
use alpm_sys::*;

// NOTE the linked list in alpm_list_t is actually a doubly linked list, where you can't rely on
// the first item->prev = null

/// Convert an alpm_list_t to a rust Vec.
///
/// It's important not to take ownership of the data pointer (raw->data). It should be cleared
/// using the original data structure. If you do this you can let the vec go out of scope and not
/// touch the original data. The vec must not outlive the underlying struct, this bound can be
/// enforced by the type T having a lifetime parameter.
pub unsafe fn alpm_list_to_vec<T, F>(raw: *const alpm_list_t, f: F) -> Vec<T>
    where F: Fn(*const c_void) -> T
{
    let mut vec = Vec::new();
    if raw.is_null() {
        return vec;
    }
    // get first node
    let first = raw;
    let mut raw = raw;

    // copy list (not data)
    vec.push(f((*raw).data as *const c_void));
    while ! (*raw).next.is_null() {
        raw = (*raw).next;
        vec.push(f((*raw).data as *const c_void));
    }
    vec
}
