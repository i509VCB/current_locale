//! # os-locale
//!
//! os-locale is a library for obtaining a user's current locale from the OS.
//! os-locale provides methods to get the user's preferred language in a form similar to a BCP47 (IETF) compliant
//! language code.
//!
//! Obtaining the user's current locale is widely useful for displaying a user's preferred language
//! in applications.
//!

// Platform implementations

#[cfg(any(
target_os = "android",
// target_os = "ios" // Untested
))]
compile_error!("You are compiling for an unimplemented platform!\nContributions are welcome to os-locale to implement any new platforms.");

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform;

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[path = "apple.rs"]
mod platform;

#[cfg(all(
	unix,
	not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
#[path = "unix.rs"]
mod platform;

#[cfg(target_os = "android")]
#[path = "android.rs"]
mod platform;

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
	OsLookupFailure,
}

// Tests

#[test]
fn parse_runtime_locale() {
	let locale = get_user_locale();

	match locale {
		Ok(locale) => println!("Got locale code {}", locale),
		Err(error) => match error {
			OsLocaleError::NotIetfCompliant(s) => {
				panic!("Got locale code {} but it is not IETF compliant!", s)
			}
			OsLocaleError::OsLookupFailure => panic!("Failed to look up locale code from the OS!"),
		},
	}
}
