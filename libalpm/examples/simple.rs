extern crate libalpm;

fn main() {
    let alpm = libalpm::Alpm::new("/", "/var/lib/pacman").unwrap();
    println!("Root: {}", alpm.root());
    println!("Database path: {}", alpm.db_path());
    println!("Lockfile: {}", alpm.lockfile());
}
