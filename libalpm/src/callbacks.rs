use std::panic;
use std::ffi::CStr;

use printf::printf;
use alpm_sys::*;
use libc::{c_int, c_char, c_void, off_t};

use {LOG_CB, DOWNLOAD_CB, DLTOTAL_CB, FETCH_CB, EVENT_CB, DownloadResult};
use event::Event;

/// Function with C calling convention and required type signature to wrap our callback
pub unsafe extern "C" fn alpm_cb_log(level: alpm_loglevel_t,
                                 fmt: *const c_char,
                                 args: *const Struct_va_list) {
    let out = printf(fmt, args as *mut c_void);
    panic::catch_unwind(|| {
        let mut cb = LOG_CB.lock().unwrap();
        if let Some(ref mut cb) = *cb {
            cb(level.into(), out);
        }
    }).unwrap_or(()) // ignore all errors since we are about to cross ffi boundary
}

/** Type of download progress callbacks.
 * @param filename the name of the file being downloaded
 * @param xfered the number of transferred bytes
 * @param total the total number of bytes to transfer
 */
pub unsafe extern "C" fn alpm_cb_download(filename: *const c_char, xfered: off_t, total: off_t) {
    let filename = CStr::from_ptr(filename).to_string_lossy();
    let xfered = xfered as u64;
    let total = total as u64;
    panic::catch_unwind(|| {
        let mut cb = DOWNLOAD_CB.lock().unwrap();
        if let Some(ref mut cb) = *cb {
            cb(filename.as_ref(), xfered, total);
        }
    }).unwrap_or(()) // ignore all errors since we are about to cross ffi boundary
}

/** Type of download progress callbacks.
 * @param filename the name of the file being downloaded
 * @param xfered the number of transferred bytes
 * @param total the total number of bytes to transfer
 */
pub unsafe extern "C" fn alpm_cb_totaldl(total: off_t) {
    let total = total as u64;
    panic::catch_unwind(|| {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        if let Some(ref mut cb) = *cb {
            cb(total);
        }
    }).unwrap_or(()) // ignore all errors since we are about to cross ffi boundary
}

/** A callback for downloading files
 * @param url the URL of the file to be downloaded
 * @param localpath the directory to which the file should be downloaded
 * @param force whether to force an update, even if the file is the same
 * @return 0 on success, 1 if the file exists and is identical, -1 on
 * error.
 */
pub unsafe extern "C" fn alpm_cb_fetch(url: *const c_char,
                                       localpath: *const c_char,
                                       force: c_int) -> c_int
{
    let url = CStr::from_ptr(url).to_string_lossy();
    let localpath = CStr::from_ptr(localpath).to_string_lossy();
    let force = ! force == 0;
    panic::catch_unwind(|| {
        let mut cb = FETCH_CB.lock().unwrap();
        if let Some(ref mut cb) = *cb {
            match cb(url.as_ref(), localpath.as_ref(), force) {
                DownloadResult::Ok => 0,
                DownloadResult::NotNeeded => 1,
                DownloadResult::Err => -1,
            }
        } else {
            -1
        }
    }).unwrap_or(-1) // set error code if we have panicked
}

/** Event callback */
pub unsafe extern "C" fn alpm_cb_event(evt: *const alpm_event_t) {
    let evt = Event::new(evt);
    panic::catch_unwind(|| {
        let mut cb = EVENT_CB.lock().unwrap();
        if let Some(ref mut cb) = *cb {
            cb(evt);
        }
    }).unwrap_or(())
}
