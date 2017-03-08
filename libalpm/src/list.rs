//! The alpm_list type wrapper

use std::ptr;
use std::marker::PhantomData;

use libc::c_void;
use alpm_sys::*;

/// A linked list, where the data is behind a void pointer
#[repr(C)]
pub struct ListNode {
    data: *const c_void,
    prev: *const ListNode,
    next: *const ListNode,
}

impl ListNode {
    fn new() -> ListNode {
        ListNode {
            data: ptr::null(),
            prev: ptr::null(),
            next: ptr::null(),
        }
    }

}

#[repr(C)]
pub struct List<T> {
    first: *const ListNode,
    p: PhantomData<T>,
}

impl<T> List<T> {
    /// Finds the first node
    pub(crate) unsafe fn from_raw<U>(raw: *const alpm_list_t) -> List<U> {
        let mut raw = raw;
        while ! (*raw).prev.is_null() {
            raw = (*raw).prev;
        }
        List {
            first: raw as _,
            p: PhantomData,
        }
    }

    /// Adds the given element to the back of the list.
    pub fn push_back(&mut self, el: T) {
        unimplemented!()
    }

    /// Removes and returns the given element from the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        unimplemented!()
    }

    /// Adds the given element to the front of the list.
    pub fn push_front(&mut self, el: T) {
        unimplemented!()
    }

    /// Removes and returns the given element from the front of the list.
    pub fn pop_front(&mut self, el: T) {
        unimplemented!()
    }

    /// Adds the given element before the element at idx.
    pub fn insert_before(&mut self, idx: usize, el: T) {
        unimplemented!()
    }

    /// Gets the element at idx. Returns `None` if index is out of bounds.
    pub fn at(&mut self, idx: usize) -> Option<&T> {
        unimplemented!()
    }

    /// Removes and returns the element at idx. Returns `None` if index is out of bounds.
    pub fn pop_at(&mut self, idx: usize) -> Option<T> {
        unimplemented!()
    }

}

struct ListIterator<'a, T: 'a>(&'a mut List<T>, *const ListNode);

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        unsafe {
            if self.1.is_null() {
                return None;
            };
            let val_ptr = (*self.1).data;
            self.1 = (*self.1).next;
            Some(&*(val_ptr as *const T))
        }
    }
}

#[cfg(test)]
mod tests {
}
