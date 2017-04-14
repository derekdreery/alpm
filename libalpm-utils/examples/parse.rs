extern crate libalpm_utils;

use libalpm_utils::ini::parse_ini;

fn main() {
    println!("{:#?}", parse_ini("/etc/pacman.conf"));
}
