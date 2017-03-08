extern crate libalpm;

use std::error::Error;
use libalpm::{Alpm, PackageRef};

fn main() {
    let alpm = Alpm::new("/", "/var/lib/pacman").unwrap();
    let db = alpm.localdb();
    println!("name: {:?}", db.name());
    db.is_valid().unwrap();
    //println!("siglevel: {:?}", db.siglevel());
    //db.update(true).unwrap();
    let mut pkg = db.pkg("gcc").unwrap();
    println!("md5: {:?} - {:?}", pkg.md5(), pkg.check_md5());
    println!("error: {:?}", alpm.error().unwrap().description());
}
