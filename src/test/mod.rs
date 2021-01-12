use crate::{get_user_locale, OsLocaleError};

#[cfg(all(target_os = "linux", not(target_os = "android")))]
use crate::platform::format_raw_user_lang;

#[test]
fn parse_runtime_locale() {
	let locale = get_user_locale();

	match locale {
		Ok(locale) => println!("Got locale code {}", locale),
		Err(error) => match error {
			OsLocaleError::NotIetfCompliant(s) => panic!("Got locale code {} but it is not IETF compliant!", s),
			OsLocaleError::OsLookupFailure => panic!("Failed to look up locale code from the OS!")
		}
	}
}

#[test]
#[cfg(all(target_os = "linux", not(target_os = "android")))]
fn test_linux_locales() {
	// Test a few locales and if they parse correctly
	assert_eq!(format_raw_user_lang("aa_DJ.UTF-8 UTF-8".to_string()).unwrap(), "aa-DJ");
	assert_eq!(format_raw_user_lang("aa_DJ ISO-8859-1".to_string()).unwrap(), "aa-DJ");
	assert_eq!(format_raw_user_lang("aa_ER@saaho UTF-8".to_string()).unwrap(), "aa-ER");
	assert_eq!(format_raw_user_lang("ar_QA ISO-8859-6".to_string()).unwrap(), "ar-QA");
	assert_eq!(format_raw_user_lang("en_GB.UTF-8 UTF-8".to_string()).unwrap(), "en-GB");
	assert_eq!(format_raw_user_lang("ko_KR.EUC-KR EUC-KR".to_string()).unwrap(), "ko-KR");
	assert_eq!(format_raw_user_lang("zh_CN.GB18030 GB18030".to_string()).unwrap(), "zh-CN");
}
