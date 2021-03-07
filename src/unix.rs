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

#[test]
fn test_linux_locales() {
	// Test a few locales and if they parse correctly
	assert_eq!(
		raw_unix_to_locale("aa_DJ.UTF-8 UTF-8".to_string()).unwrap(),
		"aa-DJ"
	);
	assert_eq!(
		raw_unix_to_locale("aa_DJ ISO-8859-1".to_string()).unwrap(),
		"aa-DJ"
	);
	assert_eq!(
		raw_unix_to_locale("aa_ER@saaho UTF-8".to_string()).unwrap(),
		"aa-ER"
	);
	assert_eq!(
		raw_unix_to_locale("ar_QA ISO-8859-6".to_string()).unwrap(),
		"ar-QA"
	);
	assert_eq!(
		raw_unix_to_locale("en_GB.UTF-8 UTF-8".to_string()).unwrap(),
		"en-GB"
	);
	assert_eq!(
		raw_unix_to_locale("ko_KR.EUC-KR EUC-KR".to_string()).unwrap(),
		"ko-KR"
	);
	assert_eq!(
		raw_unix_to_locale("zh_CN.GB18030 GB18030".to_string()).unwrap(),
		"zh-CN"
	);
}
