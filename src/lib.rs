#![warn(missing_docs)]

//! # os-locale
//!
//! os-locale is a library for obtaining a user's current locale from the OS.
//! os-locale provides methods to get the user's preferred language in a form similar to a BCP47 (IETF) compliant
//! language code.
//!
//! Obtaining the user's current locale is widely useful for displaying a user's preferred language
//! in applications.

use std::{error::Error, fmt::Display};

// Platform implementations

#[cfg(any(target_os = "android",))]
compile_error!("You are compiling for an unimplemented platform!\nContributions are welcome to os-locale to implement any new platforms.");

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod imp;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "apple.rs"]
mod imp;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
#[path = "unix.rs"]
mod imp;

#[cfg(target_os = "android")]
#[path = "android.rs"]
mod imp;

/// Returns the current locale of the process or an error.
///
/// Depending on the operating system, the current locale is one of the following in order of decreasing priority:
/// - The locale of the current process.
/// - The locale of the current user.
/// - The fallback locale of the operating system.
///
/// The returned string is a BCP47 (IETF) compliant language code.
///
/// # Platform Specific Behavior
///
/// **Unix:** If the user's locale is not set, it will typically be set to `C` by default.
/// Per the C standard library, the `C` language code is U.S. English ASCII which is `en-US`.
///
/// **Windows:** If the system is a single user system, then this method will default to the system's
/// default language code rather than the user's set language code.
///
pub fn current_locale() -> Result<String, LocaleError> {
    imp::current_locale()
}

/// An error that may occur when looking up the current locale.
#[derive(Debug)]
pub struct LocaleError {
    kind: ErrorKind,
    description: Option<String>,
}

impl LocaleError {
    /// Returns the kind of the error.
    pub fn kind(self) -> ErrorKind {
        self.kind
    }
}

impl Display for LocaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(description) = &self.description {
            write!(f, "{}: {}", self.kind, description)
        } else {
            Display::fmt(&self.kind, f)
        }
    }
}

impl Error for LocaleError {}

/// A kind of error that may occur when looking up the current locale.
#[derive(Debug)]
pub enum ErrorKind {
    /// An error which indicates the returned language code is not IETF compliant.
    NotIetfCompliant(String),

    /// An error which represents a failure by the OS to look up the user's locale.
    ///
    /// # Platform Specific Behavior
    ///
    /// **Unix**: `LANG` variable is not set.
    LookupFailed,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorKind::NotIetfCompliant(_) => "locale code returned is not ietf compliant",
                ErrorKind::LookupFailed => "getting locale from system failed",
            }
        )
    }
}

impl Error for ErrorKind {}

impl From<ErrorKind> for LocaleError {
    fn from(kind: ErrorKind) -> Self {
        LocaleError {
            kind,
            description: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{current_locale, ErrorKind};

    #[test]
    fn parse_runtime_locale() {
        let locale = current_locale();

        match locale {
            Ok(locale) => println!("Got locale code {}", locale),
            Err(error) => match error.kind() {
                ErrorKind::NotIetfCompliant(s) => {
                    panic!("Got locale code {} but it is not IETF compliant!", s)
                }

                ErrorKind::LookupFailed => panic!("Failed to look up locale code from the OS!"),
            },
        }
    }
}
