//! The ubiquitous error type for libalpm

use std::error::Error as StdError;
use std::fmt;
use std::ffi;
use std::str;

/// An enum of possible errors in libalpm
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// out of memory!
    Memory,
    /// unexpected system error
    System,
    /// permission denied
    BadPerms,
    /// could not find or read file
    NotAFile,
    /// could not find or read directory
    NotADir,
    /// wrong or NULL argument passed
    WrongArgs,
    /// not enough free disk space
    DiskSpace,
    /// library not initialized
    HandleNull,
    /// library already initialized
    HandleNotNull,
    /// unable to lock database
    HandleLock,
    /// could not open database
    DbOpen,
    /// could not create database
    DbCreate,
    /// database not initialized
    DbNull,
    /// database already registered
    DbNotNull,
    /// could not find database
    DbNotFound,
    /// invalid or corrupted database
    DbInvalid,
    /// invalid or corrupted database (PGP signature)
    DbInvalidSig,
    /// database is incorrect version
    DbVersion,
    /// could not update database
    DbWrite,
    /// could not remove database entry
    DbRemove,
    /// invalid url for server
    ServerBadUrl,
    /// no servers configured for repository
    ServerNone,
    /// transaction already initialized
    TransNotNull,
    /// transaction not initialized
    TransNull,
    /// duplicate target
    TransDupTarget,
    /// transaction not initialized
    TransNotInitialized,
    /// transaction not prepared
    TransNotPrepared,
    /// transaction aborted
    TransAbort,
    /// operation not compatible with the transaction type
    TransType,
    /// transaction commit attempt when database is not locked
    TransNotLocked,
    /// failed to run transaction hooks
    TransHookFailed,
    /// could not find or read package
    PkgNotFound,
    /// operation cancelled due to ignorepkg
    PkgIgnored,
    /// invalid or corrupted package
    PkgInvalid,
    /// invalid or corrupted package (checksum)
    PkgInvalidChecksum,
    /// invalid or corrupted package (PGP signature)
    PkgInvalidSig,
    /// package missing required signature
    PkgMissingSig,
    /// cannot open package file
    PkgOpen,
    /// cannot remove all files for package
    PkgCantRemove,
    /// package filename is not valid
    PkgInvalidName,
    /// package architecture is not valid
    PkgInvalidArch,
    /// could not find repository for target
    PkgRepoNotFound,
    /// missing PGP signature
    SigMissing,
    /// invalid PGP signature
    SigInvalid,
    /// invalid or corrupted delta
    DltInvalid,
    /// delta patch failed
    DltPatchFailed,
    /// could not satisfy dependencies
    UnsatisfiedDeps,
    /// conflicting dependencies
    ConflictingDeps,
    /// conflicting files
    FileConflicts,
    /// failed to retrieve some files
    Retrieve,
    /// invalid regular expression
    InvalidRegex,
    /// libarchive error
    Libarchive,
    /// download library error
    Libcurl,
    /// gpgme error
    Gpgme,
    /// error invoking external downloader
    ExternalDownload,

    // non-alpm
    /// io error
    IO,
    /// null string error
    StrNull,
    /// utf8 decode error
    Utf8Error,
    /// unknown error
    __Unknown,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Memory => "out of memory!",
            Error::System => "unexpected system error",
            Error::BadPerms => "permission denied",
            Error::NotAFile => "could not find or read file",
            Error::NotADir => "could not find or read directory",
            Error::WrongArgs => "wrong or NULL argument passed",
            Error::DiskSpace => "not enough free disk space",
            Error::HandleNull => "library not initialized",
            Error::HandleNotNull => "library already initialized",
            Error::HandleLock => "unable to lock database",
            Error::DbOpen => "could not open database",
            Error::DbCreate => "could not create database",
            Error::DbNull => "database not initialized",
            Error::DbNotNull => "database already registered",
            Error::DbNotFound => "could not find database",
            Error::DbInvalid => "invalid or corrupted database",
            Error::DbInvalidSig => "invalid or corrupted database (PGP signature)",
            Error::DbVersion => "database is incorrect version",
            Error::DbWrite => "could not update database",
            Error::DbRemove => "could not remove database entry",
            Error::ServerBadUrl => "invalid url for server",
            Error::ServerNone => "no servers configured for repository",
            Error::TransNotNull => "transaction already initialized",
            Error::TransNull => "transaction not initialized",
            Error::TransDupTarget => "duplicate target",
            Error::TransNotInitialized => "transaction not initialized",
            Error::TransNotPrepared => "transaction not prepared",
            Error::TransAbort => "transaction aborted",
            Error::TransType => "operation not compatible with the transaction type",
            Error::TransNotLocked => "transaction commit attempt when database is not locked",
            Error::TransHookFailed => "failed to run transaction hooks",
            Error::PkgNotFound => "could not find or read package",
            Error::PkgIgnored => "operation cancelled due to ignorepkg",
            Error::PkgInvalid => "invalid or corrupted package",
            Error::PkgInvalidChecksum => "invalid or corrupted package (checksum)",
            Error::PkgInvalidSig => "invalid or corrupted package (PGP signature)",
            Error::PkgMissingSig => "package missing required signature",
            Error::PkgOpen => "cannot open package file",
            Error::PkgCantRemove => "cannot remove all files for package",
            Error::PkgInvalidName => "package filename is not valid",
            Error::PkgInvalidArch => "package architecture is not valid",
            Error::PkgRepoNotFound => "could not find repository for target",
            Error::SigMissing => "missing PGP signature",
            Error::SigInvalid => "invalid PGP signature",
            Error::DltInvalid => "invalid or corrupted delta",
            Error::DltPatchFailed => "delta patch failed",
            Error::UnsatisfiedDeps => "could not satisfy dependencies",
            Error::ConflictingDeps => "conflicting dependencies",
            Error::FileConflicts => "conflicting files",
            Error::Retrieve => "failed to retrieve some files",
            Error::InvalidRegex => "invalid regular expression",
            Error::Libarchive => "libarchive error",
            Error::Libcurl => "download library error",
            Error::Gpgme => "gpgme error",
            Error::ExternalDownload => "error invoking external downloader",
            Error::IO => "io error",
            Error::StrNull => "null string error",
            Error::Utf8Error => "utf8 decode error",
            Error::__Unknown => "unknown error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<u32> for Error {
    fn from(from: u32) -> Error {
        use alpm_sys::alpm_errno_t::*;
        match from {
            x if x == ALPM_ERR_MEMORY as u32 => Error::Memory,
            x if x == ALPM_ERR_SYSTEM as u32 => Error::System,
            x if x == ALPM_ERR_BADPERMS as u32 => Error::BadPerms,
            x if x == ALPM_ERR_NOT_A_FILE as u32 => Error::NotAFile,
            x if x == ALPM_ERR_NOT_A_DIR as u32 => Error::NotADir,
            x if x == ALPM_ERR_WRONG_ARGS as u32 => Error::WrongArgs,
            x if x == ALPM_ERR_DISK_SPACE as u32 => Error::DiskSpace,
            /* Interface */
            x if x == ALPM_ERR_HANDLE_NULL as u32 => Error::HandleNull,
            x if x == ALPM_ERR_HANDLE_NOT_NULL as u32 => Error::HandleNotNull,
            x if x == ALPM_ERR_HANDLE_LOCK as u32 => Error::HandleLock,
            /* Databases */
            x if x == ALPM_ERR_DB_OPEN as u32 => Error::DbOpen,
            x if x == ALPM_ERR_DB_CREATE as u32 => Error::DbCreate,
            x if x == ALPM_ERR_DB_NULL as u32 => Error::DbNull,
            x if x == ALPM_ERR_DB_NOT_NULL as u32 => Error::DbNotNull,
            x if x == ALPM_ERR_DB_NOT_FOUND as u32 => Error::DbNotFound,
            x if x == ALPM_ERR_DB_INVALID as u32 => Error::DbInvalid,
            x if x == ALPM_ERR_DB_INVALID_SIG as u32 => Error::DbInvalidSig,
            x if x == ALPM_ERR_DB_VERSION as u32 => Error::DbVersion,
            x if x == ALPM_ERR_DB_WRITE as u32 => Error::DbWrite,
            x if x == ALPM_ERR_DB_REMOVE as u32 => Error::DbRemove,
            /* Servers */
            x if x == ALPM_ERR_SERVER_BAD_URL as u32 => Error::ServerBadUrl,
            x if x == ALPM_ERR_SERVER_NONE as u32 => Error::ServerNone,
            /* Transactions */
            x if x == ALPM_ERR_TRANS_NOT_NULL as u32 => Error::TransNotNull,
            x if x == ALPM_ERR_TRANS_NULL as u32 => Error::TransNull,
            x if x == ALPM_ERR_TRANS_DUP_TARGET as u32 => Error::TransDupTarget,
            x if x == ALPM_ERR_TRANS_NOT_INITIALIZED as u32 => Error::TransNotInitialized,
            x if x == ALPM_ERR_TRANS_NOT_PREPARED as u32 => Error::TransNotPrepared,
            x if x == ALPM_ERR_TRANS_ABORT as u32 => Error::TransAbort,
            x if x == ALPM_ERR_TRANS_TYPE as u32 => Error::TransType,
            x if x == ALPM_ERR_TRANS_NOT_LOCKED as u32 => Error::TransNotLocked,
            x if x == ALPM_ERR_TRANS_HOOK_FAILED as u32 => Error::TransHookFailed,
            /* Packages */
            x if x == ALPM_ERR_PKG_NOT_FOUND as u32 => Error::PkgNotFound,
            x if x == ALPM_ERR_PKG_IGNORED as u32 => Error::PkgIgnored,
            x if x == ALPM_ERR_PKG_INVALID as u32 => Error::PkgInvalid,
            x if x == ALPM_ERR_PKG_INVALID_CHECKSUM as u32 => Error::PkgInvalidChecksum,
            x if x == ALPM_ERR_PKG_INVALID_SIG as u32 => Error::PkgInvalidSig,
            x if x == ALPM_ERR_PKG_MISSING_SIG as u32 => Error::PkgMissingSig,
            x if x == ALPM_ERR_PKG_OPEN as u32 => Error::PkgOpen,
            x if x == ALPM_ERR_PKG_CANT_REMOVE as u32 => Error::PkgCantRemove,
            x if x == ALPM_ERR_PKG_INVALID_NAME as u32 => Error::PkgInvalidName,
            x if x == ALPM_ERR_PKG_INVALID_ARCH as u32 => Error::PkgInvalidArch,
            x if x == ALPM_ERR_PKG_REPO_NOT_FOUND as u32 => Error::PkgRepoNotFound,
            /* Signatures */
            x if x == ALPM_ERR_SIG_MISSING as u32 => Error::SigMissing,
            x if x == ALPM_ERR_SIG_INVALID as u32 => Error::SigInvalid,
            /* Deltas */
            x if x == ALPM_ERR_DLT_INVALID as u32 => Error::DltInvalid,
            x if x == ALPM_ERR_DLT_PATCHFAILED as u32 => Error::DltPatchFailed,
            /* Dependencies */
            x if x == ALPM_ERR_UNSATISFIED_DEPS as u32 => Error::UnsatisfiedDeps,
            x if x == ALPM_ERR_CONFLICTING_DEPS as u32 => Error::ConflictingDeps,
            x if x == ALPM_ERR_FILE_CONFLICTS as u32 => Error::FileConflicts,
            /* Misc */
            x if x == ALPM_ERR_RETRIEVE as u32 => Error::Retrieve,
            x if x == ALPM_ERR_INVALID_REGEX as u32 => Error::InvalidRegex,
            /* External library errors */
            x if x == ALPM_ERR_LIBARCHIVE as u32 => Error::Libarchive,
            x if x == ALPM_ERR_LIBCURL as u32 => Error::Libcurl,
            x if x == ALPM_ERR_EXTERNAL_DOWNLOAD as u32 => Error::ExternalDownload,
            x if x == ALPM_ERR_GPGME as u32 => Error::Gpgme,
            _ => Error::__Unknown,
        }
    }
}

impl From<ffi::NulError> for Error {
    fn from(_: ffi::NulError) -> Error {
        Error::StrNull
    }
}

impl From<str::Utf8Error> for Error {
    fn from(_: str::Utf8Error) -> Error {
        Error::Utf8Error
    }
}

/// The ubiquitous crate result type.
pub type AlpmResult<T> = Result<T, Error>;

#[cfg(test)]
mod test {
    use super::*;
    extern crate alpm_sys;

    #[test]
    fn from_u32() {
        let err = alpm_sys::alpm_errno_t::ALPM_ERR_MEMORY as u32;
        assert_eq!(Error::Memory, err.into());
    }
}
