#![cfg(target_arch = "wasm32")]
use crate::LocaleError;

pub(crate) fn current_locale() -> Result<String, LocaleError> {
    let window = web_sys::window().ok_or(LocaleError {
        kind: crate::ErrorKind::LookupFailed,
        description: Some("Window property not available".to_owned()),
    })?;
    let navigator = web_sys::Window::navigator(&window);
    web_sys::Navigator::language(&navigator).ok_or(LocaleError {
        kind: crate::ErrorKind::LookupFailed,
        description: Some("Language not available".to_owned()),
    })
}
