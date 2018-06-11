//! A place for types related to alpm, rather than e.g. a package

use alpm_sys::*;

/// This version of libalpm's capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Caps {
    pub nls: bool,
    pub downloader: bool,
    pub signatures: bool,
}

impl From<alpm_caps> for Caps {
    fn from(f: alpm_caps) -> Caps {
        Caps::from(f as u32)
    }
}

impl From<u32> for Caps {
    fn from(f: u32) -> Caps {
        use alpm_caps::*;
        Caps {
            nls: f & ALPM_CAPABILITY_NLS as u32 != 0,
            downloader: f & ALPM_CAPABILITY_DOWNLOADER as u32 != 0,
            signatures: f & ALPM_CAPABILITY_SIGNATURES as u32 != 0,
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

