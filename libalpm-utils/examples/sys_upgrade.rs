extern crate libalpm_utils;
extern crate libalpm;

use libalpm_utils::ini::parse_ini;
use libalpm::{Alpm};

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
    let trans = trans.prepare().unwrap();
    trans.commit().unwrap();
}
