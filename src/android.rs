#![cfg(target_os = "android")]

use crate::LocaleError;

pub(crate) fn current_locale() -> Result<String, LocaleError> {
    unimplemented!("Android platform is not implemented yet!")
}
