//! # os-locale
//!
//! os-locale is a library for obtaining a user's current locale from the OS.
//! os-locale provides methods to get the user's preferred language in a BCP47 (IETF) compliant
//! language code.
//!
//! Obtaining the user's current locale is widely useful for displaying a user's preferred language
//! in applications.
//!

// Platform implementations

#[cfg(any(
target_os = "macos",
target_os = "android",
target_os = "ios"
))]
compile_error!("You are compiling for an unimplemented platform!");

#[cfg(target_os = "windows")]
#[path = "platform/windows.rs"]
pub(crate) mod platform;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
pub(crate) mod platform;

#[cfg(all(target_os = "linux", not(target_os = "android")))]
#[path = "platform/linux.rs"]
pub(crate) mod platform;

#[cfg(target_os = "android")]
#[path = "platform/android.rs"]
pub(crate) mod platform;

#[cfg(target_os = "ios")]
#[path = "platform/ios.rs"]
pub(crate) mod platform;

#[cfg(test)]
mod test;

/// Gets the user's locale as a BCP47 (IETF) compliant language code
///
/// # Platform Specific Behavior
///
/// **Linux:** If the user's locale is not set, it will default to `C`.
/// Per the C standard library, the `C` language code is U.S. English ASCII which is `en-US`.
///
/// **Windows:** If the system is a single user system, then this method will default to the system's
/// default language code rather than the user's set language code.
///
/// # Returns
/// This function returns a result which either contains a language tag of the system's current
/// language a parsing error for the BCP47 language code if the language code is invalid.
///
pub fn get_user_locale() -> Result<String, OsLocaleError> {
	platform::_get_user_locale()
}

#[derive(Debug)]
pub enum OsLocaleError {
	/// An error which represents a language code obtained by the OS that is not IETF compliant.
	NotIetfCompliant(String),
	/// An error which represents a failure by the OS to look up the user's locale.
	///
	/// # Platform Specific Behavior
	/// **Linux** An [OsFailedLookup] error may occur if the `LANG` variable is not set.
	///
	OsLookupFailure
}
