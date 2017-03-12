use std::ffi::{CStr, CString};
use std::str::Utf8Error;
use std::mem;

use alpm_sys::*;
use pgp::SigLevel;
use libc::{self, c_char};

use {Alpm, AlpmResult, Error, PackageRef, Group};
use util::{self, alpm_list_to_vec, vec_to_alpm_list, str_to_unowned_char_array,
    cstring_to_owned_char_array};

/// A database of packages. This is only ever available as a reference
#[derive(Debug)]
pub struct Db<'a> {
    inner: *const Struct_alpm_db,
    // we need this handle so we can get error codes
    handle: &'a Alpm,
}

impl<'a> Db<'a> {

    pub(crate) fn new(inner: *const Struct_alpm_db, handle: &'a Alpm) -> Db<'a> {
        Db {
            inner: inner,
            handle: handle,
        }
    }

    /// Gets the name of the database.
    pub fn name(&self) -> Result<&'a str, Utf8Error> {
        unsafe {
            CStr::from_ptr(alpm_db_get_name(self.inner)).to_str()
        }
    }

    /// Gets the signature checking level of the database.
    pub fn siglevel(&self) -> SigLevel {
        unsafe { alpm_db_get_siglevel(self.inner).into() }
    }

    /// Checks the database is valid. If not, an error
    pub fn is_valid(&self) -> AlpmResult<()> {
        if unsafe { alpm_db_get_valid(self.inner) == 0 } {
            Ok(())
        } else {
            Err(self.handle.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Gets a list of the servers used by the database.
    pub fn servers(&self) -> Vec<&str> {
        unsafe {
            let servers = alpm_db_get_servers(self.inner);
            alpm_list_to_vec(servers, |char_ptr| {
                CStr::from_ptr(char_ptr as *const c_char).to_str().unwrap()
            })
        }
    }

    /// Sets the servers used by the database.
    pub fn set_servers<R>(&self, servers: Vec<R>) -> AlpmResult<()>
        where R: AsRef<str>
    {
        unsafe {
            let list = vec_to_alpm_list(servers, |s| unsafe {
                str_to_unowned_char_array(&s.as_ref())
            });
            let res = alpm_db_set_servers(self.inner, list);
            if res == 0 {
                Ok(())
            } else {
                Err(self.handle.error().unwrap_or(Error::__Unknown))
            }
        }
    }

    /// Adds a server to the list of servers used by the database.
    pub fn add_server(&self, url: &str) -> AlpmResult<()> {
        let url = CString::new(url)?;
        if unsafe { alpm_db_add_server(self.inner, url.as_ptr()) } == 0 {
            Ok(())
        } else {
            Err(self.handle.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Removes a server from the list of servers used by the database.
    pub fn remove_server(&self, url: &str) -> AlpmResult<()> {
        let url = CString::new(url)?;
        if unsafe { alpm_db_remove_server(self.inner, url.as_ptr()) } == 0 {
            Ok(())
        } else {
            Err(self.handle.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Update (sync) the database with remote.
    ///
    /// If force is set to `false` and the database is up to date the function will return
    /// successfully without doing anything.
    pub fn update(&self, force: bool) -> AlpmResult<()> {
        let force = if force { 1 } else { 0 };
        if unsafe { alpm_db_update(force, self.inner) } == 0 {
            Ok(())
        } else {
            if let Some(err) = self.handle.error() {
                Err(err)
            } else {
                // Update not needed (and force == false)
                Ok(())
            }
        }
    }

    /// Gets a package with the given name from the database
    pub fn pkg(&self, name: &str) -> AlpmResult<&'a PackageRef> {
        let name = CString::new(name)?;
        let pkg_ptr = unsafe { alpm_db_get_pkg(self.inner, name.as_ptr()) };
        if pkg_ptr.is_null() {
            Err(self.handle.error().unwrap_or(Error::__Unknown))
        } else {
            //println!("{:?}", pkg_ptr);
            Ok( unsafe { PackageRef::new(pkg_ptr) })
        }
    }

    /// Gets all packages in the db cache
    pub fn pkg_cache(&self) -> Vec<&PackageRef> {
        unsafe {
            let cache_ptr = alpm_db_get_pkgcache(self.inner);
            alpm_list_to_vec(cache_ptr, |pkg_ptr| {
                &*(pkg_ptr as *const PackageRef)
            })
        }
    }

    /// Gets a package group from the database by name.
    pub fn group(&self, name: &str) -> AlpmResult<Group<'a>> {
        unsafe {
            let name = CString::new(name).unwrap();
            let group_ptr = alpm_db_get_group(self.inner, name.as_ptr());
            if group_ptr.is_null() {
                Err(self.handle.error().unwrap_or(Error::__Unknown))
            } else {
                let group_ptr = group_ptr as *const alpm_group_t;
                Ok(Group {
                    name: CStr::from_ptr((*group_ptr).name).to_str().unwrap(),
                    packages: alpm_list_to_vec((*group_ptr).packages, |pkg_ptr| {
                        &*(pkg_ptr as *const PackageRef)
                    }),
                })
            }
        }
    }

    /// Gets the package group cache of the database.
    pub fn group_cache(&self) -> AlpmResult<Vec<Group<'a>>> {
        unsafe {
            let group_cache = alpm_db_get_groupcache(self.inner);
            if group_cache.is_null() {
                Err(self.handle.error().unwrap_or(Error::__Unknown))
            } else {
                Ok(alpm_list_to_vec(group_cache, |group_ptr| {
                    let group_ptr = group_ptr as *const alpm_group_t;
                    Group {
                        name: CStr::from_ptr((*group_ptr).name).to_str().unwrap(),
                        packages: alpm_list_to_vec((*group_ptr).packages, |pkg_ptr| {
                            &*(pkg_ptr as *const PackageRef)
                        }),
                    }
                }))
            }
        }
    }

    /// Searches the database for packages matching the needles.
    ///
    /// This function has a memory leak, but I'm 99% sure it's internal to libalpm. Needs more
    /// testing.
    pub fn search(&self, needles: Vec<&str>) -> AlpmResult<Vec<&PackageRef>> {
        let needles_outer: Vec<CString> = needles.iter()
            .map(|s| CString::new(*s).unwrap())
            .collect();
        println!("{:?}", needles_outer);
        unsafe {
            let needles = util::vec_as_alpm_list(&needles_outer, cstring_to_owned_char_array);
            let pkgs = alpm_db_search(self.inner, needles);
            alpm_list_free(needles);
            if ! pkgs.is_null() {
                Ok(alpm_list_to_vec(pkgs, |pkg_ptr| &*(pkg_ptr as *mut PackageRef )))
            } else {
                Err(self.handle.error().unwrap_or(Error::__Unknown))
            }
        }
    }

    /// Sets what this database is to be used for.
    pub fn set_usage(&self, usage: Usage) -> AlpmResult<()> {
        if unsafe { alpm_db_set_usage(self.inner, usage.into()) } == 0 {
            Ok(())
        } else {
            Err(self.handle.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Gets what this database is to be used for.
    pub fn usage(&self) -> AlpmResult<Usage> {
        unsafe {
            let usage: u32 = mem::zeroed();
            if alpm_db_get_usage(self.inner, &usage) == 0 {
                Ok(usage.into())
            } else {
                Err(self.handle.error().unwrap_or(Error::__Unknown))
            }
        }
    }
}

/// A struct to say what to use a given database for
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Usage {
    pub sync: bool,
    pub search: bool,
    pub install: bool,
    pub upgrade: bool,
}

impl Usage {
    #[inline(always)]
    pub fn sync() -> Usage { Usage { sync: true, ..Default::default() } }
    #[inline(always)]
    pub fn search() -> Usage { Usage { search: true, ..Default::default() } }
    #[inline(always)]
    pub fn install() -> Usage { Usage { install: true, ..Default::default() } }
    #[inline(always)]
    pub fn upgrade() -> Usage { Usage { upgrade: true, ..Default::default() } }
    #[inline(always)]
    pub fn all() -> Usage {
        Usage {
            sync: true,
            search: true,
            install: true,
            upgrade: true,
        }
    }
}

impl Default for Usage {
    fn default() -> Usage {
        Usage {
            sync: false,
            search: false,
            install: false,
            upgrade: false,
        }
    }
}

impl Into<u32> for Usage {
    fn into(self) -> u32 {
        let mut acc = 0;
        if self.sync {
            acc |= ALPM_DB_USAGE_SYNC;
        };
        if self.search {
            acc |= ALPM_DB_USAGE_SEARCH;
        };
        if self.install {
            acc |= ALPM_DB_USAGE_INSTALL;
        };
        if self.upgrade {
            acc |= ALPM_DB_USAGE_UPGRADE;
        };
        acc
    }
}

impl From<u32> for Usage {
    fn from(from: u32) -> Self {
        Usage {
            sync: from & ALPM_DB_USAGE_SYNC != 0,
            search: from & ALPM_DB_USAGE_SEARCH != 0,
            install: from & ALPM_DB_USAGE_INSTALL != 0,
            upgrade: from & ALPM_DB_USAGE_UPGRADE != 0,
        }
    }
}
