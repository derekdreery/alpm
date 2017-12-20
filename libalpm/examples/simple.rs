extern crate libalpm;
extern crate term;

use std::io::Write;
use libalpm::{LogLevel, LogLevels};

fn log(level: LogLevels, msg: String) {
    let mut t = match term::stdout() {
        Some(t) => t,
        None => {
            return;
        }
    };
    let color = match level.into() {
        LogLevel::Error => term::color::RED,
        LogLevel::Warning => term::color::YELLOW,
        LogLevel::Debug => term::color::GREEN,
        _ => term::color::BLACK,
    };
    t.fg(color).unwrap();
    write!(t, "{}: {}", level, msg).unwrap();
    t.reset().unwrap();
    t.flush().unwrap();
}

fn download(filename: &str, transferred: u64, total: u64) {
    println!("{}: {}/{}", filename, transferred, total);
}

fn main() {
    let arch = libalpm::util::uname().machine().to_owned();
    let alpm = libalpm::Alpm::new("/", "/var/lib/pacman").unwrap();

    // write a log function that colors output based on level
    alpm.log_function(log);
    alpm.file_download_progress_function(download);
    println!("Root: {}", alpm.root());
    println!("Database path: {}", alpm.db_path());
    println!("Lockfile: {}", alpm.lockfile());
    println!(
        "{:?}",
        libalpm::util::get_servers("/etc/pacman.d/mirrorlist", "core", &arch)
    );
}
