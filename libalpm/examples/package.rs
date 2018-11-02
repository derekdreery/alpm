extern crate libalpm;
extern crate term;

use libalpm::{util, Alpm, Config, LogLevel, LogLevels, PackageRef, SigLevel};

fn log(level: LogLevels, msg: String) {
    let mut t = match term::stdout() {
        Some(t) => t,
        None => {
            return;
        }
    };
    let level = level.into();
    let color = match level {
        LogLevel::Error => term::color::RED,
        LogLevel::Warning => term::color::YELLOW,
        LogLevel::Debug => term::color::GREEN,
        _ => term::color::BLACK,
    };
    if level >= LogLevel::Debug {
        t.fg(color).unwrap();
        write!(t, "{}: {}", level, msg).unwrap();
        t.reset().unwrap();
        t.flush().unwrap();
    }
}

fn download(filename: &str, transferred: u64, total: u64) {
    println!("{}: {}/{}", filename, transferred, total);
}

fn print_pkg_info(pkg: &PackageRef) {
    println!("In search(gcc), found {:?}, info:", pkg);
    println!("         name: {:?}", pkg.name());
    //println!("         required by: {:?}", pkg.compute_required_by());
    //println!("         optional for: {:?}", pkg.compute_optional_for());
    println!("         base: {:?}", pkg.base());
    println!("         check_md5: {:?}", pkg.check_md5());
    println!("         filename: {}", pkg.filename());
    println!("         version: {}", pkg.version());
    println!("         origin: {:?}", pkg.origin());
    println!("         description: {:?}", pkg.description());
    println!("         url: {:?}", pkg.url());
    println!("         build date: {:?}", pkg.build_date());
    println!("         install date: {:?}", pkg.install_date());
    println!("         packager: {:?}", pkg.packager());
    println!("         md5: {:?}", pkg.md5());
    println!("         sha256: {:?}", pkg.sha256());
    println!("         arch: {:?}", pkg.arch());
    println!("         remote size: {:?}", pkg.remote_size());
    println!("         local size: {:?}", pkg.local_size());
    println!("         reason: {:?}", pkg.reason());
    println!("         licenses: {:?}", pkg.licenses());
    println!("         groups: {:?}", pkg.groups());
    println!("         dependencies: {:?}", pkg.depends());
    println!("         optional dependencies: {:?}", pkg.optionally_depends());
    //println!("         check dependencies: {:?}", pkg.check_depends());
    //println!("         make dependencies: {:?}", pkg.make_depends());
    println!("         conflicts: {:?}", pkg.conflicts());
    println!("         provides: {:?}", pkg.provides());
    println!("         deltas: {:?}", pkg.deltas());
    println!("         replaces: {:?}", pkg.replaces());
    println!("         files: {:?}", pkg.files());
    println!("         validation: {:?}", pkg.validation());
}

fn main() {
    let arch = util::uname().machine().to_owned();
    println!("arch: {:?}", arch);
    let options = Config::default();
    let alpm = Alpm::new("./tmp", "./tmp/var/lib/pacman").unwrap();
    alpm.set_arch(&arch).unwrap();
    println!("arch: {:?}", alpm.arch());
    //panic!("bail");
    alpm.log_function(log);
    alpm.file_download_progress_function(download);
    let db = alpm.local_db();
    println!("name: {:?}", db.name());
    for pkg in db.pkg_cache() {
        println!("pkg: {:?}", pkg.name());
    }
    print!("check {} is valid...", db.name().unwrap());
    db.is_valid().unwrap();
    println!("OK");
    //println!("siglevel: {:?}", db.siglevel());
    //db.update(true).unwrap();
    //let mut pkg = db.pkg("gcc").unwrap();
    //println!("md5: {:?} - {:?}", pkg.md5(), pkg.check_md5());
    //println!("error: {:?}", alpm.error().unwrap().description());

    for (name, repo) in &options.repositories {
        let db = alpm.register_sync_db(name, &SigLevel::default()).unwrap();
        let mut fixed_servers = repo.servers
            .iter()
            .map(|el| el.replace("$arch", &arch).replace("$repo", name));
        db.add_server(&fixed_servers.next().unwrap());
        println!("  name: {:?}", db.name());
    }
    let dbs = alpm.sync_dbs();
    println!("Iter sync");
    for db in dbs.iter().take(1) {
        println!("  db name: {:?}, servers: {:?}", db.name(), db.servers());
        println!("    group cache: {:?}", db.group_cache());
        //println!("  Updating: {:?}", db.name());
        //db.update(false).unwrap();
        for pkg in db.search(&["gcc"]).unwrap() {
            print_pkg_info(pkg);
        }
    }
}
