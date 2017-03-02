#![feature(untagged_unions)]

use std::ffi::{CString, CStr};
use std::ops::Drop;

extern crate alpm_sys;
use alpm_sys::*;

mod error;
pub use error::{Error, AlpmResult};

mod log;
pub use log::{LogLevel};
use log::{RustLogFn};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

/// A handle on an alpm instance
pub struct Alpm {
    handle: *mut Struct_alpm_handle,
}

/// This version of libalpm's capabilities
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Caps {
    pub nls: bool,
    pub downloader: bool,
    pub signatures: bool,
}

impl Alpm {
    /// Get a handle on the alpm instance defined by the given root/db_path
    pub fn new(root: &str, db_path: &str) -> AlpmResult<Alpm> {
        // Requires alloc, but str is more standard
        let root = CString::new(root)?;
        let db_path = CString::new(db_path)?;
        unsafe {
            let mut err: alpm_errno_t = 0;
            let handle = alpm_initialize(root.as_ptr(), db_path.as_ptr(), &mut err);
            if err != 0 {
                Err(Error::from(err))
            } else {
                Ok(Alpm {
                    handle: handle,
                })
            }
        }
    }

    /// Get the root path used in this instance of alpm
    ///
    /// The api doesn't make clear the lifetime of the result, so I am conservative (same goes for
    /// db_path)
    pub fn root<'a>(&'a self) -> &'a str {
        let root = unsafe { CStr::from_ptr(alpm_option_get_root(self.handle)) };
        root.to_str().ok().expect("instance root path is not utf8")
    }

    /// Get the database path used in this instance of alpm
    pub fn db_path<'a>(&'a self) -> &'a str {
        let db_path = unsafe { CStr::from_ptr(alpm_option_get_dbpath(self.handle)) };
        db_path.to_str().ok().expect("instance db path is not utf8")
    }

    /// Get the lockfile path used in this instance of alpm
    pub fn lockfile<'a>(&'a self) -> &'a str {
        let lockfile = unsafe { CStr::from_ptr(alpm_option_get_lockfile(self.handle)) };
        lockfile.to_str().ok().expect("instance lockfile path is not utf8")
    }

    /// Set the logging callback
    pub fn set_log_callback<F>(f: F) -> AlpmResult<()>
        where F: RustLogFn
    {
        Ok(())
    }
}

impl Drop for Alpm {
    fn drop(&mut self) {
        unsafe { alpm_release(self.handle); }
    }
}

/// Get the version of the attached libalpm
pub fn version() -> &'static str {
    unsafe {
        let v = CStr::from_ptr(alpm_version());
        v.to_str().ok().expect("For some reason the libalpm version is not utf8")
    }
}

/// Get the capabilities of the attached libalpm
pub fn capabilities() -> Caps {
    // could do this faster if used bitfields for Caps (no branch)
    let caps = unsafe { alpm_capabilities() };
    Caps {
        nls: caps & ALPM_CAPABILITY_NLS != 0,
        downloader: caps & ALPM_CAPABILITY_DOWNLOADER != 0,
        signatures: caps & ALPM_CAPABILITY_SIGNATURES != 0,
    }
}
