extern crate libalpm;
extern crate libalpm_utils;

use libalpm_utils::ini::parse_ini;
use libalpm::{Alpm, TransactionError};
use std::process;

fn main() {
    // Load config
    let conf = parse_ini("/etc/pacman.conf").unwrap();
    let alpm = Alpm::with_config(&conf).unwrap();

    // Update sync dbs
    for db in alpm.sync_dbs().iter() {
        db.update(false).unwrap();
    }

    // Run upgrade transaction
    let trans = alpm.init_transaction(Default::default()).unwrap();
    trans.sys_upgrade(true).unwrap();
    let trans = match trans.prepare() {
        Ok(t) => t,
        Err(TransactionError::NothingToDo(_)) => {
            println!("There are no packages that need updating");
            process::exit(0);
        }
        Err(e) => panic!(format!("{:?}", e)),
    };

    print!("Packages that need adding/updating: ");
    for pkg in trans.added_packages() {
        print!("{}, ", pkg.name());
    }
    println!();

    print!("Package that will be removed: ");
    for pkg in trans.removed_packages() {
        print!("{}, ", pkg.name());
    }
    println!();
}
