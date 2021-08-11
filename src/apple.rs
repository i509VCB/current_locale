#![cfg(any(target_os = "macos", target_os = "ios"))]

use crate::LocaleError;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl}; // sel_impl needs to be imported since `sel` does not export it within it's own scope
use objc_foundation::{INSString, NSString};

pub(crate) fn current_locale() -> Result<String, LocaleError> {
    // https://developer.apple.com/documentation/foundation/nslocale/1409990-currentlocale?language=objc
    let nslocale = class!(NSLocale);
    let identifier = unsafe {
        let current: *const Object = msg_send![nslocale, currentLocale]; // [NSLocale currentLocale]
        let identifier: *const NSString = msg_send![current, localeIdentifier]; // [currentLocale localIdentifier]
        identifier.as_ref().unwrap()
    };

    // Convert to IETF
    // FIXME: Not every locale code is ietf compliant?
    Ok(identifier.as_str().replace("_", "-"))
}

#[cfg(test)]
mod test {
    use objc::{class, msg_send, sel, sel_impl}; // sel_impl needs to be imported since `sel` does not export it within it's own scope
    use objc_foundation::{INSArray, INSString, NSArray, NSString};

    #[test]
    fn list_available() {
        let nslocale = class!(NSLocale);

        let available = unsafe {
            let available: *const NSArray<NSString> =
                msg_send![nslocale, availableLocaleIdentifiers];
            available.as_ref().unwrap()
        };

        for locale in available.to_vec() {
            println!("{}", locale.as_str());
        }
    }
}
