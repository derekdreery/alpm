//! Logging in libalpm

use std::default::Default;
use std::fmt;
use std::cmp::{self, Ordering};

use alpm_sys::*;

/// The highest log level marked true
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Debug,
    Function,
    None
}

impl cmp::Ord for LogLevel {
    fn cmp(&self, other: &LogLevel) -> Ordering {
        match *self {
            LogLevel::Error => match *other {
                LogLevel::Error => Ordering::Equal,
                _ => Ordering::Greater,
            },
            LogLevel::Warning => match *other {
                LogLevel::Error => Ordering::Less,
                LogLevel::Warning => Ordering::Equal,
                _ => Ordering::Greater,
            },
            LogLevel::Debug => match *other {
                LogLevel::Error | LogLevel::Warning => Ordering::Less,
                LogLevel::Debug => Ordering::Equal,
                _ => Ordering::Greater,
            },
            LogLevel::Function => match *other {
                LogLevel::None => Ordering::Greater,
                LogLevel::Function => Ordering::Equal,
                _ => Ordering::Less,
            },
            LogLevel::None => match *other {
                LogLevel::None => Ordering::Equal,
                _ => Ordering::Less,
            }
        }
    }
}

impl cmp::PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &LogLevel) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Function => write!(f, "FUNCTION"),
            LogLevel::None => write!(f, "NONE"),
        }
    }
}

/// The possible log-levels
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LogLevels {
    pub error: bool,
    pub warning: bool,
    pub debug: bool,
    pub function: bool
}

impl fmt::Display for LogLevels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let single: LogLevel = self.clone().into();
        write!(f, "{}", single)
    }
}

impl Default for LogLevels {
    fn default() -> LogLevels {
        LogLevels {
            error: false,
            warning: false,
            debug: false,
            function: false,
        }
    }
}

impl LogLevels {
    /// Error loglevel
    pub fn error() -> LogLevels {
        LogLevels { error: true, ..Default::default() }
    }

    /// Warning loglevel
    pub fn warning() -> LogLevels {
        LogLevels { warning: true, ..Default::default() }
    }

    /// Debug loglevel
    pub fn debug() -> LogLevels {
        LogLevels { debug: true, ..Default::default() }
    }

    /// Function loglevel
    pub fn function() -> LogLevels {
        LogLevels { function: true, ..Default::default() }
    }
}

impl Into<u32> for LogLevels {
    fn into(self) -> u32 {
        use alpm_sys::alpm_loglevel_t::*;
        let mut acc = 0;
        if self.error {
            acc |= ALPM_LOG_ERROR as u32;
        };
        if self.warning {
            acc |= ALPM_LOG_WARNING as u32;
        };
        if self.debug {
            acc |= ALPM_LOG_DEBUG as u32;
        };
        if self.function {
            acc |= ALPM_LOG_FUNCTION as u32;
        };
        acc
    }
}

impl From<u32> for LogLevels {
    fn from(from: u32) -> LogLevels {
        use alpm_loglevel_t::*;
        LogLevels {
            error: from & ALPM_LOG_ERROR as u32 != 0,
            warning: from & ALPM_LOG_WARNING as u32 != 0,
            debug: from & ALPM_LOG_DEBUG as u32 != 0,
            function: from & ALPM_LOG_FUNCTION as u32 != 0,
        }
    }
}

impl From<alpm_loglevel_t> for LogLevels {
    fn from(from: alpm_loglevel_t) -> LogLevels {
        LogLevels::from(from as u32)
    }
}

impl Into<LogLevel> for LogLevels {
    fn into(self) -> LogLevel {
        if self.error { LogLevel::Error }
        else if self.warning { LogLevel::Warning }
        else if self.debug { LogLevel::Debug }
        else if self.function { LogLevel::Function }
        else { LogLevel::None }
    }
}

