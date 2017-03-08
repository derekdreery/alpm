//! Logging in libalpm

use std::default::Default;
use std::fmt;

use alpm_sys::*;

/// The highest log level marked true
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Debug,
    Function,
    None
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
        let mut acc = 0;
        if self.error {
            acc |= ALPM_LOG_ERROR;
        };
        if self.warning {
            acc |= ALPM_LOG_WARNING;
        };
        if self.debug {
            acc |= ALPM_LOG_DEBUG;
        };
        if self.function {
            acc |= ALPM_LOG_FUNCTION;
        };
        acc
    }
}

impl From<u32> for LogLevels {
    fn from(from: u32) -> LogLevels {
        LogLevels {
            error: from & ALPM_LOG_ERROR != 0,
            warning: from & ALPM_LOG_WARNING != 0,
            debug: from & ALPM_LOG_DEBUG != 0,
            function: from & ALPM_LOG_FUNCTION != 0,
        }
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

