
use libc::*;
pub use libarchive3_sys::ffi::{Struct_archive, Struct_archive_entry};

// Constants
// alpm_errno_t
pub const ALPM_ERR_MEMORY: alpm_errno_t = 1;
pub const ALPM_ERR_SYSTEM: alpm_errno_t = 2;
pub const ALPM_ERR_BADPERMS: alpm_errno_t = 3;
pub const ALPM_ERR_NOT_A_FILE: alpm_errno_t = 4;
pub const ALPM_ERR_NOT_A_DIR: alpm_errno_t = 5;
pub const ALPM_ERR_WRONG_ARGS: alpm_errno_t = 6;
pub const ALPM_ERR_DISK_SPACE: alpm_errno_t = 7;
pub const ALPM_ERR_HANDLE_NULL: alpm_errno_t = 8;
pub const ALPM_ERR_HANDLE_NOT_NULL: alpm_errno_t = 9;
pub const ALPM_ERR_HANDLE_LOCK: alpm_errno_t = 10;
pub const ALPM_ERR_DB_OPEN: alpm_errno_t = 11;
pub const ALPM_ERR_DB_CREATE: alpm_errno_t = 12;
pub const ALPM_ERR_DB_NULL: alpm_errno_t = 13;
pub const ALPM_ERR_DB_NOT_NULL: alpm_errno_t = 14;
pub const ALPM_ERR_DB_NOT_FOUND: alpm_errno_t = 15;
pub const ALPM_ERR_DB_INVALID: alpm_errno_t = 16;
pub const ALPM_ERR_DB_INVALID_SIG: alpm_errno_t = 17;
pub const ALPM_ERR_DB_VERSION: alpm_errno_t = 18;
pub const ALPM_ERR_DB_WRITE: alpm_errno_t = 19;
pub const ALPM_ERR_DB_REMOVE: alpm_errno_t = 20;
pub const ALPM_ERR_SERVER_BAD_URL: alpm_errno_t = 21;
pub const ALPM_ERR_SERVER_NONE: alpm_errno_t = 22;
pub const ALPM_ERR_TRANS_NOT_NULL: alpm_errno_t = 23;
pub const ALPM_ERR_TRANS_NULL: alpm_errno_t = 24;
pub const ALPM_ERR_TRANS_DUP_TARGET: alpm_errno_t = 25;
pub const ALPM_ERR_TRANS_NOT_INITIALIZED: alpm_errno_t = 26;
pub const ALPM_ERR_TRANS_NOT_PREPARED: alpm_errno_t = 27;
pub const ALPM_ERR_TRANS_ABORT: alpm_errno_t = 28;
pub const ALPM_ERR_TRANS_TYPE: alpm_errno_t = 29;
pub const ALPM_ERR_TRANS_NOT_LOCKED: alpm_errno_t = 30;
pub const ALPM_ERR_TRANS_HOOK_FAILED: alpm_errno_t = 31;
pub const ALPM_ERR_PKG_NOT_FOUND: alpm_errno_t = 32;
pub const ALPM_ERR_PKG_IGNORED: alpm_errno_t = 33;
pub const ALPM_ERR_PKG_INVALID: alpm_errno_t = 34;
pub const ALPM_ERR_PKG_INVALID_CHECKSUM: alpm_errno_t = 35;
pub const ALPM_ERR_PKG_INVALID_SIG: alpm_errno_t = 36;
pub const ALPM_ERR_PKG_MISSING_SIG: alpm_errno_t = 37;
pub const ALPM_ERR_PKG_OPEN: alpm_errno_t = 38;
pub const ALPM_ERR_PKG_CANT_REMOVE: alpm_errno_t = 39;
pub const ALPM_ERR_PKG_INVALID_NAME: alpm_errno_t = 40;
pub const ALPM_ERR_PKG_INVALID_ARCH: alpm_errno_t = 41;
pub const ALPM_ERR_PKG_REPO_NOT_FOUND: alpm_errno_t = 42;
pub const ALPM_ERR_SIG_MISSING: alpm_errno_t = 43;
pub const ALPM_ERR_SIG_INVALID: alpm_errno_t = 44;
pub const ALPM_ERR_DLT_INVALID: alpm_errno_t = 45;
pub const ALPM_ERR_DLT_PATCHFAILED: alpm_errno_t = 46;
pub const ALPM_ERR_UNSATISFIED_DEPS: alpm_errno_t = 47;
pub const ALPM_ERR_CONFLICTING_DEPS: alpm_errno_t = 48;
pub const ALPM_ERR_FILE_CONFLICTS: alpm_errno_t = 49;
pub const ALPM_ERR_RETRIEVE: alpm_errno_t = 50;
pub const ALPM_ERR_INVALID_REGEX: alpm_errno_t = 51;
pub const ALPM_ERR_LIBARCHIVE: alpm_errno_t = 52;
pub const ALPM_ERR_LIBCURL: alpm_errno_t = 53;
pub const ALPM_ERR_EXTERNAL_DOWNLOAD: alpm_errno_t = 54;
pub const ALPM_ERR_GPGME: alpm_errno_t = 55;
// alpm_pkgreason_t
pub const ALPM_PKG_REASON_EXPLICIT: alpm_pkgreason_t = 0;
pub const ALPM_PKG_REASON_DEPEND: alpm_pkgreason_t = 1;
// alpm_pkgfrom_t
pub const ALPM_PKG_FROM_FILE: alpm_pkgfrom_t = 1;
pub const ALPM_PKG_FROM_LOCALDB: alpm_pkgfrom_t = 2;
pub const ALPM_PKG_FROM_SYNCDB: alpm_pkgfrom_t = 3;
// alpm_pkgvalidation_t
pub const ALPM_PKG_VALIDATION_UNKNOWN: alpm_pkgvalidation_t = 0;
pub const ALPM_PKG_VALIDATION_NONE: alpm_pkgvalidation_t = 1 << 0;
pub const ALPM_PKG_VALIDATION_MD5SUM: alpm_pkgvalidation_t = 1 << 1;
pub const ALPM_PKG_VALIDATION_SHA256SUM: alpm_pkgvalidation_t = 1 << 2;
pub const ALPM_PKG_VALIDATION_SIGNATURE: alpm_pkgvalidation_t = 1 << 3;
// alpm_depmod_t
pub const ALPM_DEP_MOD_ANY: alpm_depmod_t = 1;
pub const ALPM_DEP_MOD_EQ: alpm_depmod_t = 2;
pub const ALPM_DEP_MOD_GE: alpm_depmod_t = 3;
pub const ALPM_DEP_MOD_LE: alpm_depmod_t = 4;
pub const ALPM_DEP_MOD_GT: alpm_depmod_t = 5;
pub const ALPM_DEP_MOD_LT: alpm_depmod_t = 6;
// alpm_siglevel_t
pub const ALPM_SIG_PACKAGE: alpm_siglevel_t = 1 << 0;
pub const ALPM_SIG_PACKAGE_OPTIONAL: alpm_siglevel_t = 1 << 1;
pub const ALPM_SIG_PACKAGE_MARGINAL_OK: alpm_siglevel_t = 1 << 2;
pub const ALPM_SIG_PACKAGE_UNKNOWN_OK: alpm_siglevel_t = 1 << 3;
pub const ALPM_SIG_DATABASE: alpm_siglevel_t = 1 << 10;
pub const ALPM_SIG_DATABASE_OPTIONAL: alpm_siglevel_t = 1 << 11;
pub const ALPM_SIG_DATABASE_MARGINAL_OK: alpm_siglevel_t = 1 << 12;
pub const ALPM_SIG_DATABASE_UNKNOWN_OK: alpm_siglevel_t = 1 << 13;
pub const ALPM_SIG_USE_DEFAULT: alpm_siglevel_t = 1 << 31;
// alpm_fileconflicttype_t
pub const ALPM_FILECONFLICT_TARGET: alpm_fileconflicttype_t = 1;
pub const ALPM_FILECONFLICT_FILESYSTEM: alpm_fileconflicttype_t = 2;
// alpm_sigstatus_t
pub const ALPM_SIGSTATUS_VALID: alpm_sigstatus_t = 0;
pub const ALPM_SIGSTATUS_KEY_EXPIRED: alpm_sigstatus_t = 1;
pub const ALPM_SIGSTATUS_SIG_EXPIRED: alpm_sigstatus_t = 2;
pub const ALPM_SIGSTATUS_KEY_UNKNOWN: alpm_sigstatus_t = 3;
pub const ALPM_SIGSTATUS_KEY_DISABLED: alpm_sigstatus_t = 4;
pub const ALPM_SIGSTATUS_INVALID: alpm_sigstatus_t = 5;
// alpm_sigvalidity_t
pub const ALPM_SIGVALIDITY_FULL: alpm_sigvalidity_t = 0;
pub const ALPM_SIGVALIDITY_MARGINAL: alpm_sigvalidity_t = 1;
pub const ALPM_SIGVALIDITY_NEVER: alpm_sigvalidity_t = 2;
pub const ALPM_SIGVALIDITY_UNKNOWN: alpm_sigvalidity_t = 3;
// alpm_hook_when_t
pub const ALPM_HOOK_PRE_TRANSACTION: alpm_hook_when_t = 1;
pub const ALPM_HOOK_POST_TRANSACTION: alpm_hook_when_t = 2;
// alpm_loglevel_t
pub const ALPM_LOG_ERROR: alpm_loglevel_t = 1 << 0;
pub const ALPM_LOG_WARNING: alpm_loglevel_t = 1 << 1;
pub const ALPM_LOG_DEBUG: alpm_loglevel_t = 1 << 2;
pub const ALPM_LOG_FUNCTION: alpm_loglevel_t = 1 << 3;
// alpm_event_type_t {
pub const ALPM_EVENT_CHECKDEPS_START: alpm_event_type_t = 1;
pub const ALPM_EVENT_CHECKDEPS_DONE: alpm_event_type_t = 2;
pub const ALPM_EVENT_FILECONFLICTS_START: alpm_event_type_t = 3;
pub const ALPM_EVENT_FILECONFLICTS_DONE: alpm_event_type_t = 4;
pub const ALPM_EVENT_RESOLVEDEPS_START: alpm_event_type_t = 5;
pub const ALPM_EVENT_RESOLVEDEPS_DONE: alpm_event_type_t = 6;
pub const ALPM_EVENT_INTERCONFLICTS_START: alpm_event_type_t = 7;
pub const ALPM_EVENT_INTERCONFLICTS_DONE: alpm_event_type_t = 8;
pub const ALPM_EVENT_TRANSACTION_START: alpm_event_type_t = 9;
pub const ALPM_EVENT_TRANSACTION_DONE: alpm_event_type_t = 10;
pub const ALPM_EVENT_PACKAGE_OPERATION_START: alpm_event_type_t = 11;
pub const ALPM_EVENT_PACKAGE_OPERATION_DONE: alpm_event_type_t = 12;
pub const ALPM_EVENT_INTEGRITY_START: alpm_event_type_t = 13;
pub const ALPM_EVENT_INTEGRITY_DONE: alpm_event_type_t = 14;
pub const ALPM_EVENT_LOAD_START: alpm_event_type_t = 15;
pub const ALPM_EVENT_LOAD_DONE: alpm_event_type_t = 16;
pub const ALPM_EVENT_DELTA_INTEGRITY_START: alpm_event_type_t = 17;
pub const ALPM_EVENT_DELTA_INTEGRITY_DONE: alpm_event_type_t = 18;
pub const ALPM_EVENT_DELTA_PATCHES_START: alpm_event_type_t = 19;
pub const ALPM_EVENT_DELTA_PATCHES_DONE: alpm_event_type_t = 20;
pub const ALPM_EVENT_DELTA_PATCH_START: alpm_event_type_t = 21;
pub const ALPM_EVENT_DELTA_PATCH_DONE: alpm_event_type_t = 22;
pub const ALPM_EVENT_DELTA_PATCH_FAILED: alpm_event_type_t = 23;
pub const ALPM_EVENT_SCRIPTLET_INFO: alpm_event_type_t = 24;
pub const ALPM_EVENT_RETRIEVE_START: alpm_event_type_t = 25;
pub const ALPM_EVENT_RETRIEVE_DONE: alpm_event_type_t = 26;
pub const ALPM_EVENT_RETRIEVE_FAILED: alpm_event_type_t = 27;
pub const ALPM_EVENT_PKGDOWNLOAD_START: alpm_event_type_t = 28;
pub const ALPM_EVENT_PKGDOWNLOAD_DONE: alpm_event_type_t = 29;
pub const ALPM_EVENT_PKGDOWNLOAD_FAILED: alpm_event_type_t = 30;
pub const ALPM_EVENT_DISKSPACE_START: alpm_event_type_t = 31;
pub const ALPM_EVENT_DISKSPACE_DONE: alpm_event_type_t = 32;
pub const ALPM_EVENT_OPTDEP_REMOVAL: alpm_event_type_t = 33;
pub const ALPM_EVENT_DATABASE_MISSING: alpm_event_type_t = 34;
pub const ALPM_EVENT_KEYRING_START: alpm_event_type_t = 35;
pub const ALPM_EVENT_KEYRING_DONE: alpm_event_type_t = 36;
pub const ALPM_EVENT_KEY_DOWNLOAD_START: alpm_event_type_t = 37;
pub const ALPM_EVENT_KEY_DOWNLOAD_DONE: alpm_event_type_t = 38;
pub const ALPM_EVENT_PACNEW_CREATED: alpm_event_type_t = 39;
pub const ALPM_EVENT_PACSAVE_CREATED: alpm_event_type_t = 40;
pub const ALPM_EVENT_HOOK_START: alpm_event_type_t = 41;
pub const ALPM_EVENT_HOOK_DONE: alpm_event_type_t = 42;
pub const ALPM_EVENT_HOOK_RUN_START: alpm_event_type_t = 43;
pub const ALPM_EVENT_HOOK_RUN_DONE: alpm_event_type_t = 44;
// alpm_package_operation_t
pub const ALPM_PACKAGE_INSTALL: alpm_package_operation_t = 1;
pub const ALPM_PACKAGE_UPGRADE: alpm_package_operation_t = 2;
pub const ALPM_PACKAGE_REINSTALL: alpm_package_operation_t = 3;
pub const ALPM_PACKAGE_DOWNGRADE: alpm_package_operation_t = 4;
pub const ALPM_PACKAGE_REMOVE: alpm_package_operation_t = 5;
// alpm_question_type_t
pub const ALPM_QUESTION_INSTALL_IGNOREPKG: alpm_question_type_t = 1;
pub const ALPM_QUESTION_REPLACE_PKG: alpm_question_type_t = 2;
pub const ALPM_QUESTION_CONFLICT_PKG: alpm_question_type_t = 4;
pub const ALPM_QUESTION_CORRUPTED_PKG: alpm_question_type_t = 8;
pub const ALPM_QUESTION_REMOVE_PKGS: alpm_question_type_t = 16;
pub const ALPM_QUESTION_SELECT_PROVIDER: alpm_question_type_t = 32;
pub const ALPM_QUESTION_IMPORT_KEY: alpm_question_type_t = 64;
// alpm_progress_t
pub const ALPM_PROGRESS_ADD_START: alpm_progress_t = 0;
pub const ALPM_PROGRESS_UPGRADE_START: alpm_progress_t = 1;
pub const ALPM_PROGRESS_DOWNGRADE_START: alpm_progress_t = 2;
pub const ALPM_PROGRESS_REINSTALL_START: alpm_progress_t = 3;
pub const ALPM_PROGRESS_REMOVE_START: alpm_progress_t = 4;
pub const ALPM_PROGRESS_CONFLICTS_START: alpm_progress_t = 5;
pub const ALPM_PROGRESS_DISKSPACE_START: alpm_progress_t = 6;
pub const ALPM_PROGRESS_INTEGRITY_START: alpm_progress_t = 7;
pub const ALPM_PROGRESS_LOAD_START: alpm_progress_t = 8;
pub const ALPM_PROGRESS_KEYRING_START: alpm_progress_t = 9;
// alpm_db_usage_t;
pub const ALPM_DB_USAGE_SYNC: alpm_db_usage_t = 1 << 0;
pub const ALPM_DB_USAGE_SEARCH: alpm_db_usage_t = 1 << 1;
pub const ALPM_DB_USAGE_INSTALL: alpm_db_usage_t = 1 << 2;
pub const ALPM_DB_USAGE_UPGRADE: alpm_db_usage_t = 1 << 3;
pub const ALPM_DB_USAGE_ALL: alpm_db_usage_t = (1 << 4) - 1;
// alpm_transflag_t
pub const ALPM_TRANS_FLAG_NODEPS: alpm_transflag_t = 1 << 0;
pub const ALPM_TRANS_FLAG_FORCE: alpm_transflag_t = 1 << 1;
pub const ALPM_TRANS_FLAG_NOSAVE: alpm_transflag_t = 1 << 2;
pub const ALPM_TRANS_FLAG_NODEPVERSION: alpm_transflag_t = 1 << 3;
pub const ALPM_TRANS_FLAG_CASCADE: alpm_transflag_t = 1 << 4;
pub const ALPM_TRANS_FLAG_RECURSE: alpm_transflag_t = 1 << 5;
pub const ALPM_TRANS_FLAG_DBONLY: alpm_transflag_t = 1 << 6;
pub const ALPM_TRANS_FLAG_ALLDEPS: alpm_transflag_t = 1 << 8;
pub const ALPM_TRANS_FLAG_DOWNLOADONLY: alpm_transflag_t = 1 << 9;
pub const ALPM_TRANS_FLAG_NOSCRIPTLET: alpm_transflag_t = 1 << 10;
pub const ALPM_TRANS_FLAG_NOCONFLICTS: alpm_transflag_t = 1 << 11;
pub const ALPM_TRANS_FLAG_NEEDED: alpm_transflag_t = 1 << 13;
pub const ALPM_TRANS_FLAG_ALLEXPLICIT: alpm_transflag_t = 1 << 14;
pub const ALPM_TRANS_FLAG_UNNEEDED: alpm_transflag_t = 1 << 15;
pub const ALPM_TRANS_FLAG_RECURSEALL: alpm_transflag_t = 1 << 16;
pub const ALPM_TRANS_FLAG_NOLOCK: alpm_transflag_t = 1 << 17;
// alpm_caps
pub const ALPM_CAPABILITY_NLS: alpm_caps = 1;
pub const ALPM_CAPABILITY_DOWNLOADER: alpm_caps = 2;
pub const ALPM_CAPABILITY_SIGNATURES: alpm_caps = 4;

// Opaque types
pub enum Struct_alpm_handle { }
pub enum Struct_alpm_db { }
pub enum Struct_alpm_pkg { }
pub enum Struct_va_list { }

pub type alpm_time_t = int64_t;

// const'd enums
pub type alpm_errno_t = u32;
pub type alpm_pkgreason_t = u32;
pub type alpm_pkgfrom_t = u32;
pub type alpm_pkgvalidation_t = u32;
pub type alpm_depmod_t = u32;
pub type alpm_fileconflicttype_t = u32;
pub type alpm_siglevel_t = u32;
pub type alpm_sigstatus_t = u32;
pub type alpm_sigvalidity_t = u32;
pub type alpm_hook_when_t = u32;
pub type alpm_loglevel_t = u32;
pub type alpm_event_type_t = u32;
pub type alpm_package_operation_t = u32;
pub type alpm_question_type_t = u32;
pub type alpm_progress_t = u32;
pub type alpm_db_usage_t = u32;
pub type alpm_transflag_t = u32;
pub type alpm_caps = u32;

// callbacks
pub type alpm_list_fn_free = Option<unsafe extern "C" fn(arg1: *mut c_void)>;
pub type alpm_list_fn_cmp = Option<unsafe extern "C" fn(arg1: *const c_void,
                                                        arg2: *const c_void)
                                                        -> c_int>;
pub type alpm_cb_log = Option<unsafe extern "C" fn(arg1: alpm_loglevel_t,
                                                   arg2: *const c_char,
                                                   arg3: *const Struct_va_list)>;
pub type alpm_cb_download = Option<unsafe extern "C" fn(filename: *const c_char,
                                                        xfered: off_t,
                                                        total: off_t)>;
pub type alpm_cb_totaldl = Option<unsafe extern "C" fn(total: off_t)>;
pub type alpm_cb_fetch = Option<unsafe extern "C" fn(url: *const c_char,
                                                     localpath: *const c_char,
                                                     force: c_int)
                                                     -> c_int>;
pub type alpm_cb_event = Option<unsafe extern "C" fn(arg1: *const alpm_event_t)>;
pub type alpm_cb_question = Option<unsafe extern "C" fn(arg1: *const alpm_question_t)>;
pub type alpm_cb_progress = Option<unsafe extern "C" fn(arg1: alpm_progress_t,
                                                        arg2: *const c_char,
                                                        arg3: c_int,
                                                        arg4: usize,
                                                        arg5: usize)>;

// structs
#[repr(C)]
pub struct alpm_list_t {
    pub data: *const c_void,
    pub prev: *const alpm_list_t,
    pub next: *const alpm_list_t,
}

#[repr(C)]
pub struct alpm_depend_t {
    pub name: *const c_char,
    pub version: *const c_char,
    pub desc: *const c_char,
    pub name_hash: c_ulong,
    pub mod_: alpm_depmod_t,
}

#[repr(C)]
pub struct alpm_depmissing_t {
    pub target: *const c_char,
    pub depend: *const alpm_depend_t,
    pub causingpkg: *const c_char,
}

#[repr(C)]
pub struct alpm_conflict_t {
    pub package1_hash: c_ulong,
    pub package2_hash: c_ulong,
    pub package1: *const c_char,
    pub package2: *const c_char,
    pub reason: *const alpm_depend_t,
}

#[repr(C)]
pub struct alpm_fileconflict_t {
    pub target: *const c_char,
    pub type_: alpm_fileconflicttype_t,
    pub file: *const c_char,
    pub ctarget: *const c_char,
}

#[repr(C)]
pub struct alpm_group_t {
    pub name: *const c_char,
    pub packages: *const alpm_list_t,
}

#[repr(C)]
pub struct alpm_delta_t {
    pub delta: *const c_char,
    pub delta_md5: *const c_char,
    pub from: *const c_char,
    pub to: *const c_char,
    pub delta_size: off_t,
    pub download_size: off_t,
}

#[repr(C)]
pub struct alpm_file_t {
    pub name: *const c_char,
    pub size: off_t,
    pub mode: mode_t,
}

#[repr(C)]
pub struct alpm_filelist_t {
    pub count: usize,
    pub files: *const alpm_file_t,
}

#[repr(C)]
pub struct alpm_backup_t {
    pub name: *const c_char,
    pub hash: *const c_char,
}

#[repr(C)]
pub struct alpm_pgpkey_t {
    pub data: *const c_void,
    pub fingerprint: *const c_char,
    pub uid: *const c_char,
    pub name: *const c_char,
    pub email: *const c_char,
    pub created: alpm_time_t,
    pub expires: alpm_time_t,
    pub length: c_uint,
    pub revoked: c_uint,
    pub pubkey_algo: c_char,
}

#[repr(C)]
pub struct alpm_sigresult_t {
    pub key: alpm_pgpkey_t,
    pub status: alpm_sigstatus_t,
    pub validity: alpm_sigvalidity_t,
}

#[repr(C)]
pub struct alpm_siglist_t {
    pub count: usize,
    pub results: *const alpm_sigresult_t,
}

#[repr(C)]
pub struct alpm_event_any_t {
    type_: alpm_event_type_t,
}

#[repr(C)]
pub struct alpm_event_package_operation_t {
    pub type_: alpm_event_type_t,
    pub operation: alpm_package_operation_t,
    pub oldpkg: *const Struct_alpm_pkg,
    pub newpkg: *const Struct_alpm_pkg,
}

#[repr(C)]
pub struct alpm_event_optdep_removal_t {
    pub type_: alpm_event_type_t,
    pub pkg: *const Struct_alpm_pkg,
    pub optdep: *const alpm_depend_t,
}

#[repr(C)]
pub struct alpm_event_delta_patch_t {
    pub type_: alpm_event_type_t,
    pub delta: *const alpm_delta_t,
}

#[repr(C)]
pub struct alpm_event_scriptlet_info_t {
    pub type_: alpm_event_type_t,
    pub line: *const c_char,
}

#[repr(C)]
pub struct alpm_event_database_missing_t {
    pub type_: alpm_event_type_t,
    pub dbname: *const c_char,
}

#[repr(C)]
pub struct alpm_event_pkgdownload_t {
    pub type_: alpm_event_type_t,
    pub file: *const c_char,
}

#[repr(C)]
pub struct alpm_event_pacnew_created_t {
    pub type_: alpm_event_type_t,
    pub from_noupgrade: c_int,
    pub oldpkg: *const Struct_alpm_pkg,
    pub newpkg: *const Struct_alpm_pkg,
    pub file: *const c_char,
}

#[repr(C)]
pub struct alpm_event_pacsave_created_t {
    pub type_: alpm_event_type_t,
    pub oldpkg: *const Struct_alpm_pkg,
    pub file: *const c_char,
}

#[repr(C)]
pub struct alpm_event_hook_t {
    pub type_: alpm_event_type_t,
    pub when: alpm_hook_when_t,
}

#[repr(C)]
pub struct alpm_event_hook_run_t {
    pub type_: alpm_event_type_t,
    pub name: *const c_char,
    pub desc: *const c_char,
    pub position: usize,
    pub total: usize,
}

#[repr(C)]
pub union alpm_event_t {
    pub type_: alpm_event_type_t,
    pub any: alpm_event_any_t,
    pub package_operation: alpm_event_package_operation_t,
    pub optdep_removal: alpm_event_optdep_removal_t,
    pub delta_patch: alpm_event_delta_patch_t,
    pub scriptlet_info: alpm_event_scriptlet_info_t,
    pub database_missing: alpm_event_database_missing_t,
    pub pkgdownload: alpm_event_pkgdownload_t,
    pub pacnew_created: alpm_event_pacnew_created_t,
    pub pacsave_created: alpm_event_pacsave_created_t,
    pub hook: alpm_event_hook_t,
    pub hook_run: alpm_event_hook_run_t,
}

#[repr(C)]
pub struct alpm_question_any_t {
    pub type_: alpm_question_type_t,
    pub answer: c_int,
}

#[repr(C)]
pub struct alpm_question_install_ignorepkg_t {
    pub type_: alpm_question_type_t,
    pub install: c_int,
    pub pkg: *const Struct_alpm_pkg,
}

#[repr(C)]
pub struct alpm_question_replace_t {
    pub type_: alpm_question_type_t,
    pub replace: c_int,
    pub oldpkg: *const Struct_alpm_pkg,
    pub newpkg: *const Struct_alpm_pkg,
    pub newdb: *const Struct_alpm_db,
}

#[repr(C)]
pub struct alpm_question_conflict_t {
    pub type_: alpm_question_type_t,
    pub remove: c_int,
    pub conflict: *const alpm_conflict_t,
}

#[repr(C)]
pub struct alpm_question_corrupted_t {
    pub type_: alpm_question_type_t,
    pub remove: c_int,
    pub filepath: *const c_char,
    pub reason: alpm_errno_t,
}

#[repr(C)]
pub struct alpm_question_remove_pkgs_t {
    pub type_: alpm_question_type_t,
    pub skip: c_int,
    pub packages: *const alpm_list_t,
}

#[repr(C)]
pub struct alpm_question_select_provider_t {
    pub type_: alpm_question_type_t,
    pub use_index: c_int,
    pub providers: *const alpm_list_t,
    pub depend: *const alpm_depend_t,
}

#[repr(C)]
pub struct alpm_question_import_key_t {
    pub type_: alpm_question_type_t,
    pub import: c_int,
    pub key: *const alpm_pgpkey_t,
}

#[repr(C)]
pub union alpm_question_t {
    pub type_: alpm_question_type_t,
    pub any: alpm_question_any_t,
    pub install_ignorepkg: alpm_question_install_ignorepkg_t,
    pub replace: alpm_question_replace_t,
    pub conflict: alpm_question_conflict_t,
    pub corrupted: alpm_question_corrupted_t,
    pub remove_pkgs: alpm_question_remove_pkgs_t,
    pub select_provider: alpm_question_select_provider_t,
    pub import_key: alpm_question_import_key_t,
}

#[link(name = "alpm")]
extern "C" {
    // alpm_list
    pub fn alpm_list_free(list: *const alpm_list_t);
    pub fn alpm_list_free_inner(list: *const alpm_list_t, fn_: alpm_list_fn_free);
    pub fn alpm_list_add(list: *const alpm_list_t, data: *const c_void) -> *const alpm_list_t;
    pub fn alpm_list_append(list: *const *mut alpm_list_t, data: *const c_void) -> *const alpm_list_t;
    pub fn alpm_list_add_sorted(list: *const alpm_list_t,
                                data: *const c_void,
                                fn_: alpm_list_fn_cmp)
                                -> *const alpm_list_t;
    pub fn alpm_list_join(first: *const alpm_list_t, second: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_list_mmerge(left: *const alpm_list_t,
                            right: *const alpm_list_t,
                            fn_: alpm_list_fn_cmp)
                            -> *const alpm_list_t;
    pub fn alpm_list_msort(list: *const alpm_list_t,
                           n: usize,
                           fn_: alpm_list_fn_cmp)
                           -> *const alpm_list_t;
    pub fn alpm_list_remove_item(haystack: *const alpm_list_t,
                                 item: *const alpm_list_t)
                                 -> *const alpm_list_t;
    pub fn alpm_list_remove(haystack: *const alpm_list_t,
                            needle: *const c_void,
                            fn_: alpm_list_fn_cmp,
                            data: *const *mut c_void)
                            -> *const alpm_list_t;
    pub fn alpm_list_remove_str(haystack: *const alpm_list_t,
                                needle: *const c_char,
                                data: *const *mut c_char)
                                -> *const alpm_list_t;
    pub fn alpm_list_remove_dupes(list: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_list_strdup(list: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_list_copy(list: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_list_copy_data(list: *const alpm_list_t, size: usize) -> *const alpm_list_t;
    pub fn alpm_list_reverse(list: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_list_nth(list: *const alpm_list_t, n: usize) -> *const alpm_list_t;
    pub fn alpm_list_next(list: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_list_previous(list: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_list_last(list: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_list_count(list: *const alpm_list_t) -> usize;
    pub fn alpm_list_find(haystack: *const alpm_list_t,
                          needle: *const c_void,
                          fn_: alpm_list_fn_cmp)
                          -> *const c_void;
    pub fn alpm_list_find_ptr(haystack: *const alpm_list_t, needle: *const c_void) -> *const c_void;
    pub fn alpm_list_find_str(haystack: *const alpm_list_t, needle: *const c_char) -> *const c_char;
    pub fn alpm_list_diff(lhs: *const alpm_list_t,
                          rhs: *const alpm_list_t,
                          fn_: alpm_list_fn_cmp)
                          -> *const alpm_list_t;
    pub fn alpm_list_diff_sorted(left: *const alpm_list_t,
                                 right: *const alpm_list_t,
                                 fn_: alpm_list_fn_cmp,
                                 onlyleft: *const *mut alpm_list_t,
                                 onlyright: *const *mut alpm_list_t);
    pub fn alpm_list_to_array(list: *const alpm_list_t, n: usize, size: usize) -> *const c_void;

    // alpm
    pub fn alpm_errno(handle: *const Struct_alpm_handle) -> alpm_errno_t;
    pub fn alpm_strerror(err: alpm_errno_t) -> *const c_char;
    pub fn alpm_logaction(handle: *const Struct_alpm_handle,
                          prefix: *const c_char,
                          fmt: *const c_char, ...)
                          -> c_int;
    pub fn alpm_fetch_pkgurl(handle: *const Struct_alpm_handle,
                             url: *const c_char)
                             -> *const c_char;
    pub fn alpm_option_get_logcb(handle: *const Struct_alpm_handle) -> alpm_cb_log;
    pub fn alpm_option_set_logcb(handle: *const Struct_alpm_handle, cb: alpm_cb_log) -> c_int;
    pub fn alpm_option_get_dlcb(handle: *const Struct_alpm_handle) -> alpm_cb_download;
    pub fn alpm_option_set_dlcb(handle: *const Struct_alpm_handle, cb: alpm_cb_download) -> c_int;
    pub fn alpm_option_get_fetchcb(handle: *const Struct_alpm_handle) -> alpm_cb_fetch;
    pub fn alpm_option_set_fetchcb(handle: *const Struct_alpm_handle, cb: alpm_cb_fetch) -> c_int;
    pub fn alpm_option_get_totaldlcb(handle: *const Struct_alpm_handle) -> alpm_cb_totaldl;
    pub fn alpm_option_set_totaldlcb(handle: *const Struct_alpm_handle, cb: alpm_cb_totaldl) -> c_int;
    pub fn alpm_option_get_eventcb(handle: *const Struct_alpm_handle) -> alpm_cb_event;
    pub fn alpm_option_set_eventcb(handle: *const Struct_alpm_handle, cb: alpm_cb_event) -> c_int;
    pub fn alpm_option_get_questioncb(handle: *const Struct_alpm_handle) -> alpm_cb_question;
    pub fn alpm_option_set_questioncb(handle: *const Struct_alpm_handle, cb: alpm_cb_question) -> c_int;
    pub fn alpm_option_get_progresscb(handle: *const Struct_alpm_handle) -> alpm_cb_progress;
    pub fn alpm_option_set_progresscb(handle: *const Struct_alpm_handle, cb: alpm_cb_progress) -> c_int;
    pub fn alpm_option_get_root(handle: *const Struct_alpm_handle) -> *const c_char;
    pub fn alpm_option_get_dbpath(handle: *const Struct_alpm_handle) -> *const c_char;
    pub fn alpm_option_get_lockfile(handle: *const Struct_alpm_handle) -> *const c_char;
    pub fn alpm_option_get_cachedirs(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_option_set_cachedirs(handle: *const Struct_alpm_handle,
                                     cachedirs: *const alpm_list_t)
                                     -> c_int;
    pub fn alpm_option_add_cachedir(handle: *const Struct_alpm_handle, cachedir: *const c_char) -> c_int;
    pub fn alpm_option_remove_cachedir(handle: *const Struct_alpm_handle,
                                       cachedir: *const c_char)
                                       -> c_int;
    pub fn alpm_option_get_hookdirs(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_option_set_hookdirs(handle: *const Struct_alpm_handle,
                                    hookdirs: *const alpm_list_t)
                                    -> c_int;
    pub fn alpm_option_add_hookdir(handle: *const Struct_alpm_handle,
                                   hookdir: *const c_char)
                                   -> c_int;
    pub fn alpm_option_remove_hookdir(handle: *const Struct_alpm_handle,
                                      hookdir: *const c_char)
                                      -> c_int;
    pub fn alpm_option_get_logfile(handle: *const Struct_alpm_handle) -> *const c_char;
    pub fn alpm_option_set_logfile(handle: *const Struct_alpm_handle,
                                   logfile: *const c_char)
                                   -> c_int;
    pub fn alpm_option_get_gpgdir(handle: *const Struct_alpm_handle)
                                  -> *const c_char;
    pub fn alpm_option_set_gpgdir(handle: *const Struct_alpm_handle,
                                  gpgdir: *const c_char)
                                  -> c_int;
    pub fn alpm_option_get_usesyslog(handle: *const Struct_alpm_handle) -> c_int;
    pub fn alpm_option_set_usesyslog(handle: *const Struct_alpm_handle, usesyslog: c_int) -> c_int;
    pub fn alpm_option_get_noupgrades(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_option_add_noupgrade(handle: *const Struct_alpm_handle, path: *const c_char) -> c_int;
    pub fn alpm_option_set_noupgrades(handle: *const Struct_alpm_handle,
                                      noupgrade: *const alpm_list_t)
                                      -> c_int;
    pub fn alpm_option_remove_noupgrade(handle: *const Struct_alpm_handle,
                                        path: *const c_char)
                                        -> c_int;
    pub fn alpm_option_match_noupgrade(handle: *const Struct_alpm_handle, path: *const c_char) -> c_int;
    pub fn alpm_option_get_noextracts(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_option_add_noextract(handle: *const Struct_alpm_handle, path: *const c_char) -> c_int;
    pub fn alpm_option_set_noextracts(handle: *const Struct_alpm_handle,
                                      noextract: *const alpm_list_t)
                                      -> c_int;
    pub fn alpm_option_remove_noextract(handle: *const Struct_alpm_handle, path: *const c_char) -> c_int;
    pub fn alpm_option_match_noextract(handle: *const Struct_alpm_handle, path: *const c_char) -> c_int;
    pub fn alpm_option_get_ignorepkgs(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_option_add_ignorepkg(handle: *const Struct_alpm_handle, pkg: *const c_char) -> c_int;
    pub fn alpm_option_set_ignorepkgs(handle: *const Struct_alpm_handle,
                                      ignorepkgs: *const alpm_list_t)
                                      -> c_int;
    pub fn alpm_option_remove_ignorepkg(handle: *const Struct_alpm_handle, pkg: *const c_char) -> c_int;
    pub fn alpm_option_get_ignoregroups(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_option_add_ignoregroup(handle: *const Struct_alpm_handle, grp: *const c_char) -> c_int;
    pub fn alpm_option_set_ignoregroups(handle: *const Struct_alpm_handle,
                                        ignoregrps: *const alpm_list_t)
                                        -> c_int;
    pub fn alpm_option_remove_ignoregroup(handle: *const Struct_alpm_handle, grp: *const c_char) -> c_int;
    pub fn alpm_option_get_assumeinstalled(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_option_add_assumeinstalled(handle: *const Struct_alpm_handle,
                                           dep: *const alpm_depend_t)
                                           -> c_int;
    pub fn alpm_option_set_assumeinstalled(handle: *const Struct_alpm_handle,
                                           deps: *const alpm_list_t)
                                           -> c_int;
    pub fn alpm_option_remove_assumeinstalled(handle: *const Struct_alpm_handle,
                                              dep: *const alpm_depend_t)
                                              -> c_int;
    pub fn alpm_option_get_arch(handle: *const Struct_alpm_handle) -> *const c_char;
    pub fn alpm_option_set_arch(handle: *const Struct_alpm_handle, arch: *const c_char) -> c_int;
    pub fn alpm_option_get_deltaratio(handle: *const Struct_alpm_handle) -> c_double;
    pub fn alpm_option_set_deltaratio(handle: *const Struct_alpm_handle, ratio: c_double) -> c_int;
    pub fn alpm_option_get_checkspace(handle: *const Struct_alpm_handle) -> c_int;
    pub fn alpm_option_set_checkspace(handle: *const Struct_alpm_handle, checkspace: c_int) -> c_int;
    pub fn alpm_option_get_dbext(handle: *const Struct_alpm_handle) -> *const c_char;
    pub fn alpm_option_set_dbext(handle: *const Struct_alpm_handle, dbext: *const c_char) -> c_int;
    pub fn alpm_option_get_default_siglevel(handle: *const Struct_alpm_handle) -> alpm_siglevel_t;
    pub fn alpm_option_set_default_siglevel(handle: *const Struct_alpm_handle,
                                            level: alpm_siglevel_t)
                                            -> c_int;
    pub fn alpm_option_get_local_file_siglevel(handle: *const Struct_alpm_handle) -> alpm_siglevel_t;
    pub fn alpm_option_set_local_file_siglevel(handle: *const Struct_alpm_handle,
                                               level: alpm_siglevel_t)
                                               -> c_int;
    pub fn alpm_option_get_remote_file_siglevel(handle: *const Struct_alpm_handle) -> alpm_siglevel_t;
    pub fn alpm_option_set_remote_file_siglevel(handle: *const Struct_alpm_handle,
                                                level: alpm_siglevel_t)
                                                -> c_int;
    pub fn alpm_get_localdb(handle: *const Struct_alpm_handle) -> *const Struct_alpm_db;
    pub fn alpm_get_syncdbs(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_register_syncdb(handle: *const Struct_alpm_handle,
                                treename: *const c_char,
                                level: alpm_siglevel_t)
                                -> *const Struct_alpm_db;
    pub fn alpm_unregister_all_syncdbs(handle: *const Struct_alpm_handle) -> c_int;
    pub fn alpm_db_unregister(db: *const Struct_alpm_db) -> c_int;
    pub fn alpm_db_get_name(db: *const Struct_alpm_db) -> *const c_char;
    pub fn alpm_db_get_siglevel(db: *const Struct_alpm_db) -> alpm_siglevel_t;
    pub fn alpm_db_get_valid(db: *const Struct_alpm_db) -> c_int;
    pub fn alpm_db_get_servers(db: *const Struct_alpm_db) -> *const alpm_list_t;
    pub fn alpm_db_set_servers(db: *const Struct_alpm_db, servers: *const alpm_list_t) -> c_int;
    pub fn alpm_db_add_server(db: *const Struct_alpm_db, url: *const c_char) -> c_int;
    pub fn alpm_db_remove_server(db: *const Struct_alpm_db, url: *const c_char) -> c_int;
    pub fn alpm_db_update(force: c_int, db: *const Struct_alpm_db) -> c_int;
    pub fn alpm_db_get_pkg(db: *const Struct_alpm_db, name: *const c_char) -> *const Struct_alpm_pkg;
    pub fn alpm_db_get_pkgcache(db: *const Struct_alpm_db) -> *const alpm_list_t;
    pub fn alpm_db_get_group(db: *const Struct_alpm_db, name: *const c_char) -> *const alpm_group_t;
    pub fn alpm_db_get_groupcache(db: *const Struct_alpm_db) -> *const alpm_list_t;
    pub fn alpm_db_search(db: *const Struct_alpm_db, needles: *const alpm_list_t) -> *const alpm_list_t;
    pub fn alpm_db_set_usage(db: *const Struct_alpm_db, usage: alpm_db_usage_t) -> c_int;
    pub fn alpm_db_get_usage(db: *const Struct_alpm_db, usage: *const alpm_db_usage_t) -> c_int;
    pub fn alpm_pkg_load(handle: *const Struct_alpm_handle,
                         filename: *const c_char,
                         full: c_int, level: alpm_siglevel_t,
                         pkg: *const *mut Struct_alpm_pkg)
                         -> c_int;
    pub fn alpm_pkg_find(haystack: *const alpm_list_t, needle: *const c_char) -> *const Struct_alpm_pkg;
    pub fn alpm_pkg_free(pkg: *const Struct_alpm_pkg) -> c_int;
    pub fn alpm_pkg_checkmd5sum(pkg: *const Struct_alpm_pkg) -> c_int;
    pub fn alpm_pkg_vercmp(a: *const c_char, b: *const c_char) -> c_int;
    pub fn alpm_pkg_compute_requiredby(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_compute_optionalfor(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_should_ignore(handle: *const Struct_alpm_handle, pkg: *const Struct_alpm_pkg) -> c_int;
    pub fn alpm_pkg_get_filename(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_base(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_name(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_version(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_origin(pkg: *const Struct_alpm_pkg) -> alpm_pkgfrom_t;
    pub fn alpm_pkg_get_desc(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_url(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_builddate(pkg: *const Struct_alpm_pkg) -> alpm_time_t;
    pub fn alpm_pkg_get_installdate(pkg: *const Struct_alpm_pkg) -> alpm_time_t;
    pub fn alpm_pkg_get_packager(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_md5sum(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_sha256sum(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_arch(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_size(pkg: *const Struct_alpm_pkg) -> off_t;
    pub fn alpm_pkg_get_isize(pkg: *const Struct_alpm_pkg) -> off_t;
    pub fn alpm_pkg_get_reason(pkg: *const Struct_alpm_pkg) -> alpm_pkgreason_t;
    pub fn alpm_pkg_get_licenses(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_groups(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_depends(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_optdepends(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_checkdepends(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_makedepends(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_conflicts(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_provides(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_deltas(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_replaces(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_files(pkg: *const Struct_alpm_pkg) -> *const alpm_filelist_t;
    pub fn alpm_pkg_get_backup(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_get_db(pkg: *const Struct_alpm_pkg) -> *const Struct_alpm_db;
    pub fn alpm_pkg_get_base64_sig(pkg: *const Struct_alpm_pkg) -> *const c_char;
    pub fn alpm_pkg_get_validation(pkg: *const Struct_alpm_pkg) -> alpm_pkgvalidation_t;
    pub fn alpm_pkg_changelog_open(pkg: *const Struct_alpm_pkg) -> *const c_void;
    pub fn alpm_pkg_changelog_read(ptr: *const c_void,
                                   size: usize,
                                   pkg: *const Struct_alpm_pkg,
                                   fp: *const c_void)
                                   -> usize;
    pub fn alpm_pkg_changelog_close(pkg: *const Struct_alpm_pkg, fp: *const c_void) -> c_int;
    pub fn alpm_pkg_mtree_open(pkg: *const Struct_alpm_pkg) -> *const Struct_archive;
    pub fn alpm_pkg_mtree_next(pkg: *const Struct_alpm_pkg,
                               archive: *const Struct_archive,
                               entry: *const *mut Struct_archive_entry)
                               -> c_int;
    pub fn alpm_pkg_mtree_close(pkg: *const Struct_alpm_pkg,
                                archive: *const Struct_archive)
                                -> c_int;
    pub fn alpm_pkg_has_scriptlet(pkg: *const Struct_alpm_pkg) -> c_int;
    pub fn alpm_pkg_download_size(newpkg: *const Struct_alpm_pkg) -> off_t;
    pub fn alpm_pkg_unused_deltas(pkg: *const Struct_alpm_pkg) -> *const alpm_list_t;
    pub fn alpm_pkg_set_reason(pkg: *const Struct_alpm_pkg, reason: alpm_pkgreason_t) -> c_int;
    pub fn alpm_filelist_contains(filelist: *const alpm_filelist_t,
                                  path: *const c_char)
                                  -> *const alpm_file_t;
    pub fn alpm_pkg_check_pgp_signature(pkg: *const Struct_alpm_pkg,
                                        siglist: *const alpm_siglist_t)
                                        -> c_int;
    pub fn alpm_db_check_pgp_signature(db: *const Struct_alpm_db, siglist: *const alpm_siglist_t) -> c_int;
    pub fn alpm_siglist_cleanup(siglist: *const alpm_siglist_t) -> c_int;
    pub fn alpm_decode_signature(base64_data: *const c_char,
                                 data: *const *mut c_uchar,
                                 data_len: *const usize)
                                 -> c_int;
    pub fn alpm_extract_keyid(handle: *const Struct_alpm_handle,
                              identifier: *const c_char,
                              sig: *const c_uchar, len: usize,
                              keys: *const *mut alpm_list_t)
                              -> c_int;
    pub fn alpm_find_group_pkgs(dbs: *const alpm_list_t, name: *const c_char) -> *const alpm_list_t;
    pub fn alpm_sync_newversion(pkg: *const Struct_alpm_pkg,
                                dbs_sync: *const alpm_list_t)
                                -> *const Struct_alpm_pkg;
    pub fn alpm_trans_get_flags(handle: *const Struct_alpm_handle) -> alpm_transflag_t;
    pub fn alpm_trans_get_add(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_trans_get_remove(handle: *const Struct_alpm_handle) -> *const alpm_list_t;
    pub fn alpm_trans_init(handle: *const Struct_alpm_handle, flags: alpm_transflag_t) -> c_int;
    pub fn alpm_trans_prepare(handle: *const Struct_alpm_handle,
                              data: *const *mut alpm_list_t)
                              -> c_int;
    pub fn alpm_trans_commit(handle: *const Struct_alpm_handle,
                             data: *const *mut alpm_list_t)
                             -> c_int;
    pub fn alpm_trans_interrupt(handle: *const Struct_alpm_handle) -> c_int;
    pub fn alpm_trans_release(handle: *const Struct_alpm_handle) -> c_int;
    pub fn alpm_sync_sysupgrade(handle: *const Struct_alpm_handle, enable_downgrade: c_int) -> c_int;
    pub fn alpm_add_pkg(handle: *const Struct_alpm_handle, pkg: *const Struct_alpm_pkg) -> c_int;
    pub fn alpm_remove_pkg(handle: *const Struct_alpm_handle, pkg: *const Struct_alpm_pkg) -> c_int;
    pub fn alpm_checkdeps(handle: *const Struct_alpm_handle,
                          pkglist: *const alpm_list_t,
                          remove: *const alpm_list_t,
                          upgrade: *const alpm_list_t,
                          reversedeps: c_int)
                          -> *const alpm_list_t;
    pub fn alpm_find_satisfier(pkgs: *const alpm_list_t,
                               depstring: *const c_char)
                               -> *const Struct_alpm_pkg;
    pub fn alpm_find_dbs_satisfier(handle: *const Struct_alpm_handle,
                                   dbs: *const alpm_list_t,
                                   depstring: *const c_char)
                                   -> *const Struct_alpm_pkg;
    pub fn alpm_checkconflicts(handle: *const Struct_alpm_handle,
                               pkglist: *const alpm_list_t)
                               -> *const alpm_list_t;
    pub fn alpm_dep_compute_string(dep: *const alpm_depend_t) -> *const c_char;
    pub fn alpm_dep_from_string(depstring: *const c_char) -> *const alpm_depend_t;
    pub fn alpm_dep_free(dep: *const alpm_depend_t);
    pub fn alpm_compute_md5sum(filename: *const c_char) -> *const c_char;
    pub fn alpm_compute_sha256sum(filename: *const c_char) -> *const c_char;
    pub fn alpm_initialize(root: *const c_char,
                           dbpath: *const c_char,
                           err: *mut alpm_errno_t)
                           -> *const Struct_alpm_handle;
    pub fn alpm_release(handle: *const Struct_alpm_handle) -> c_int;
    pub fn alpm_unlock(handle: *const Struct_alpm_handle) -> c_int;
    pub fn alpm_version() -> *const c_char;
    pub fn alpm_capabilities() -> alpm_caps;
    pub fn alpm_fileconflict_free(conflict: *const alpm_fileconflict_t);
    pub fn alpm_depmissing_free(miss: *const alpm_depmissing_t);
    pub fn alpm_conflict_free(conflict: *const alpm_conflict_t);
}
