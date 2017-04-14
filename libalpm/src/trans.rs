//! A module for transactions. Private

use std::any::Any;
use std::ptr;
use std::mem;
use std::marker::PhantomData;

use alpm_sys::*;
use super::{Alpm, Package, PackageRef, Error, AlpmResult, util};
use libc;

/// A state marker for before a transaction is prepared
#[derive(Debug)]
pub enum Initialized {}

/// A state marker for before a transaction is committed, but after it is prepared
#[derive(Debug)]
pub enum Prepared {}

/// A special error type for transactions.
#[derive(Debug)]
pub enum TransactionError<'a> {
    /// An error from the library.
    AlpmError(Error),
    /// The transaction was not prepared as there is nothing to do (no packages added or removed).
    NothingToDo(Transaction<'a, Initialized>),
}

impl From<Error> for TransactionError<'static> {
    fn from(e: Error) -> TransactionError<'static> {
        TransactionError::AlpmError(e)
    }
}

/// A transaction of package operations
///
/// Only certain state transitions are valid TODO model this how hyper crate models response state
///
/// Consumes an Alpm instance as only 1 transaction can be performed at a time. Use `commit` or
/// `rollback` to recover the Alpm instance.
#[derive(Debug)]
pub struct Transaction<'a, S: Any = Initialized> {
    pub(crate) alpm: &'a Alpm,
    pub(crate) _state: PhantomData<S>,
    // We could cache added/removed packages here for speed
}

// This removes the lockfile to make sure future alpm changes can happen
impl<'a, S: Any> Drop for Transaction<'a, S> {
    fn drop(&mut self) {
        unsafe { alpm_trans_release(self.alpm.handle) };
    }
}

impl<'a, S: Any> Transaction<'a, S> {

    /// Returns the flags for the current transaction.
    pub fn flags(&self) -> TransactionFlags {
        unsafe { alpm_trans_get_flags(self.alpm.handle).into() }
    }

    /// Deconstructs the transaction without dropping. Internal only. From hyper.
    fn deconstruct(self) -> &'a Alpm {
        unsafe {
            let alpm = ptr::read(&self.alpm);
            mem::forget(self);
            alpm
        }
    }

    /// Gets packages added by the current transaction.
    pub fn added_packages(&'a self) -> Vec<&'a PackageRef> {
        unsafe {
            let raw_list = alpm_trans_get_add(self.alpm.handle);
            util::alpm_list_to_vec(raw_list, |ptr| {
                &*(ptr as *const PackageRef)
            })
        }
    }

    /// Gets packages removed by the current transaction.
    pub fn removed_packages(&'a self) -> Vec<&'a PackageRef> {
        unsafe {
            let raw_list = alpm_trans_get_remove(self.alpm.handle);
            util::alpm_list_to_vec(raw_list, |ptr| {
                &*(ptr as *const PackageRef)
            })
        }
    }

}

impl<'a> Transaction<'a, Initialized> {

    /// Prepares a transaction for committing.
    ///
    ///  - Checks arch of added packages (fails if arch is wrong for any of them).
    ///  - Checks package removal (todo how does this work?)
    ///  - Reorders package addition and removal into correct dependency order. Emits warning on
    ///    circular dependency.
    ///
    /// TODO an alternative strategy is not return the transaction. This makes things simpler. If
    /// the user needs to recover from this state, there could be `no_op` method (with a better
    /// name) to check if a prepare will fail for this reason.
    pub fn prepare(mut self)
        -> Result<Transaction<'a, Prepared>, TransactionError<'a>>
    {
        unsafe {
            let mut p: *mut alpm_list_t = ptr::null_mut();
            // This introduces overhead but means that we can get the correct transaction state.
            if self.added_packages().is_empty() && self.removed_packages().is_empty() {
                return Err(TransactionError::NothingToDo(self))
            }
            let res = alpm_trans_prepare(self.alpm.handle, &mut p as *mut _);
            if res == 0 {
                let alpm = self.deconstruct();
                Ok(Transaction {
                    alpm: alpm,
                    _state: PhantomData
                })
            } else {
                Err(self.alpm.error().unwrap_or(Error::__Unknown).into())
            }
        }
    }

    /// Adds a system upgrade to this transaction.
    pub fn sys_upgrade(&self, enable_downgrade: bool) -> AlpmResult<()> {
        unsafe {
            let res = alpm_sync_sysupgrade(self.alpm.handle, enable_downgrade as libc::c_int);
            if res == 0 {
                Ok(())
            } else {
                Err(self.alpm.error().unwrap_or(Error::__Unknown))
            }
        }
    }

    /// Adds a new package to system in this transaction.
    pub fn add_package(&self, pkg: &PackageRef) -> AlpmResult<()> {
        unsafe {
            if alpm_add_pkg(self.alpm.handle, pkg as *const _ as _) == 0 {
                Ok(())
            } else {
                Err(self.alpm.error().unwrap_or(Error::__Unknown))
            }
        }
    }

    /// Adds an owned package, moving ownership to the library. A borrowed reference is available,
    /// but it is not necessary to use it.
    pub fn add_owned_package(&self, pkg: Package) -> AlpmResult<&'a PackageRef> {
        unsafe {
            let pkg_ref = PackageRef::new(pkg.forget());
            self.add_package(pkg_ref)?;
            Ok(pkg_ref)
        }
    }

    /// Removes a package from the system in this transaction.
    pub fn remove_package(&self, pkg: &PackageRef) -> AlpmResult<()> {
        unsafe {
            if alpm_remove_pkg(self.alpm.handle, pkg as *const _ as _) == 0 {
                Ok(())
            } else {
                Err(self.alpm.error().unwrap_or(Error::__Unknown))
            }
        }
    }
}

impl<'a> Transaction<'a, Prepared> {

    /// Commits the transaction and returns the alpm instance. TODO conflict type
    ///
    ///  - Download required new packages
    ///  - Check downloaded packages for integrity
    ///  - Synchronize filesystem
    ///
    /// TODO find out how this long-run op works (I guess that this blocks, but another thread can
    /// call interrupt?)
    pub fn commit(self) -> AlpmResult<()> {
        use std::ptr;
        unsafe {
            let mut p: *mut alpm_list_t = ptr::null_mut();
            let res = alpm_trans_commit(self.alpm.handle, &mut p as *mut _);
            if res == 0 {
                Ok(())
            } else {
                Err(self.alpm.error().unwrap_or(Error::__Unknown))
            }
        }
    }

}

/// Configuration options for a transaction.
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct TransactionFlags {
    /// Ignore dependency checks
    no_deps: bool,
    /// Ignore file conflicts and overwrite files
    force: bool,
    /// Delete files even if they are tagged as backup
    no_save: bool,
    /// Ignore version numbers when checking dependencies
    no_dep_version: bool,
    /// Remove also any packages depending on a package being removed
    cascade: bool,
    /// Remove packages and their unneeded deps (not explicitally installed)
    recurse: bool,
    /// Modify database but do not commit changes to filesystem
    db_only: bool,
    /// Mark all installed packages as dependencies.
    all_deps: bool,
    /// Only download packages and do not actually install.
    download_only: bool,
    /// Do not execute install scriptlets after installing
    no_scriptlet: bool,
    /// Ignore dependency conflicts
    no_conflicts: bool,
    /// Do not install a package if it is already installed and up to date
    needed: bool,
    /// Mark all installed packages as explicitally requested.
    all_explicit: bool,
    /// Do not remove a package if it is needed by another one.
    unneeded: bool,
    /// Remove also explicitly installed unneeded deps (use with `recurse: true`)
    recurse_all: bool,
    /// Do not lock the database during the operation.
    no_lock: bool,
}

impl Into<u32> for TransactionFlags {
    fn into(self) -> u32 {
        let mut acc = 0;
        if self.no_deps {
            acc |= ALPM_TRANS_FLAG_NODEPS;
        }
        if self.force {
            acc |= ALPM_TRANS_FLAG_FORCE;
        }
        if self.no_save {
            acc |= ALPM_TRANS_FLAG_NOSAVE;
        }
        if self.no_dep_version {
            acc |= ALPM_TRANS_FLAG_NODEPVERSION;
        }
        if self.cascade {
            acc |= ALPM_TRANS_FLAG_CASCADE;
        }
        if self.recurse {
            acc |= ALPM_TRANS_FLAG_RECURSE;
        }
        if self.db_only {
            acc |= ALPM_TRANS_FLAG_DBONLY;
        }
        if self.all_deps {
            acc |= ALPM_TRANS_FLAG_ALLDEPS;
        }
        if self.download_only {
            acc |= ALPM_TRANS_FLAG_DOWNLOADONLY;
        }
        if self.no_scriptlet {
            acc |= ALPM_TRANS_FLAG_NOSCRIPTLET;
        }
        if self.no_conflicts {
            acc |= ALPM_TRANS_FLAG_NOCONFLICTS;
        }
        if self.needed {
            acc |= ALPM_TRANS_FLAG_NEEDED;
        }
        if self.all_explicit {
            acc |= ALPM_TRANS_FLAG_ALLEXPLICIT;
        }
        if self.unneeded {
            acc |= ALPM_TRANS_FLAG_UNNEEDED;
        }
        if self.recurse_all {
            acc |= ALPM_TRANS_FLAG_RECURSEALL;
        }
        if self.no_lock {
            acc |= ALPM_TRANS_FLAG_NOLOCK;
        }
        acc
    }
}

impl From<u32> for TransactionFlags {
    fn from(from: u32) -> TransactionFlags {
        TransactionFlags {
            no_deps: from & ALPM_TRANS_FLAG_NODEPS != 0,
            force: from & ALPM_TRANS_FLAG_FORCE != 0,
            no_save: from & ALPM_TRANS_FLAG_NOSAVE != 0,
            no_dep_version: from & ALPM_TRANS_FLAG_NODEPVERSION != 0,
            cascade: from & ALPM_TRANS_FLAG_CASCADE != 0,
            recurse: from & ALPM_TRANS_FLAG_RECURSE != 0,
            db_only: from & ALPM_TRANS_FLAG_DBONLY != 0,
            all_deps: from & ALPM_TRANS_FLAG_ALLDEPS != 0,
            download_only: from & ALPM_TRANS_FLAG_DOWNLOADONLY != 0,
            no_scriptlet: from & ALPM_TRANS_FLAG_NOSCRIPTLET != 0,
            no_conflicts: from & ALPM_TRANS_FLAG_NOCONFLICTS != 0,
            needed: from & ALPM_TRANS_FLAG_NEEDED != 0,
            all_explicit: from & ALPM_TRANS_FLAG_ALLEXPLICIT != 0,
            unneeded: from & ALPM_TRANS_FLAG_UNNEEDED != 0,
            recurse_all: from & ALPM_TRANS_FLAG_RECURSEALL != 0,
            no_lock: from & ALPM_TRANS_FLAG_NOLOCK != 0,
        }
    }
}

#[test]
fn test_transaction_flags() {
    let t: TransactionFlags = Default::default();
    // (my) sanity check that deriving bool = false
    assert!(!t.no_lock);
}

