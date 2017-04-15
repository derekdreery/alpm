use std::path::Path;
use std::collections::HashMap;

/// The options that can be set in the pacman conf file.
#[derive(Debug)]
pub struct Config {
    /// The root directory of the instance. Packages are installed relative to here.
    pub root_dir: String,
    /// The location of the synced databases.
    pub db_path: String,
    /// The location of the cache directory.
    pub cache_dirs: Vec<String>,
    /// The location of the log file.
    pub log_file: String,
    /// The location of the gpg directory.
    pub gpg_dir: String,
    pub hook_dirs: Vec<String>,
    pub hold_pkg: Vec<String>,
    pub transfer_command: Option<String>,
    //pub clean_method: TODO,
    pub use_delta: f32,
    pub arch: String,
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
    pub repositories: HashMap<String, RepoConfig>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            root_dir: "/".into(),
            db_path: "/var/lib/pacman/".into(),
            cache_dirs: vec!["/var/cache/pacman/pkg/".into()],
            log_file: "/var/log/pacman.log".into(),
            gpg_dir: "/etc/pacman.d/gnupg/".into(),
            hook_dirs: vec!["/etc/pacman.d/hooks/".into()],
            hold_pkg: vec![],
            transfer_command: None,
            //clean_method
            use_delta: 0.7,
            arch: "auto".into(),
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
            repositories: HashMap::new(),
        }
    }

}

/// Config for a repository.
#[derive(Debug, Default)]
pub struct RepoConfig {
    /// A vector containing urls for the repository's mirrors.
    pub servers: Vec<String>
}

impl RepoConfig {
    fn new(servers: Vec<String>) -> RepoConfig {
        RepoConfig { servers }
    }
}
