//! Utility types/fn for `alpm_event_t`

use alpm_sys::*;

use package::PackageOperation;

/// An event emitted from libalpm
pub enum Event<'a> {
    /// Dependencies will be computed for a package.
    CheckDepsStart,
    /// Dependencies were computed for a package.
    CheckDepsDone,
    /// File conflicts will be computed for a package.
    FileConflictsStart,
    /// File conflicts were computed for a package.
    FileConflictsDone,
    /// Dependencies will be resolved for target package.
    ResolveDepsStart,
    /// Dependencies were resolved for target package.
    ResolveDepsDone,
    /// Inter-conflicts will be checked for target package.
    InterConflictsStart,
    /// Inter-conflicts were checked for target package.
    InterConflictsDone,
    /// Processing the package transaction is starting.
    TransactionStart,
    /// Processing the package transaction is finished.
    TransactionDone,
    /// Package will be installed/upgraded/downgraded/re-installed/removed; See alpm_event_package_operation_t for arguments.
    PackageOperationStart(PackageOperation<'a>),
    /// Package was installed/upgraded/downgraded/re-installed/removed; See alpm_event_package_operation_t for arguments.
    PackageOperationDone(PackageOperation<'a>),
    /// Target package's integrity will be checked.
    IntegrityStart,
    /// Target package's integrity was checked.
    IntegrityDone,
    /// Target package will be loaded.
    LoadStart,
    /// Target package is finished loading.
    LoadDone,
    /// Target delta's integrity will be checked.
    DeltaIntegrityStart,
    /// Target delta's integrity was checked.
    DeltaIntegrityDone,
    /// Deltas will be applied to packages.
    DeltaPatchesStart,
    /// Deltas were applied to packages.
    DeltaPatchesDone,
    /// Delta patch will be applied to target package; See alpm_event_delta_patch_t for arguments..
    DeltaPatchStart,
    /// Delta patch was applied to target package.
    DeltaPatchDone,
    /// Delta patch failed to apply to target package.
    DeltaPatchFailed,
    /// Scriptlet has printed information; See alpm_event_scriptlet_info_t for arguments.
    ScriptletInfo,
    /// Files will be downloaded from a repository.
    RetrieveStart,
    /// Files were downloaded from a repository.
    RetrieveDone,
    /// Not all files were successfully downloaded from a repository.
    RetrieveFailed,
    /// A file will be downloaded from a repository; See alpm_event_pkgdownload_t for arguments
    PkgDownloadStart,
    /// A file was downloaded from a repository; See alpm_event_pkgdownload_t for arguments
    PkgDownloadDone,
    /// A file failed to be downloaded from a repository; See alpm_event_pkgdownload_t for arguments
    PkgDownloadFailed,
    /// Disk space usage will be computed for a package.
    DiskspaceStart,
    /// Disk space usage was computed for a package.
    DiskspaceDone,
    /// An optdepend for another package is being removed; See alpm_event_optdep_removal_t for arguments.
    OptDepRemoval,
    /// A configured repository database is missing; See alpm_event_database_missing_t for arguments.
    DatabaseMissing,
    /// Checking keys used to create signatures are in keyring.
    KeyringStart,
    /// Keyring checking is finished.
    KeyringDone,
    /// Downloading missing keys into keyring.
    KeyDownloadStart,
    /// Key downloading is finished.
    KeyDownloadDone,
    /// A .pacnew file was created; See alpm_event_pacnew_created_t for arguments.
    PacnewCreated,
    /// A .pacsave file was created; See alpm_event_pacsave_created_t for arguments
    PacsaveCreated,
    /// Processing hooks will be started.
    HookStart,
    /// Processing hooks is finished.
    HookDone,
    /// A hook is starting
    HookRunStart,
    /// A hook has finished running
    HookRunDone,
    __Unknown
}

impl<'a> Event<'a> {
    pub(crate) unsafe fn new(e: *const alpm_event_t) -> Event<'static> {
        match (*e).type_ {
            ALPM_EVENT_CHECKDEPS_START => Event::CheckDepsStart,
            ALPM_EVENT_CHECKDEPS_DONE => Event::CheckDepsDone,
            ALPM_EVENT_FILECONFLICTS_START => Event::FileConflictsStart,
            ALPM_EVENT_FILECONFLICTS_DONE => Event::FileConflictsDone,
            ALPM_EVENT_RESOLVEDEPS_START => Event::ResolveDepsStart,
            ALPM_EVENT_RESOLVEDEPS_DONE => Event::ResolveDepsDone,
            ALPM_EVENT_INTERCONFLICTS_START => Event::InterConflictsStart,
            ALPM_EVENT_INTERCONFLICTS_DONE => Event::InterConflictsDone,
            ALPM_EVENT_TRANSACTION_START => Event::TransactionStart,
            ALPM_EVENT_TRANSACTION_DONE => Event::TransactionDone,
            ALPM_EVENT_PACKAGE_OPERATION_START => Event::PackageOperationStart(
                PackageOperation::new(&(*e).package_operation)),
            ALPM_EVENT_PACKAGE_OPERATION_DONE => Event::PackageOperationDone(
                PackageOperation::new(&(*e).package_operation)),
            ALPM_EVENT_INTEGRITY_START => Event::IntegrityStart,
            ALPM_EVENT_INTEGRITY_DONE => Event::IntegrityDone,
            ALPM_EVENT_LOAD_START => Event::LoadStart,
            ALPM_EVENT_LOAD_DONE => Event::LoadDone,
            ALPM_EVENT_DELTA_INTEGRITY_START => Event::DeltaIntegrityStart,
            ALPM_EVENT_DELTA_INTEGRITY_DONE => Event::DeltaIntegrityDone,
            ALPM_EVENT_DELTA_PATCHES_START => Event::DeltaPatchesStart,
            ALPM_EVENT_DELTA_PATCHES_DONE => Event::DeltaPatchesDone,
            ALPM_EVENT_DELTA_PATCH_START => Event::DeltaPatchStart,
            ALPM_EVENT_DELTA_PATCH_DONE => Event::DeltaPatchDone,
            ALPM_EVENT_DELTA_PATCH_FAILED => Event::DeltaPatchFailed,
            ALPM_EVENT_SCRIPTLET_INFO => Event::ScriptletInfo,
            ALPM_EVENT_RETRIEVE_START => Event::RetrieveStart,
            ALPM_EVENT_RETRIEVE_DONE => Event::RetrieveDone,
            ALPM_EVENT_RETRIEVE_FAILED => Event::RetrieveFailed,
            ALPM_EVENT_PKGDOWNLOAD_START => Event::PkgDownloadStart,
            ALPM_EVENT_PKGDOWNLOAD_DONE => Event::PkgDownloadDone,
            ALPM_EVENT_PKGDOWNLOAD_FAILED => Event::PkgDownloadFailed,
            ALPM_EVENT_DISKSPACE_START => Event::DiskspaceStart,
            ALPM_EVENT_DISKSPACE_DONE => Event::DiskspaceDone,
            ALPM_EVENT_OPTDEP_REMOVAL => Event::OptDepRemoval,
            ALPM_EVENT_DATABASE_MISSING => Event::DatabaseMissing,
            ALPM_EVENT_KEYRING_START => Event::KeyringStart,
            ALPM_EVENT_KEYRING_DONE => Event::KeyringDone,
            ALPM_EVENT_KEY_DOWNLOAD_START => Event::KeyDownloadStart,
            ALPM_EVENT_KEY_DOWNLOAD_DONE => Event::KeyDownloadDone,
            ALPM_EVENT_PACNEW_CREATED => Event::PacnewCreated,
            ALPM_EVENT_PACSAVE_CREATED => Event::PacsaveCreated,
            ALPM_EVENT_HOOK_START => Event::HookStart,
            ALPM_EVENT_HOOK_DONE => Event::HookDone,
            ALPM_EVENT_HOOK_RUN_START => Event::HookRunStart,
            ALPM_EVENT_HOOK_RUN_DONE => Event::HookRunDone,
            _ => Event::__Unknown
        }
    }
}
