
use alpm_sys::*;
use Alpm;

use std::ops::Deref;
use std::marker::PhantomData;
use std::mem;
use std::ffi::CStr;
use std::str::Utf8Error;

// https://github.com/jeremyletang/rust-sfml/blob/csfml-2.4/src/graphics/texture.rs#L44-L60 for
// pattern

/// An owning version of Package
pub struct Package (*const Struct_alpm_pkg);

impl Package {
    pub(crate) fn new(raw: *const Struct_alpm_pkg) -> Package {
        Package(raw)
    }
}

impl Drop for Package {
    fn drop(&mut self) {
        unsafe { alpm_pkg_free(self.0); }
    }
}

impl Deref for Package {
    type Target = PackageRef;
    fn deref(&self) -> &PackageRef {
        unsafe { &*(self.0 as *const PackageRef as *mut PackageRef) }
    }
}

/// A package in libalpm that can only be held by reference.
pub enum PackageRef {}

impl PackageRef {
    pub(crate) unsafe fn new<'b>(p: *const Struct_alpm_pkg) -> &'b PackageRef {
        &*(p as *const PackageRef as *mut PackageRef)
    }

    pub fn md5(&self) -> Option<&str> {
        unsafe {
            let str_ptr = alpm_pkg_get_md5sum(self as *const _ as _);
            if str_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(str_ptr).to_str().unwrap()) // cannot fail
            }
        }
    }

    /// Checks package integrity using md5. Returns true on success.
    pub fn check_md5(&self) -> bool {
        //unsafe { println!("{:?}, {:?}", self.0, alpm_pkg_checkmd5sum(self.0)); }
        unsafe { alpm_pkg_checkmd5sum(self as *const _ as _) == 0 }
    }
}

pub enum PackageOperation<'a> {
    /// Package (to be) installed. (No oldpkg)
    Install {
        new_pkg: &'a PackageRef,
    },
    /// Package (to be) upgraded
    Upgrade {
        old_pkg: &'a PackageRef,
        new_pkg: &'a PackageRef,
    },
    /// Package (to be) re-installed.
    Reinstall {
        old_pkg: &'a PackageRef,
        new_pkg: &'a PackageRef,
    },
    /// Package (to be) downgraded.
    Downgrade {
        old_pkg: &'a PackageRef,
        new_pkg: &'a PackageRef,
    },
    /// Package (to be) removed. (No newpkg)
    Remove {
        old_pkg: &'a PackageRef,
    },
}

impl<'a> PackageOperation<'a> {
    pub(crate) unsafe fn new<'b>(op: &alpm_event_package_operation_t) -> PackageOperation<'b> {
        match op.operation {
            ALPM_PACKAGE_INSTALL => PackageOperation::Install {
                new_pkg: PackageRef::new(op.newpkg),
            },
            ALPM_PACKAGE_UPGRADE => PackageOperation::Upgrade {
                new_pkg: PackageRef::new(op.newpkg),
                old_pkg: PackageRef::new(op.oldpkg),
            },
            ALPM_PACKAGE_REINSTALL => PackageOperation::Reinstall {
                new_pkg: PackageRef::new(op.newpkg),
                old_pkg: PackageRef::new(op.oldpkg),
            },
            ALPM_PACKAGE_DOWNGRADE => PackageOperation::Downgrade {
                new_pkg: PackageRef::new(op.newpkg),
                old_pkg: PackageRef::new(op.oldpkg),
            },
            ALPM_PACKAGE_REMOVE => PackageOperation::Remove {
                old_pkg: PackageRef::new(op.oldpkg),
            },
            _ => panic!("Unrecognised package operation"),
        }
    }
}
