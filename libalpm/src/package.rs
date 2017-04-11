
use alpm_sys::*;
use libc::{self, c_char, c_ulong};
use chrono::{NaiveDateTime, NaiveDate};

use util;
use {Alpm, SigLevel, AlpmResult, Error, Db};

use std::ops::Deref;
use std::ffi::{CStr, CString};
use std::cmp;
use std::ptr;
use std::fmt;
use std::marker::PhantomData;

// https://github.com/jeremyletang/rust-sfml/blob/csfml-2.4/src/graphics/texture.rs#L44-L60 for
// pattern

/// An owning version of Package
pub struct Package<'a> {
    inner: *const Struct_alpm_pkg,
    handle: &'a Alpm
}

impl<'b> Package<'b> {
    pub(crate) fn new<'a>(raw: *const Struct_alpm_pkg, handle: &'a Alpm) -> Package<'a> {
        Package {
            inner: raw,
            handle: handle,
        }
    }

    /// Creates a package from a file
    pub fn load<'a>(alpm: &'a Alpm, filename: &str, full: bool, level: SigLevel)
        -> AlpmResult<Package<'a>>
    {
        unsafe {
            let pkg: *mut Struct_alpm_pkg = ptr::null_mut();
            let res = alpm_pkg_load(alpm.handle,
                                    CString::new(filename).unwrap().as_ptr(),
                                    if full { 1 } else { 0 },
                                    level.into(),
                                    &pkg as *const *mut Struct_alpm_pkg);
            if res == 0 {
                Ok(Package::new(pkg, alpm))
            } else {
                Err(alpm.error().unwrap_or(Error::__Unknown))
            }
        }
    }
}

impl<'a> Drop for Package<'a> {
    fn drop(&mut self) {
        unsafe { alpm_pkg_free(self.inner); }
    }
}

impl<'a> Deref for Package<'a> {
    type Target = PackageRef;
    fn deref(&self) -> &PackageRef {
        unsafe { &*(self.inner as *const PackageRef as *mut PackageRef) }
    }
}

// if this wasn't used in events it could contain a reference to alpm. Sadly this is not the case.
/// A package in libalpm that can only be held by reference.
pub enum PackageRef {}

impl fmt::Debug for PackageRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Package(\"{}\")", self.name())
    }
}

impl PackageRef {
    pub(crate) unsafe fn new<'b>(p: *const Struct_alpm_pkg) -> &'b PackageRef {
        &*(p as *const PackageRef as *mut PackageRef)
    }

    /// Checks package integrity using md5. Returns true on success.
    pub fn check_md5(&self) -> bool {
        //unsafe { println!("{:?}, {:?}", self.0, alpm_pkg_checkmd5sum(self.0)); }
        unsafe { alpm_pkg_checkmd5sum(self as *const _ as _) == 0 }
    }

    /// Gets a list of all packages that require this package.
    pub fn compute_required_by(&self) -> Vec<String> {
        unsafe {
            let pkg_list = alpm_pkg_compute_requiredby(self as *const _ as _);
            let pkgs = util::alpm_list_to_vec(pkg_list, |char_ptr| {
                CStr::from_ptr(char_ptr as *const c_char).to_str().unwrap().to_owned()
            });
            alpm_list_free_inner(pkg_list, Some(libc::free));
            alpm_list_free(pkg_list);
            pkgs
        }
    }

    /// Gets a list of all packages optionally require this package.
    pub fn compute_optional_for(&self) -> Vec<String> {
        unsafe {
            let pkg_list = alpm_pkg_compute_optionalfor(self as *const _ as _);
            let pkgs = util::alpm_list_to_vec(pkg_list, |char_ptr| {
                CStr::from_ptr(char_ptr as *const c_char).to_str().unwrap().to_owned()
            });
            alpm_list_free_inner(pkg_list, Some(libc::free));
            alpm_list_free(pkg_list);
            pkgs
        }
    }

    /// Should this package be ignored when upgrading (as set on the alpm handle).
    pub fn should_ignore(&self, alpm: &Alpm) -> bool {
        unsafe {
            let res = alpm_pkg_should_ignore(alpm.handle, self as *const _ as _);
            res != 0
        }
    }

    /// Get the name of the file containing this package.
    pub fn filename(&self) -> &str {
        unsafe {
            let fname = alpm_pkg_get_filename(self as *const _ as _);
            assert!(!fname.is_null());
            CStr::from_ptr(fname).to_str().unwrap()
        }
    }

    /// Gets the package base name, if it has one.
    pub fn base(&self) -> Option<&str> {
        unsafe {
            let char_ptr = alpm_pkg_get_base(self as *const _ as _);
            if char_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(char_ptr).to_str().unwrap())
            }
        }
    }

    /// Gets the package name.
    pub fn name(&self) -> &str {
        unsafe {
            let char_ptr = alpm_pkg_get_name(self as *const _ as _);
            assert!(!char_ptr.is_null());
            CStr::from_ptr(char_ptr).to_str().unwrap()
        }
    }

    /// Gets the package version.
    pub fn version<'a>(&'a self) -> PackageVersion<'a> {
        unsafe {
            let v = alpm_pkg_get_version(self as *const _ as _);
            PackageVersion::new(v)
        }
    }

    /// Gets the origin of the package.
    pub fn origin(&self) -> PackageFrom {
        unsafe { alpm_pkg_get_origin(self as *const _ as _).into() }
    }

    /// Gets the package description.
    pub fn description(&self) -> &str {
        unsafe {
            let char_ptr = alpm_pkg_get_desc(self as *const _ as _);
            assert!(!char_ptr.is_null());
            CStr::from_ptr(char_ptr).to_str().unwrap()
        }
    }

    /// Gets the package url.
    pub fn url(&self) -> &str {
        unsafe {
            let char_ptr = alpm_pkg_get_url(self as *const _ as _);
            assert!(!char_ptr.is_null());
            CStr::from_ptr(char_ptr).to_str().unwrap()
        }
    }

    /// Gets the package build date.
    pub fn build_date(&self) -> NaiveDate {
        unsafe {
            let time = alpm_pkg_get_builddate(self as *const _ as _);
            NaiveDateTime::from_timestamp(time, 0).date()
        }
    }

    /// Gets the install timestamp of this package.
    pub fn install_date(&self) -> Option<NaiveDate> {
        unsafe {
            let time = alpm_pkg_get_installdate(self as *const _ as _);
            if time == 0 {
                None
            } else {
                Some(NaiveDateTime::from_timestamp(time, 0).date())
            }
        }
    }

    /// Gets the packager's name
    pub fn packager(&self) -> &str {
        unsafe {
            let char_ptr = alpm_pkg_get_packager(self as *const _ as _);
            assert!(!char_ptr.is_null()); // safety first
            CStr::from_ptr(char_ptr).to_str().unwrap()
        }
    }

    /// Gets the md5 checksum for this package.
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

    /// Gets the sha256 checksum for this package.
    pub fn sha256(&self) -> Option<&str> {
        unsafe {
            let str_ptr = alpm_pkg_get_sha256sum(self as *const _ as _);
            if str_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(str_ptr).to_str().unwrap()) // cannot fail
            }
        }
    }

    /// Gets the architecture for which this package was built.
    pub fn arch(&self) -> &str {
        unsafe {
            let char_ptr = alpm_pkg_get_arch(self as *const _ as _);
            assert!(!char_ptr.is_null()); // safety first
            CStr::from_ptr(char_ptr).to_str().unwrap()
        }
    }

    /// Gets the size of the package on a sync database.
    pub fn remote_size(&self) -> u64 {
        unsafe { alpm_pkg_get_size(self as *const _ as _) as u64 }
    }

    /// Returns the size of the package when it is installed.
    pub fn local_size(&self) -> u64 {
        unsafe { alpm_pkg_get_isize(self as *const _ as _) as u64 }
    }

    /// Gets the reason this package was installed.
    pub fn reason(&self) -> Reason {
        unsafe { alpm_pkg_get_reason(self as *const _ as _).into() }
    }

    /// Gets the licenses for this package
    pub fn licenses(&self) -> Vec<&str> {
        unsafe {
            let licenses = alpm_pkg_get_licenses(self as *const _ as _);
            util::alpm_list_to_vec(licenses, |char_ptr| {
                CStr::from_ptr(char_ptr as *const c_char).to_str().unwrap()
            })
        }
    }

    /// Gets the groups this package belongs to.
    pub fn groups(&self) -> Vec<&str> {
        unsafe {
            let groups = alpm_pkg_get_groups(self as *const _ as _);
            util::alpm_list_to_vec(groups, |char_ptr| {
                CStr::from_ptr(char_ptr as *const c_char).to_str().unwrap()
            })
        }
    }

    /// Gets the packages this package depends on.
    pub fn depends<'a>(&'a self) -> Vec<Dependency<'a>> {
        unsafe {
            let deps = alpm_pkg_get_depends(self as *const _ as _);
            util::alpm_list_to_vec(deps, |dep| {
                Dependency::new(dep as *const alpm_depend_t)
            })
        }
    }

    /// Gets the packages this package optionally depends on.
    pub fn optionally_depends<'a>(&'a self) -> Vec<Dependency<'a>> {
        unsafe {
            let deps = alpm_pkg_get_optdepends(self as *const _ as _);
            util::alpm_list_to_vec(deps, |dep| {
                Dependency::new(dep as *const alpm_depend_t)
            })
        }
    }

    /*
    /// Gets the packages required to check this package.
    pub fn check_depends<'a>(&'a self) -> Vec<Dependency<'a>> {
        unsafe {
            let deps = alpm_pkg_get_checkdepends(self as *const _ as _);
            util::alpm_list_to_vec(deps, |dep| {
                Dependency::new(dep as *const alpm_depend_t)
            })
        }
    }

    /// Gets the packages required to make (build) this package.
    pub fn make_depends<'a>(&'a self) -> Vec<Dependency<'a>> {
        unsafe {
            let deps = alpm_pkg_get_makedepends(self as *const _ as _);
            util::alpm_list_to_vec(deps, |dep| {
                Dependency::new(dep as *const alpm_depend_t)
            })
        }
    }
    */

    /// Gets the packages this package conflicts with.
    pub fn conflicts<'a>(&'a self) -> Vec<Dependency<'a>> {
        unsafe {
            let deps = alpm_pkg_get_conflicts(self as *const _ as _);
            util::alpm_list_to_vec(deps, |dep| {
                Dependency::new(dep as *const alpm_depend_t)
            })
        }
    }

    /// Gets the packages provided by this package.
    pub fn provides<'a>(&'a self) -> Vec<Dependency<'a>> {
        unsafe {
            let deps = alpm_pkg_get_provides(self as *const _ as _);
            util::alpm_list_to_vec(deps, |dep| {
                Dependency::new(dep as *const alpm_depend_t)
            })
        }
    }

    /// Gets the available deltas for this package.
    pub fn deltas(&self) -> Vec<&str> {
        unsafe {
            let deltas = alpm_pkg_get_deltas(self as *const _ as _);
            util::alpm_list_to_vec(deltas, |char_ptr| {
                CStr::from_ptr(char_ptr as *const c_char).to_str().unwrap()
            })
        }
    }

    /// Gets a list of packages to be replaced by this package.
    pub fn replaces<'a>(&'a self) -> Vec<Dependency<'a>> {
        unsafe {
            let deps = alpm_pkg_get_replaces(self as *const _ as _);
            util::alpm_list_to_vec(deps, |dep| {
                Dependency::new(dep as *const alpm_depend_t)
            })
        }
    }

    /// Gets a list of files installed by this package.
    pub fn files<'a>(&'a self) -> FileList<'a> {
        unsafe { FileList::from_raw(alpm_pkg_get_files(self as *const _ as _)) }
    }

    /// Gets a list of files backed up when installing this package.
    pub fn backup<'a>(&self) -> Vec<Backup<'a>> {
        unsafe {
            let backups = alpm_pkg_get_backup(self as *const _ as _);
            util::alpm_list_to_vec(backups, |bkup| Backup {
                name: CStr::from_ptr((*(bkup as *const alpm_backup_t)).name).to_str().unwrap(),
                hash: CStr::from_ptr((*(bkup as *const alpm_backup_t)).hash).to_str().unwrap(),
            })
        }
    }

    /*
    /// Gets the database this package is from
    ///
    /// This is currently unimplemented as it is not clear if there are times when it would be
    /// useful to get the underlying database.
    pub fn db(&self) {
        unimplemented!()
    }
    */

    /// Get thie base64 encoded package signature.
    pub fn base64_signature(&self) -> &str {
        unsafe {
            let char_ptr = alpm_pkg_get_packager(self as *const _ as _);
            assert!(!char_ptr.is_null()); // safety first
            CStr::from_ptr(char_ptr).to_str().unwrap()
        }
    }

    /// Gets the method used to validate a package during install
    pub fn validation(&self) -> Validation {
        unsafe { alpm_pkg_get_validation(self as *const _ as _).into() }
    }

    /// Opens the changelog for reading
    pub fn changelog(&self) {
        unimplemented!()
    }

    // changelog_read and changelog_close belong on changelog struct

    /// Gets the package's mtree.
    pub fn mtree(&self) {
        unimplemented!()
    }

    /// Returns true if the package has an install scriptlet, false if not.
    pub fn has_scriptlet(&self) -> bool {
        unsafe { alpm_pkg_has_scriptlet(self as *const _ as _) != 0 }
    }

    /// Gets the size of the download required to install this package, or to upgrade this package
    /// to the version of `self`.
    pub fn download_size(&self) -> u64 {
        unsafe { alpm_pkg_download_size(self as *const _ as _) as u64 }
    }

    /// Dont know what this is for
    ///
    /// TODO I'm guessing the return type as it's not in the alpm.h docs. I'm setting it to match
    /// `deltas`. check for segfaults.
    pub fn unused_deltas(&self) -> Vec<&str> {
        unsafe {
            let deltas = alpm_pkg_get_licenses(self as *const _ as _);
            util::alpm_list_to_vec(deltas, |char_ptr| {
                CStr::from_ptr(char_ptr as *const c_char).to_str().unwrap()
            })
        }
    }

    /// Sets the reason for a package in the local database. Returns true if reason was
    /// successfully changed, false otherwise.
    ///
    /// Use `Alpm.error` to get an error on failure.
    pub fn set_reason(&self, r: Reason) -> bool {
        unsafe { alpm_pkg_set_reason(self as *const _ as _, r.into()) == 0 }
    }

    /// (As part of overall transaction) Checks for new version of this package in sync repos.
    ///
    /// Returns the first newer version found
    pub fn sync_new_version<'a>(&self, dbs: Vec<Db<'a>>) -> Option<&'a PackageRef> {
        unsafe {
            let dbs = util::vec_to_alpm_list(dbs, |db| db.inner as *const libc::c_void);
            let new_pkg_ptr = alpm_sync_newversion(self as *const _ as _, dbs);
            if new_pkg_ptr.is_null() {
                None
            } else {
                Some(PackageRef::new(new_pkg_ptr))
            }
        }
    }
}

/// An operation on packages in this database
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

/// A group of related pacakges in a database
pub struct Group<'a> {
    pub name: &'a str,
    pub packages: Vec<&'a PackageRef>,
}

impl<'a> fmt::Debug for Group<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Group(\"{}\")", self.name)
    }
}

impl<'a> Group<'a> {
    pub(crate) unsafe fn new<'b>(name: *const c_char, packages: *const alpm_list_t) -> Group<'b> {
        let name = CStr::from_ptr(name).to_str().unwrap(); //probably should't fail
        let packages = util::alpm_list_to_vec(packages, |pkg_ptr| {
            &*(pkg_ptr as *const PackageRef)
        });
        Group {
            name: name,
            packages: packages,
        }
    }
}

/// A package version
pub struct PackageVersion<'a>(*const c_char, PhantomData<&'a u8>);

impl<'b> PackageVersion<'b> {
    fn new<'a>(p: *const c_char) -> PackageVersion<'a> {
        PackageVersion(p, PhantomData)
    }
}

impl<'a> AsRef<str> for PackageVersion<'a> {
    fn as_ref(&self) -> &str {
        unsafe { CStr::from_ptr(self.0).to_str().unwrap() }
    }
}

impl<'a> fmt::Display for PackageVersion<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<'a> PartialEq for PackageVersion<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            CStr::from_ptr(self.0) == CStr::from_ptr(other.0)
        }
    }
}

impl<'a> Eq for PackageVersion<'a> { }

impl<'a> cmp::Ord for PackageVersion<'a> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let res = unsafe { alpm_pkg_vercmp(self.0, other.0) };
        match res {
            x if x > 0 => cmp::Ordering::Greater,
            x if x < 0 => cmp::Ordering::Less,
            _ => cmp::Ordering::Equal,
        }
    }
}

impl<'a> cmp::PartialOrd for PackageVersion<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

#[test]
fn test_cmp_pkg_version() {
    use std::ffi::CString;
    unsafe {
        let less = CString::new("1.0").unwrap();
        let greater = CString::new("1.1").unwrap();
        let less_v = PackageVersion::new(less.as_ptr());
        let greater_v = PackageVersion::new(greater.as_ptr());
        assert!(less_v < greater_v);
        assert!(!(less_v >= greater_v));
        assert!(greater_v > less_v);
        assert!(!(greater_v <= less_v));
        assert!(greater_v <= greater_v);
        assert!(!(greater_v > greater_v));
        assert!(greater_v >= greater_v);
        assert!(!(greater_v < greater_v));
    }
}

/// Where a package came from
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PackageFrom {
    File,
    LocalDb,
    SyncDb,
}

impl Into<u32> for PackageFrom {
    fn into(self) -> u32 {
        match self {
            PackageFrom::File => ALPM_PKG_FROM_FILE,
            PackageFrom::LocalDb => ALPM_PKG_FROM_LOCALDB,
            PackageFrom::SyncDb => ALPM_PKG_FROM_SYNCDB,
        }
    }
}

impl From<u32> for PackageFrom {
    fn from(f: u32) -> Self {
        match f {
            ALPM_PKG_FROM_FILE => PackageFrom::File,
            ALPM_PKG_FROM_LOCALDB => PackageFrom::LocalDb,
            ALPM_PKG_FROM_SYNCDB => PackageFrom::SyncDb,
            _ => unreachable!()
        }
    }
}

/// Why a package was installed, either because it was explicitally requested, or required as a
/// dependency.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Reason {
    /// Package was explicitally installed by the user.
    Explicit,
    /// Package was installed because it was a dependency of another package.
    Depend
}

impl Into<u32> for Reason {
    fn into(self) -> u32 {
        match self {
            Reason::Explicit => ALPM_PKG_REASON_EXPLICIT,
            Reason::Depend => ALPM_PKG_REASON_DEPEND,
        }
    }
}

impl From<u32> for Reason {
    fn from(f: u32) -> Self {
        match f {
            ALPM_PKG_REASON_EXPLICIT => Reason::Explicit,
            ALPM_PKG_REASON_DEPEND  => Reason::Depend,
            _ => unreachable!()
        }
    }
}

/// The methods used to validate a package when it was downloaded.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ValidationMethod {
    /// Package was validated using md5
    pub md5sum: bool,
    /// Package was validated using sha256
    pub sha256sum: bool,
    /// Package was validated using gpg signature
    pub signature: bool,
}

impl Default for ValidationMethod {
    fn default() -> ValidationMethod {
        ValidationMethod {
            md5sum: false,
            sha256sum: false,
            signature: false,
        }
    }
}

/// Whether validation was performed.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Validation {
    /// It is not known whether validation was performed.
    Unknown,
    /// No validation was performed
    None,
    /// 1 or more validation methods were performed.
    Some(ValidationMethod)
}

impl Validation {
    /// Creates a `ValidationMethod` for when a package validation method is not known.
    pub fn unknown() -> Validation {
        Validation::Unknown
    }

    /// Creates a `ValidationMethod` for when no validation was performed.
    pub fn none() -> Validation {
        Validation::None
    }

    /// Creates a `ValidationMethod` for when md5sum validation was performed.
    pub fn md5sum() -> Validation {
        Validation::Some(ValidationMethod { md5sum: true, ..Default::default() })
    }

    /// Creates a `ValidationMethod` for when sha256sum validation was performed.
    pub fn sha256sum() -> Validation {
        Validation::Some(ValidationMethod { sha256sum: true, ..Default::default() })
    }

    /// Creates a `ValidationMethod` for when gpg signature validation was performed.
    pub fn signature() -> Validation {
        Validation::Some(ValidationMethod { signature: true, ..Default::default() })
    }
}

impl Into<u32> for Validation {
    fn into(self) -> u32 {
        match self {
            Validation::Unknown => ALPM_PKG_VALIDATION_UNKNOWN,
            Validation::None => ALPM_PKG_VALIDATION_NONE,
            Validation::Some(methods) => {
                let mut acc = 0;
                if methods.md5sum {
                    acc |= ALPM_PKG_VALIDATION_MD5SUM;
                };
                if methods.sha256sum {
                    acc |= ALPM_PKG_VALIDATION_SHA256SUM;
                };
                if methods.signature {
                    acc |= ALPM_PKG_VALIDATION_SIGNATURE;
                };
                acc
            }
        }
    }
}

impl From<u32> for Validation {
    fn from(from: u32) -> Validation {
        match from {
            ALPM_PKG_VALIDATION_UNKNWON => Validation::Unknown,
            ALPM_PKG_VALIDATION_NONE => Validation::None,
            other => Validation::Some(ValidationMethod {
                md5sum: from & ALPM_PKG_VALIDATION_MD5SUM != 0,
                sha256sum: from & ALPM_PKG_VALIDATION_SHA256SUM != 0,
                signature: from & ALPM_PKG_VALIDATION_SIGNATURE != 0,
            }),
        }
    }
}

/// A package's dependency
pub struct Dependency<'a> {
    name: *const c_char,
    version: PackageVersion<'a>,
    description: *const c_char,
    name_hash: c_ulong,
    version_constraint_type: VersionConstraintType
}

impl<'a> fmt::Debug for Dependency<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dependency(\"{}\")", self.name())
    }
}

impl<'a> Dependency<'a> {
    unsafe fn new<'b>(raw: *const alpm_depend_t) -> Dependency<'b> {
        Dependency {
            name: (*raw).name,
            version: PackageVersion::new((*raw).version),
            description: (*raw).desc,
            name_hash: (*raw).name_hash,
            version_constraint_type: (*raw).mod_.into()
        }
    }

    /// Gets the name of the dependency
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap() }
    }

    /// Gets the version of the dependency
    pub fn version(&self) -> &PackageVersion<'a> {
        &self.version
    }

    /// Gets a description of the dependency
    pub fn description(&self) -> &str {
        unsafe { CStr::from_ptr(self.description).to_str().unwrap() }
    }

    /// Gets a hash of the dependency's name
    pub fn hash(&self) -> u64 {
        self.name_hash as u64
    }

    /// Gets the version restriction type
    pub fn version_constraint_type(&self) -> VersionConstraintType {
        self.version_constraint_type
    }
}

/// Types of version constraint to be applied to a package's dependency.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VersionConstraintType {
    Any,
    Equal,
    GreaterOrEqual,
    LessOrEqual,
    Greater,
    Less,
}

impl From<u32> for VersionConstraintType {
    fn from(f: u32) -> VersionConstraintType {
        match f {
            ALPM_DEP_MOD_ANY => VersionConstraintType::Any,
            ALPM_DEP_MOD_EQ => VersionConstraintType::Equal,
            ALPM_DEP_MOD_GE => VersionConstraintType::GreaterOrEqual,
            ALPM_DEP_MOD_LE => VersionConstraintType::LessOrEqual,
            ALPM_DEP_MOD_GT => VersionConstraintType::Greater,
            ALPM_DEP_MOD_LT => VersionConstraintType::Less,
            _ => unreachable!()
        }
    }
}

/// A list of files in a package
#[derive(Debug, PartialEq, Eq)]
pub struct FileList<'a> {
    /// Thie files this filelist contains
    pub list: Vec<File<'a>>,
    inner: *const alpm_filelist_t,
}

impl<'a> FileList<'a> {
    pub(crate) unsafe fn from_raw<'b>(raw: *const alpm_filelist_t) -> FileList<'b> {
        let count = (*raw).count;
        let mut file_ptr = (*raw).files;
        let mut files: Vec<File<'b>> = Vec::new();
        for i in 0..count {
            files.push(File::new(file_ptr));
            file_ptr = file_ptr.offset(1);
        }
        FileList {
            list: files,
            inner: raw,
        }
    }

    /// Test to see whether this filelist contains a path given.
    ///
    /// The path should be relative to the installation root without preceeding slash (e.g.
    /// `etc/pacman.conf`). Directories should have a trailing slash (e.g. `etc/`)
    pub fn contains(&self, path: &str) -> Option<File<'a>> {
        unsafe {
            let path = CString::new(path).unwrap();
            let file_ptr = alpm_filelist_contains(self.inner, path.as_ptr());
            if file_ptr.is_null() {
                None
            } else {
                Some(File::new(file_ptr))
            }
        }
    }
}

/// A file in a package
#[derive(Debug, PartialEq, Eq)]
pub struct File<'a> {
    /// The filename.
    pub name: &'a str,
    /// The size of the file in bytes.
    pub size: u64,
    /// The file mode. e.g. 0644 would be urw,gr,or
    pub mode: u32,
}

impl<'a> File<'a> {
    pub(crate) unsafe fn new<'b>(file_ptr: *const alpm_file_t) ->  File<'b> {
        File {
            name: CStr::from_ptr((*file_ptr).name).to_str().unwrap(),
            size: (*file_ptr).size as u64,
            mode: (*file_ptr).mode,
        }
    }
}

/// A backup
#[derive(Debug, PartialEq, Eq)]
pub struct Backup<'a> {
    pub name: &'a str,
    pub hash: &'a str,
}
