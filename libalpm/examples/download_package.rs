extern crate libalpm;
extern crate curl;
extern crate url;

use std::fs::File;
use std::path::Path;
use std::io::Write;

use libalpm::{Alpm, DownloadResult};
use curl::easy::Easy;
use url::Url;

fn fetch(url: &str, loc: &str, force: bool) -> DownloadResult {
    let url = url.parse::<Url>().unwrap();
    let path = url.path_segments().unwrap().last().unwrap_or("".into());
    let filename = match url.query() {
        Some(q) => format!("{}?{}", path, q),
        None => path.into()
    };
    let mut file = File::create(Path::new(&filename)).unwrap();
    let mut easy = Easy::new();
    easy.url(url.as_str()).unwrap();
    easy.write_function(move |data| {
        Ok(file.write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
    DownloadResult::Ok
}

fn main() {
    let alpm = Alpm::new("/", "/var/lib/pacman").unwrap();
    unsafe { alpm.fetch_function(fetch); }
    let dl_loc = alpm.fetch_pkgurl("http://archlinux.polymorf.fr/extra/os/x86_64/a2ps");
    println!("{:?}", dl_loc);
}
