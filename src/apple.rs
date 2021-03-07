use crate::OsLocaleError;
use objc::runtime::Object;
// Yes I do need to import sel and sel_impl due to bugs with objc
use objc::{class, msg_send, sel, sel_impl};
use objc_foundation::{INSString, NSString};

pub(crate) fn _get_user_locale() -> Result<String, OsLocaleError> {
	// https://developer.apple.com/documentation/foundation/nslocale/1409990-currentlocale?language=objc
	let nslocale = class!(NSLocale);
	// Protocols galore to get the currentLocale then the localeIdentifier
	let current: *const Object = unsafe { msg_send![nslocale, currentLocale] };
	let identifier: *const NSString = unsafe { msg_send![current, localeIdentifier] };
	let identifier = unsafe { identifier.as_ref() }.unwrap();

	// Convert to IETF
	Ok(identifier.as_str().replace("_", "-"))
}

#[test]
fn list_available() {
	let nslocale = class!(NSLocale);
	let current: *const NSArray<NSString> =
		unsafe { msg_send![nslocale, availableLocaleIdentifiers] };

	let a = unsafe { current.as_ref() }.unwrap();

	for x in a.to_vec() {
		println!("{}", x.as_str());
	}
}
