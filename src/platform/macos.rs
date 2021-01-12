use crate::OsLocaleError;
use crate::OsLocaleError::OsLookupFailure;

pub(crate) fn _get_user_locale() -> Result<String, OsLocaleError> {
	// An implementation needs to look up the locale from Foundation:
	// https://developer.apple.com/documentation/foundation/nslocale/1409990-currentlocale?language=objc
	unimplemented!("Mac OS Platform is not implemented yet!")
}
