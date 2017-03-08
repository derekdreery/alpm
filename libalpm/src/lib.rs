#![feature(untagged_unions)]
#![feature(pub_restricted)]

extern crate alpm_sys;
extern crate url;
extern crate libc;
extern crate printf;
#[macro_use] extern crate lazy_static;

mod list;
mod error;
mod event;
mod package;
mod db;
mod pgp;
mod log;
mod callbacks;
mod options;
mod util;

use std::ffi::{CString, CStr};
use std::ops::Drop;
use std::path::{PathBuf};
use std::sync::Mutex;
use std::mem;
use std::collections::LinkedList;

use alpm_sys::*;
use libc::{c_void};

pub use options::{Options, RepoOptions};
pub use error::{Error, AlpmResult};
pub use log::{LogLevel, LogLevels};
pub use event::Event;
pub use package::{Package, PackageRef};
pub use db::Db;
pub use pgp::SigLevel;
use callbacks::{alpm_cb_log, alpm_cb_download, alpm_cb_totaldl, alpm_cb_fetch, alpm_cb_event};
use list::alpm_list_to_vec;

// callbacks
lazy_static! {
    static ref LOG_CB: Mutex<Option<Box<FnMut(LogLevels, String) + Send>>> = Default::default();
    static ref DOWNLOAD_CB: Mutex<Option<Box<FnMut(&str, u64, u64) + Send>>> = Default::default();
    static ref FETCH_CB: Mutex<Option<Box<FnMut(&str, &str, bool) -> DownloadResult + Send>>> = Default::default();
    static ref DLTOTAL_CB: Mutex<Option<Box<FnMut(u64) + Send>>> = Default::default();
    static ref EVENT_CB: Mutex<Option<Box<FnMut(Event) + Send>>> = Default::default();
    //static ref QUESTION_CB: Mutex<Option<Box<FnMut(LogLevels, String) + Send>>> = Default::default();
    //static ref PROGRESS_CB: Mutex<Option<Box<FnMut(LogLevels, String) + Send>>> = Default::default();
}

/// A handle on an alpm instance
///
/// Note that I have NOT checked whether the interface is threadsafe, so it's best to use only one
/// instance of Alpm at present (doing your own synchronization if you want to share between
/// threads). Also, callbacks must be stored in global state, so if they are changed for one they
/// will be changed for all.
#[derive(Debug)]
pub struct Alpm {
    handle: *const Struct_alpm_handle,
}

/// This version of libalpm's capabilities
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Caps {
    pub nls: bool,
    pub downloader: bool,
    pub signatures: bool,
}

pub enum DownloadResult {
    Ok,
    NotNeeded,
    Err
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
                let alpm = Alpm {
                    handle: handle
                };
                Ok(alpm)
            }
        }
    }

    /// Gets the current (last) error status. Most functions use this internally to get the
    /// error type to return, so there isn't much need to use this externally.
    pub fn error(&self) -> Option<Error> {
        let code = unsafe { alpm_errno(self.handle) };
        if code == 0 {
            None
        } else {
            Some(code.into())
        }
    }

    /// Set the callback called when a log message is received
    pub fn log_function<F>(&self, func: F)
        where F: FnMut(LogLevels, String) + Send + 'static
    {
        let mut cb = LOG_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        unsafe { alpm_option_set_logcb(self.handle, Some(alpm_cb_log)); }
    }

    /// Clears the log callback
    pub fn clear_log_function(&self) {
        let mut cb = LOG_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_logcb(self.handle, None); }
    }

    /// Set the callback called to report progress on downloading a file
    pub fn file_download_progress_function<F>(&self, func: F)
        where F: FnMut(&str, u64, u64) + Send + 'static
    {
        let mut cb = DOWNLOAD_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        unsafe { alpm_option_set_dlcb(self.handle, Some(alpm_cb_download)); }
    }

    /// Clears the file download progress callback
    pub fn clear_file_download_progress_function(&self) {
        let mut cb = DOWNLOAD_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_dlcb(self.handle, None); }
    }

    /// Set the callback called to report progress on total download
    pub fn total_download_progress_function<F>(&self, func: F)
        where F: FnMut(u64) + Send + 'static
    {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        unsafe { alpm_option_set_totaldlcb(self.handle, Some(alpm_cb_totaldl)); }
    }

    /// Clears the total download progress callback
    pub fn clear_total_download_progress_function(&self) {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_totaldlcb(self.handle, None); }
    }

    /// Set the callback called to download a file.
    ///
    /// Providing this function is optional and it is recommended that you don't set it (and use
    /// the built in-fetch fn). This could be useful e.g. if you are behind a complicated proxy or
    /// want to use something other than http to fetch.
    ///
    /// # Safety
    /// Note that if you supply this function, you promise that if you return DownloadResult::Ok,
    /// the requested file is correctly located in the given location.
    ///
    /// A panic in the function will cause DownloadResult::Err to be sent to the underlying
    /// libalpm (i.e. not undefined behaviour).
    ///
    /// TODO investigate whether safe to relax 'static bound
    pub unsafe fn fetch_function<F>(&self, func: F)
        where F: FnMut(&str, &str, bool) -> DownloadResult + Send + 'static
    {
        let mut cb = FETCH_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        alpm_option_set_fetchcb(self.handle, Some(alpm_cb_fetch));
    }

    /// Clears the file download callback, falling back to built-in fetch functionality.
    pub fn clear_fetch_function(&self) {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_fetchcb(self.handle, None); }
    }

    /// Sets the function called when an event occurs
    pub fn event_function<F>(&self, func: F)
        where F: FnMut(Event) + Send + 'static
    {
        let mut cb = EVENT_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        unsafe { alpm_option_set_eventcb(self.handle, Some(alpm_cb_event)); }
    }

    /// Clears the file download callback, falling back to built-in fetch functionality.
    pub fn clear_event_function(&self) {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_eventcb(self.handle, None); }
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

    /// Get the set architecture
    pub fn arch(&self) -> AlpmResult<&str> {
        unsafe {
            let arch = alpm_option_get_arch(self.handle);
            if arch.is_null() {
                Err(Error::StrNull)
            } else {
                CStr::from_ptr(arch).to_str().map_err(|e| e.into())
            }
        }
    }

    /// Fetch a remote pkg from the given URL and return its path.
    pub fn fetch_pkg(&self, url: url::Url) -> AlpmResult<PathBuf> {
        unsafe {
            let url = CString::new(url.into_string())?;
            let path = alpm_fetch_pkgurl(self.handle, url.as_ptr());
            if path.is_null() {
                Err(Error::__Unknown)
            } else {
                // copy path into rust alloc'd data struct
                let path_rust = PathBuf::from(CStr::from_ptr(path).to_str()?);
                libc::free(path as *mut c_void);
                Ok(path_rust)
            }
        }
    }

    /// Get the local database instance.
    pub fn local_db<'a>(&'a self) -> Db<'a> {
        unsafe { Db::new(alpm_get_localdb(self.handle), self) }
    }

    /// Get a list of remote databases registered.
    pub fn sync_dbs<'a>(&'a self) -> Vec<Db<'a>> {
        //use std::error::Error;
        unsafe {
            let raw_list = alpm_get_syncdbs(self.handle);
            //println!("{:?}", raw_list);
            //println!("error: {:?}", self.error().unwrap().description());
            alpm_list_to_vec(raw_list, |ptr| {
                Db::new(ptr as *const Struct_alpm_db, &self)
            })
        }
    }

    /// Register a sync db (remote db). You will need to attach servers to the db to be able to
    /// sync
    pub fn register_sync_db<'a>(&'a self, treename: &str, level: SigLevel) -> AlpmResult<Db<'a>> {
        unsafe {
            let db = alpm_register_syncdb(self.handle,
                                          (CString::new(treename)?).as_ptr(),
                                          level.into());
            if db.is_null() {
                Err(self.error().unwrap_or(Error::__Unknown))
            } else {
                Ok(Db::new(db, &self))
            }
        }
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
