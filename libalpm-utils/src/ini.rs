
use std::fs;
use std::io;
use std::io::prelude::*;

use nom::{IResult};

/// Library error type
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    /// Filename, line number, text
    Lex(String, usize, String),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

#[derive(Debug)]
pub enum Token {
    /// Start of a new section
    Header(String),
    /// A key/value pair
    Pair(String, String),
    /// A single value without key
    Single(String),
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
/// Parses a pacman-style INI config file.
///
/// # Params
///  - file path to the config file
///  - cb callback for key/value pairs
///  - data caller defined data to be passed to the callback
///
/// Returns the callback return value
///
/// The callback will be called at the beginning of each section with an
/// empty key and value and for each key/value pair.
///
/// If the parser encounters an error the callback will be called with
/// section, key, and value set to NULL and errno set by fopen, fgets, or
/// strdup.
///
/// The key and value passed to cb will be overwritten between
/// calls. The section name will remain valid until after cb is called to
/// begin a new section.
///
/// Parsing will immediately stop if the callback returns non-zero.
///
pub fn lex_ini(filename: &str) -> Result<Vec<Token>, Error> {

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
                tok_list.append(&mut lex_ini(value)?);
            } else {
                tok_list.push(Token::Pair(key.into(), value.into()));
            }
        } else {
            tok_list.push(Token::Single(line.into()));
        }

    }

    Ok(tok_list)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
