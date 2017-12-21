
use std::fs;
use std::io;
use std::io::prelude::*;

use nom::{IResult};
use libalpm::{Config, RepoConfig};

/// Library error type
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    /// Filename, line number, text
    Parse(String, usize, String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

/// A token TODO include file/line no.
#[derive(Debug)]
enum Token {
    /// Start of a new section
    Header(String),
    /// A key/value pair
    Pair(String, String),
    /// A single key without value
    Valueless(String),
}

/// What section we are in
#[derive(Debug)]
enum Section {
    /// No section set yet
    None,
    /// In options section
    Options,
    /// In a repo section
    Repo(String),
}

/// Matches a header line
named!(parse_header<&str, &str>, do_parse!(
    tag!("[") >>
    inner: take_until!("]") >>
    tag!("]") >>
    (inner.trim())
));

/// Matches a key and the '='
named!(parse_key<&str, &str>, do_parse!(
    key: take_until!("=") >>
    tag!("=") >>
    (key.trim())
));

///
/// Lexes a pacman-style INI config file.
///
/// Returns a list of tokens. `Include` directives are expanded.
///
fn lex_ini(filename: &str) -> Result<Vec<Token>, Error> {

    let mut tok_list = Vec::new();

    let config_file = fs::File::open(filename)?;
    let config_reader = io::BufReader::new(config_file);


    for line in config_reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            // skip comments
        } else if let IResult::Done(_, name) = parse_header(&line) {
            tok_list.push(Token::Header(name.into()))
        } else if let IResult::Done(value, key) = parse_key(&line) {
            let value = value.trim();
            if key == "Include" {
                // Then we include the tokens from the filename in value into the token list
                // silently ignore if file doesn't exist
                match lex_ini(value) {
                    Ok(mut v) => { tok_list.append(&mut v) },
                    Err(Error::Io(_)) => (), // ignore
                    Err(e) => { return Err(e); }
                }
            } else {
                tok_list.push(Token::Pair(key.into(), value.into()));
            }
        } else {
            tok_list.push(Token::Valueless(line.into()));
        }

    }

    Ok(tok_list)
}

/// Parses an ini file into an `Config` object.
pub fn parse_ini(filename: &str) -> Result<Config, Error> {
    let mut section = Section::None;
    let mut options = Config::default();
    let toks = lex_ini(filename)?;
    for tok in toks.into_iter() {
        parse_token(tok, &mut options, &mut section)?;
    }
    Ok(options)
}

/// Parses a single token a token
fn parse_token(tok: Token, conf: &mut Config, section: &mut Section) -> Result<(), Error> {
    match &tok {
        &Token::Header(ref name) if name == "Options" || name == "options" => {
            *section = Section::Options;
        }
        &Token::Header(ref name) => {
            *section = Section::Repo(name.clone());
            // log error if section has already been processed
            if let Some(_) = conf.repositories.insert(name.clone(), RepoConfig::default()) {
                println!("Repository \"{}\" has already been added - replacing", name);
            }
        }
        &Token::Pair(ref key, ref value) => match section {
            &mut Section::None => {
                println!("Key \"{}\" found before any section header - ignoring", key);
            }
            &mut Section::Options => parse_pair_option(&key, &value, conf),
            &mut Section::Repo(ref repo_name) => parse_pair_repo(&repo_name, &key, &value, conf),
        },
        &Token::Valueless(ref key) => match section {
            &mut Section::None => {
                println!("Key \"{}\" found before any section header - ignoring", key);
            }
            &mut Section::Options => parse_valueless_option(&key, conf),
            &mut Section::Repo(..) => (),
        }
    };
    Ok(())
}

/// Parses a key-value pair in the *Options* section.
fn parse_pair_option(key: &str, value: &str, config: &mut Config) {
    if key == "NoUpgrade" {
        config.no_upgrade.append(&mut split_whitespace(value));
    } else if key == "NoExtract" {
        config.no_extract.append(&mut split_whitespace(value));
    } else if key == "IgnorePkg" {
        config.ignore_pkg.append(&mut split_whitespace(value));
    } else if key == "IgnoreGroup" {
        config.ignore_group.append(&mut split_whitespace(value));
    } else if key == "HoldPkg" {
        config.hold_pkg.append(&mut split_whitespace(value));
    } else if key == "CacheDir" {
        config.cache_dirs.append(&mut split_whitespace(value));
    } else if key == "HookDir" {
        config.hook_dirs.append(&mut split_whitespace(value));
    } else if key == "Architecture" {
        // TODO their lib uses first, this uses last
        config.arch = value.into();
    } else if key == "UseDelta" {
        if let Ok(parsed_val) = value.parse::<f32>() {
            let parsed_val = match parsed_val {
                v if v > 2.0 => {
                    println!("Delta set above 2.0, clamping");
                    2.0
                }
                v if v < 0.0 => {
                    println!("Delta set below 0.0, clamping");
                    0.0
                }
                v => v
            };
            config.use_delta = parsed_val;
        } else {
            println!("Cannot parse \"{}\" as float for UseDelta, ignoring", value);
        }
    } else if key == "DBPath" {
        config.db_path = value.into();
    } else if key == "RootDir" {
        config.root_dir = value.into();
    } else if key == "GPGDir" {
        config.gpg_dir = value.into();
    } else if key == "LogFile" {
        config.log_file = value.into();
    } else if key == "XferCommand" {
        config.transfer_command = Some(value.into());
    } else if key == "CleanMethod" {
        for method in value.split_whitespace() {
            if method == "KeepInstalled" {
                // TODO
            } else if method == "KeepCurrent" {
                // TODO
            } else {
                println!("Unrecognised clean method: \"{}\".", method)
            }
        }
        // TODO
    } else if key == "SigLevel" {
        // TODO
    } else if key == "LocalFileSigLevel" {
        // TODO
    } else if key == "RemoteFileSigLevel" {
        // TODO
    } else {
        println!("Unrecognised options key: \"{}\" = \"{}\".", key, value)
    }

}

/// Parses a valueless key in the *Options* section.
fn parse_valueless_option(key: &str, conf: &mut Config) {
    if key == "UseSyslog" {
        conf.use_syslog = true;
    } else if key == "ILoveCandy" {
        // ??
    } else if key == "VerbosePkgLists" {
        conf.verbose_pkg_lists = true;
    } else if key == "UseDelta" {
        conf.use_delta = 0.7;
    } else if key == "TotalDownload" {
        conf.total_download = true;
    } else if key == "CheckSpace" {
        conf.check_space = true;
    } else if key == "Color" {
        // todo check if we are a tty
    } else {
        println!("Unrecognised valueless option: {}.", key)
    }
}

fn parse_pair_repo(repo: &str, key: &str, value: &str, conf: &mut Config) {
    if key == "Server" {
        conf.repositories.get_mut(repo).unwrap().servers.push(value.into());
    } else if key == "" {
        conf.repositories.get_mut(repo).unwrap().sig_level = None;
    } else {
        println!("Unrecognised repo key in repo \"{}\": \"{}\" = \"{}\".", repo, key, value)
    }
}

/// Our own split_whitespace to make sure we own the strings
#[inline]
fn split_whitespace(s: &str) -> Vec<String> {
    let v: Vec<String> = s.split_whitespace().map(|el| el.into()).collect();
    v
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
