use crate::OsLocaleError;

pub(crate) fn _get_user_locale() -> Result<String, OsLocaleError> {
	// Unix-like OSes use the LANG environment variable to store the locale
	if let Ok(raw_value) = std::env::var("LANG") {
		// Unset locale - C ANSI standards say default to en_US
		if raw_value == "C" {
			return Ok(String::from("en-US"));
		}

		if let Ok(result) = raw_unix_to_locale(raw_value) {
			return Ok(result);
		}
	}

	Err(OsLocaleError::OsLookupFailure)
}

pub(crate) fn raw_unix_to_locale(raw: String) -> Result<String, OsLocaleError> {
	/*
	 * Find one of the following to split off the lang code:
	 * First index of `.` as in `en_US.UTF_8`
	 * A space which separates generic code from char set.
	 * Terminate at an `@` which specifies a locale at a specific location
	 */
	if let Some(pos) = raw.find(|c| c == ' ' || c == '.') {
		let (raw_lang_code, _) = raw.split_at(pos);
		let result = raw_lang_code.replace("_", "-");

		// Finally replace underscores with `-` and drop everything after an `@`
		return Ok(result.split('@').next().unwrap().to_string());
	}

	Err(OsLocaleError::NotIetfCompliant(raw))
}
