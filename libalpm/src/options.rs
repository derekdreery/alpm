use std::path::Path;

const MIRROR: &'static str = "http://archlinux.polymorf.fr/$repo/os/$arch";

pub struct Options {
    pub root_dir: String,
    pub db_path: String,
    pub cache_dir: String,
    pub log_file: String,
    pub gpg_dir: String,
    pub hook_dir: String,
    pub hold_pkg: Vec<String>,
    //pub XferCommand
    //pub clean_method: TODO,
    //pub use_delta: TODO,
    //pub architecture: TODO,
    pub ignore_pkg: Vec<String>,
    pub ignore_group: Vec<String>,
    pub no_upgrade: Vec<String>,
    pub no_extract: Vec<String>,
    pub use_syslog: bool,
    pub color: bool,
    pub total_download: bool,
    pub check_space: bool,
    pub verbose_pkg_lists: bool,
    //pub sig_level: TODO,
    //pub local_files_sig_level: TODO,
    //pub remote_files_sig_level: TODO,
    pub repositories: Vec<RepoOptions>,
}

impl Options {
    /// Reads a pacman-style ini file and returns an Options instance to match
    ///
    /// TODO will only be implemented after I've finished the rest of the lib
    pub fn from_ini(loc: &Path) -> Option<Options> {
        unimplemented!()
    }
}


impl Default for Options {
    fn default() -> Options {
        Options {
            root_dir: "/".into(),
            db_path: "/var/lib/pacman/".into(),
            cache_dir: "/var/cache/pacman/pkg/".into(),
            log_file: "/var/log/pacman.log".into(),
            gpg_dir: "/etc/pacman.d/gnupg/".into(),
            hook_dir: "/etc/pacman.d/hooks/".into(),
            hold_pkg: vec!["pacman".into(), "glibc".into()],
            // xfer_command
            //clean_method
            //use_delta: TODO,
            //architecture: TODO,
            ignore_pkg: vec![],
            ignore_group: vec![],
            no_upgrade: vec![],
            no_extract: vec![],
            use_syslog: false,
            color: false,
            total_download: false,
            check_space: true,
            verbose_pkg_lists: false,
            //sig_level: TODO,
            //local_files_sig_level: TODO,
            //remote_files_sig_level: TODO,
            repositories: vec![
                RepoOptions::new("core", vec![MIRROR.into()]),
                RepoOptions::new("extra", vec![MIRROR.into()]),
                RepoOptions::new("community", vec![MIRROR.into()]),
                RepoOptions::new("multilib", vec![MIRROR.into()]),
            ],
        }
    }

}

pub struct RepoOptions {
    pub name: String,
    pub servers: Vec<String>
}

impl RepoOptions {
    fn new<Name: Into<String>>(name: Name, servers: Vec<String>) -> RepoOptions {
        RepoOptions { name: name.into(), servers: servers }
    }
}
