extern crate libalpm_utils;

use libalpm_utils::ini::lex_ini;

fn main() {
    println!("{:#?}", lex_ini("/etc/pacman.conf"));
}
