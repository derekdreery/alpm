extern crate libalpm;
extern crate term;

use std::error::Error;
use libalpm::{Alpm, PackageRef, Options, SigLevel, LogLevels, LogLevel};

fn log(level: LogLevels, msg: String) {
    let mut t = match term::stdout() {
        Some(t) => t,
        None => { return; }
    };
    let level = level.into();
    let color = match level {
        LogLevel::Error => term::color::RED,
        LogLevel::Warning => term::color::YELLOW,
        LogLevel::Debug => term::color::GREEN,
        _ => term::color::BLACK,
    };
    if level >= LogLevel::None {
        t.fg(color).unwrap();
        write!(t, "{}: {}", level, msg).unwrap();
        t.reset().unwrap();
        t.flush().unwrap();
    }
}

fn download(filename: &str, transferred: u64, total: u64) {
    println!("{}: {}/{}", filename, transferred, total);
}

fn main() {
    let options = Options::default();
    let alpm = Alpm::new("./tmp", "./tmp/var/lib/pacman").unwrap();
    println!("arch: {:?}", alpm.arch());
    panic!("bail");
    alpm.log_function(log);
    alpm.file_download_progress_function(download);
    let db = alpm.local_db();
    println!("name: {:?}", db.name());
    //db.is_valid().unwrap();
    //println!("siglevel: {:?}", db.siglevel());
    //db.update(true).unwrap();
    //let mut pkg = db.pkg("gcc").unwrap();
    //println!("md5: {:?} - {:?}", pkg.md5(), pkg.check_md5());
    //println!("error: {:?}", alpm.error().unwrap().description());

    for repo in options.repositories {
        let db = alpm.register_sync_db(&repo.name, SigLevel::default()).unwrap();
        db.add_server(&repo.servers[0]).unwrap();
        println!("  name: {:?}", db.name());
    }
    let dbs = alpm.sync_dbs();
    println!("Iter sync");
    for db in dbs.iter() {
        println!("  Updating: {:?}", db.name());
        db.update(false).unwrap();
    }
}
