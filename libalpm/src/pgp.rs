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

impl From<SigLevel> for u32 {
    fn from(from: SigLevel) -> u32 {
        (&from).into()
    }
}

impl<'a> From<&'a SigLevel> for u32 {
    fn from(from: &'a SigLevel) -> u32 {
        use alpm_sys::alpm_siglevel_t::*;
        let mut acc = 0;
        if from.package {
            acc |= ALPM_SIG_PACKAGE as u32;
        };
        if from.package_optional {
            acc |= ALPM_SIG_PACKAGE_OPTIONAL as u32;
        };
        if from.package_marginal_ok {
            acc |= ALPM_SIG_PACKAGE_MARGINAL_OK as u32;
        };
        if from.package_unknown_ok {
            acc |= ALPM_SIG_PACKAGE_UNKNOWN_OK as u32;
        };
        if from.database {
            acc |= ALPM_SIG_DATABASE as u32;
        };
        if from.database_optional {
            acc |= ALPM_SIG_DATABASE_OPTIONAL as u32;
        };
        if from.database_marginal_ok {
            acc |= ALPM_SIG_DATABASE_MARGINAL_OK as u32;
        }
        if from.database_unknown_ok {
            acc |= ALPM_SIG_DATABASE_UNKNOWN_OK as u32;
        };
        if from.use_default {
            acc |= ALPM_SIG_USE_DEFAULT as u32;
        };
        acc
    }
}

impl From<u32> for SigLevel {
    fn from(from: u32) -> SigLevel {
        use alpm_siglevel_t::*;
        SigLevel {
            package: from & ALPM_SIG_PACKAGE as u32 != 0,
            package_optional: from & ALPM_SIG_PACKAGE_OPTIONAL as u32 != 0,
            package_marginal_ok: from & ALPM_SIG_PACKAGE_MARGINAL_OK as u32 != 0,
            package_unknown_ok: from & ALPM_SIG_PACKAGE_UNKNOWN_OK as u32 != 0,
            database: from & ALPM_SIG_DATABASE as u32 != 0,
            database_optional: from & ALPM_SIG_DATABASE_OPTIONAL as u32 != 0,
            database_marginal_ok: from & ALPM_SIG_DATABASE_MARGINAL_OK as u32 != 0,
            database_unknown_ok: from & ALPM_SIG_DATABASE_UNKNOWN_OK as u32 != 0,
            use_default: from & ALPM_SIG_USE_DEFAULT as u32 != 0,
        }
    }
}
