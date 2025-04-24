#![cfg(any(target_os = "macos", target_os = "ios"))]

use crate::LocaleError;
use objc2::rc::autoreleasepool;
use objc2::runtime::AnyObject;
use objc2::{class, msg_send};
use objc2_foundation::NSString;

pub(crate) fn current_locale() -> Result<String, LocaleError> {
    // https://developer.apple.com/documentation/foundation/nslocale/1409990-currentlocale?language=objc
    let nslocale = class!(NSLocale);
    let identifier = unsafe {
        let current: *const AnyObject = msg_send![nslocale, currentLocale]; // [NSLocale currentLocale]
        let identifier: *const NSString = msg_send![current, localeIdentifier]; // [currentLocale localIdentifier]
        identifier.as_ref().unwrap()
    };

    // Convert to IETF
    // FIXME: Not every locale code is ietf compliant?
    let locale_string = autoreleasepool(|pool| {
        // SAFETY: The str is not used outside the autorelease pool.
        unsafe { identifier.to_str(pool).replace("_", "-") }
    });
    Ok(locale_string)
}

#[cfg(test)]
mod test {
    use objc2::{class, msg_send, rc::autoreleasepool};
    use objc2_foundation::{NSArray, NSString};

    #[test]
    fn list_available() {
        let nslocale = class!(NSLocale);

        let available = unsafe {
            let available: *const NSArray<NSString> =
                msg_send![nslocale, availableLocaleIdentifiers];
            available.as_ref().unwrap()
        };

        for locale in available.to_vec() {
            let locale_string = autoreleasepool(|pool| {
                // SAFETY: The str is not used outside the autorelease pool.
                unsafe { locale.to_str(pool).replace("_", "-") }
            });
            println!("{}", locale_string);
        }
    }
}
