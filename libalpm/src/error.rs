//! The ubiquitous error type for libalpm

use std::error::Error as StdError;
use std::fmt;
use std::ffi;
use std::str;

use alpm_sys::*;

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
        match from {
            ALPM_ERR_MEMORY => Error::Memory,
            ALPM_ERR_SYSTEM => Error::System,
            ALPM_ERR_BADPERMS => Error::BadPerms,
            ALPM_ERR_NOT_A_FILE => Error::NotAFile,
            ALPM_ERR_NOT_A_DIR => Error::NotADir,
            ALPM_ERR_WRONG_ARGS => Error::WrongArgs,
            ALPM_ERR_DISK_SPACE => Error::DiskSpace,
            /* Interface */
            ALPM_ERR_HANDLE_NULL => Error::HandleNull,
            ALPM_ERR_HANDLE_NOT_NULL => Error::HandleNotNull,
            ALPM_ERR_HANDLE_LOCK => Error::HandleLock,
            /* Databases */
            ALPM_ERR_DB_OPEN => Error::DbOpen,
            ALPM_ERR_DB_CREATE => Error::DbCreate,
            ALPM_ERR_DB_NULL => Error::DbNull,
            ALPM_ERR_DB_NOT_NULL => Error::DbNotNull,
            ALPM_ERR_DB_NOT_FOUND => Error::DbNotFound,
            ALPM_ERR_DB_INVALID => Error::DbInvalid,
            ALPM_ERR_DB_INVALID_SIG => Error::DbInvalidSig,
            ALPM_ERR_DB_VERSION => Error::DbVersion,
            ALPM_ERR_DB_WRITE => Error::DbWrite,
            ALPM_ERR_DB_REMOVE => Error::DbRemove,
            /* Servers */
            ALPM_ERR_SERVER_BAD_URL => Error::ServerBadUrl,
            ALPM_ERR_SERVER_NONE => Error::ServerNone,
            /* Transactions */
            ALPM_ERR_TRANS_NOT_NULL => Error::TransNotNull,
            ALPM_ERR_TRANS_NULL => Error::TransNull,
            ALPM_ERR_TRANS_DUP_TARGET => Error::TransDupTarget,
            ALPM_ERR_TRANS_NOT_INITIALIZED => Error::TransNotInitialized,
            ALPM_ERR_TRANS_NOT_PREPARED => Error::TransNotPrepared,
            ALPM_ERR_TRANS_ABORT => Error::TransAbort,
            ALPM_ERR_TRANS_TYPE => Error::TransType,
            ALPM_ERR_TRANS_NOT_LOCKED => Error::TransNotLocked,
            ALPM_ERR_TRANS_HOOK_FAILED => Error::TransHookFailed,
            /* Packages */
            ALPM_ERR_PKG_NOT_FOUND => Error::PkgNotFound,
            ALPM_ERR_PKG_IGNORED => Error::PkgIgnored,
            ALPM_ERR_PKG_INVALID => Error::PkgInvalid,
            ALPM_ERR_PKG_INVALID_CHECKSUM => Error::PkgInvalidChecksum,
            ALPM_ERR_PKG_INVALID_SIG => Error::PkgInvalidSig,
            ALPM_ERR_PKG_MISSING_SIG => Error::PkgMissingSig,
            ALPM_ERR_PKG_OPEN => Error::PkgOpen,
            ALPM_ERR_PKG_CANT_REMOVE => Error::PkgCantRemove,
            ALPM_ERR_PKG_INVALID_NAME => Error::PkgInvalidName,
            ALPM_ERR_PKG_INVALID_ARCH => Error::PkgInvalidArch,
            ALPM_ERR_PKG_REPO_NOT_FOUND => Error::PkgRepoNotFound,
            /* Signatures */
            ALPM_ERR_SIG_MISSING => Error::SigMissing,
            ALPM_ERR_SIG_INVALID => Error::SigInvalid,
            /* Deltas */
            ALPM_ERR_DLT_INVALID => Error::DltInvalid,
            ALPM_ERR_DLT_PATCHFAILED => Error::DltPatchFailed,
            /* Dependencies */
            ALPM_ERR_UNSATISFIED_DEPS => Error::UnsatisfiedDeps,
            ALPM_ERR_CONFLICTING_DEPS => Error::ConflictingDeps,
            ALPM_ERR_FILE_CONFLICTS => Error::FileConflicts,
            /* Misc */
            ALPM_ERR_RETRIEVE => Error::Retrieve,
            ALPM_ERR_INVALID_REGEX => Error::InvalidRegex,
            /* External library errors */
            ALPM_ERR_LIBARCHIVE => Error::Libarchive,
            ALPM_ERR_LIBCURL => Error::Libcurl,
            ALPM_ERR_EXTERNAL_DOWNLOAD => Error::ExternalDownload,
            ALPM_ERR_GPGME => Error::Gpgme,
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

    #[test]
    fn from_u32() {
        let err = ALPM_ERR_MEMORY;
        assert_eq!(Error::Memory, err.into());
    }
}
