use std::ffi::{CStr, CString};
use std::str::Utf8Error;

use alpm_sys::*;
use pgp::SigLevel;
use {Alpm, AlpmResult, Error, PackageRef};

/// A database of packages. This is only ever available as a reference
pub struct Db<'a> {
    inner: *const Struct_alpm_db,
    // we need this handle so we can get error codes
    handle: &'a Alpm,
}

impl<'a> Db<'a> {

    pub fn new(inner: *const Struct_alpm_db, handle: &'a Alpm) -> Db<'a> {
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
    pub fn servers(&self) {
        unimplemented!()
    }

    /// Sets the servers used by the database.
    pub fn set_servers(&self) {
        unimplemented!()
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

    pub fn update(&self, force: bool) -> AlpmResult<()> {
        let force = if force { 1 } else { 0 };
        if unsafe { alpm_db_update(force, self.inner) } == 0 {
            Ok(())
        } else {
            Err(self.handle.error().unwrap_or(Error::__Unknown))
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
    pub fn pkg_cache(&self) -> AlpmResult<()> {
        let pkg_ptr = unsafe { alpm_db_get_pkgcache(self.inner) };
        if pkg_ptr.is_null() {
            Err(self.handle.error().unwrap_or(Error::__Unknown))
        } else {
            unimplemented!()
        }
    }
}
