use alpm_sys::*;

/// PGP signature verification options
#[derive(Debug)]
pub struct SigLevel {
    pub package: bool,
    pub package_optional: bool,
    pub package_marginal_ok: bool,
    pub package_unknown_ok: bool,
    pub database: bool,
    pub database_optional: bool,
    pub database_marginal_ok: bool,
    pub database_unknown_ok: bool,
    pub use_default: bool,
}

impl Default for SigLevel {
    fn default() -> SigLevel {
        SigLevel {
            package: false,
            package_optional: false,
            package_marginal_ok: false,
            package_unknown_ok: false,
            database: false,
            database_optional: false,
            database_marginal_ok: false,
            database_unknown_ok: false,
            use_default: true,
        }
    }
}

impl Into<u32> for SigLevel {
    fn into(self) -> u32 {
        (&self).into()
    }
}

impl<'a> Into<u32> for &'a SigLevel {
    fn into(self) -> u32 {
        let mut acc = 0;
        if self.package {
            acc |= ALPM_SIG_PACKAGE;
        };
        if self.package_optional {
            acc |= ALPM_SIG_PACKAGE_OPTIONAL;
        };
        if self.package_marginal_ok {
            acc |= ALPM_SIG_PACKAGE_MARGINAL_OK;
        };
        if self.package_unknown_ok {
            acc |= ALPM_SIG_PACKAGE_UNKNOWN_OK;
        };
        if self.database {
            acc |= ALPM_SIG_DATABASE;
        };
        if self.database_optional {
            acc |= ALPM_SIG_DATABASE_OPTIONAL;
        };
        if self.database_marginal_ok {
            acc |= ALPM_SIG_DATABASE_MARGINAL_OK;
        }
        if self.database_unknown_ok {
            acc |= ALPM_SIG_DATABASE_UNKNOWN_OK;
        };
        if self.use_default {
            acc |= ALPM_SIG_USE_DEFAULT;
        };
        acc
    }
}

impl From<u32> for SigLevel {
    fn from(from: u32) -> SigLevel {
        SigLevel {
            package: from & ALPM_SIG_PACKAGE != 0,
            package_optional: from & ALPM_SIG_PACKAGE_OPTIONAL != 0,
            package_marginal_ok: from & ALPM_SIG_PACKAGE_MARGINAL_OK != 0,
            package_unknown_ok: from & ALPM_SIG_PACKAGE_UNKNOWN_OK != 0,
            database: from & ALPM_SIG_DATABASE != 0,
            database_optional: from & ALPM_SIG_DATABASE_OPTIONAL != 0,
            database_marginal_ok: from & ALPM_SIG_DATABASE_MARGINAL_OK != 0,
            database_unknown_ok: from & ALPM_SIG_DATABASE_UNKNOWN_OK != 0,
            use_default: from & ALPM_SIG_USE_DEFAULT != 0,
        }
    }
}


