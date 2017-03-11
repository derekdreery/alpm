//! A place for types related to alpm, rather than e.g. a package

use alpm_sys::*;

/// This version of libalpm's capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Caps {
    pub nls: bool,
    pub downloader: bool,
    pub signatures: bool,
}

impl From<u32> for Caps {
    fn from(f: u32) -> Caps {
        Caps {
            nls: f & ALPM_CAPABILITY_NLS != 0,
            downloader: f & ALPM_CAPABILITY_DOWNLOADER != 0,
            signatures: f & ALPM_CAPABILITY_SIGNATURES != 0,
        }
    }
}

/// The result of a download
pub enum DownloadResult {
    /// The download succeeded
    Ok,
    /// The download was not needed
    NotNeeded,
    /// The download failed
    Err,
}

