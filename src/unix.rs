#![cfg(all(
    unix,
    not(any(target_os = "android", target_os = "ios", target_os = "macos"))
))]

use crate::{ErrorKind, LocaleError};
use std::env;

pub(crate) fn current_locale() -> Result<String, LocaleError> {
    // Unix uses the LANG environment variable to store the locale
    match env::var("LANG") {
        Ok(raw_lang) => {
            // Unset locale - C ANSI standards say default to en-US
            if raw_lang == "C" {
                Ok("en-US".to_owned())
            } else {
                Ok(normalize_to_ietf(&raw_lang)?)
            }
        }

        Err(e) => Err(LocaleError {
            kind: ErrorKind::LookupFailed,
            description: Some(e.to_string()),
        }),
    }
}

/// Normalizes a unix locale value to an ietf compliant language code.
fn normalize_to_ietf(raw: &str) -> Result<String, LocaleError> {
    /*
     * Find one of the following to split off the lang code:
     * First index of `.` as in `en_US.UTF_8`
     * A space which separates generic code from char set.
     * Terminate at an `@` which specifies a locale at a specific location
     */
    if let Some(pos) = raw.find([' ', '.']) {
        let (raw_lang_code, _) = raw.split_at(pos);
        let result = raw_lang_code.replace("_", "-");

        // Finally replace underscores with `-` and drop everything after an `@`
        return Ok(result.split('@').next().unwrap().to_string());
    }

    Err(ErrorKind::NotIetfCompliant(raw.to_owned()).into())
}

#[cfg(test)]
mod test {
    use super::normalize_to_ietf;
    use std::error::Error;

    #[test]
    fn test_linux_locales() -> Result<(), Box<dyn Error>> {
        // Test a few locales and if they parse correctly
        assert_eq!(normalize_to_ietf("aa_DJ.UTF-8 UTF-8")?, "aa-DJ");
        assert_eq!(normalize_to_ietf("aa_DJ ISO-8859-1")?, "aa-DJ");
        assert_eq!(normalize_to_ietf("aa_ER@saaho UTF-8")?, "aa-ER");
        assert_eq!(normalize_to_ietf("ar_QA ISO-8859-6")?, "ar-QA");
        assert_eq!(normalize_to_ietf("en_GB.UTF-8 UTF-8")?, "en-GB");
        assert_eq!(normalize_to_ietf("ko_KR.EUC-KR EUC-KR")?, "ko-KR");
        assert_eq!(normalize_to_ietf("zh_CN.GB18030 GB18030")?, "zh-CN");

        Ok(())
    }
}
