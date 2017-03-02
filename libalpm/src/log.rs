//! Logging in libalpm

use std::default::Default;

use alpm_sys::*;

type RustLogFn = Fn(LogLevel, &mut fmt::Formatter);

/// The possible log-levels
pub struct LogLevel {
    pub error: bool,
    pub warning: bool,
    pub debug: bool,
    pub function: bool
}

impl Default for LogLevel {
    fn default() -> LogLevel {
        LogLevel {
            error: false,
            warning: false,
            debug: false,
            function: false,
        }
    }
}

impl LogLevel {
    /// Error loglevel
    pub fn error() -> LogLevel {
        LogLevel { error: true, ..Default::default() }
    }

    /// Warning loglevel
    pub fn warning() -> LogLevel {
        LogLevel { warning: true, ..Default::default() }
    }

    /// Debug loglevel
    pub fn debug() -> LogLevel {
        LogLevel { debug: true, ..Default::default() }
    }

    /// Function loglevel
    pub fn function() -> LogLevel {
        LogLevel { function: true, ..Default::default() }
    }
}

impl Into<u32> for LogLevel {
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

impl From<u32> for LogLevel {
    fn from(from: u32) -> LogLevel {
        LogLevel {
            error: from & ALPM_LOG_ERROR != 0,
            warning: from & ALPM_LOG_WARNING != 0,
            debug: from & ALPM_LOG_DEBUG != 0,
            function: from & ALPM_LOG_FUNCTION != 0,
        }
    }
}

