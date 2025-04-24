use crate::{ErrorKind, LocaleError};
use std::ffi::OsString;
use std::os::raw::c_int;
use std::os::windows::prelude::*;
use winapi::um::winnls::GetUserDefaultLocaleName;
use winapi::um::winnt::{LOCALE_NAME_MAX_LENGTH, LPWSTR, WCHAR};

// https://docs.microsoft.com/en-us/windows/win32/intl/locale-names
pub(crate) fn current_locale() -> Result<String, LocaleError> {
    let mut locale_name_buf: Vec<WCHAR> = Vec::with_capacity(LOCALE_NAME_MAX_LENGTH);
    let locale_name: LPWSTR = locale_name_buf.as_mut_ptr();

    /*
     * SAFETY: Input `locale_name` LPWSTR buffer must have same capacity as LOCALE_NAME_MAX_LENGTH
     * the parameter passed into `cchLocaleName`
     */
    // https://docs.microsoft.com/en-us/windows/win32/api/winnls/nf-winnls-getuserdefaultlocalename
    let buf_size: c_int =
        unsafe { GetUserDefaultLocaleName(locale_name, LOCALE_NAME_MAX_LENGTH as c_int) };

    if buf_size == 0 {
        // Buffer was likely too small
        return Err(ErrorKind::LookupFailed.into());
    }

    // Convert WCHAR buffer into OsString
    let os_lang: OsString = OsString::from_wide(unsafe {
        std::slice::from_raw_parts(locale_name, libc::wcslen(locale_name))
    });

    Ok(os_lang.to_str().unwrap().to_string())
}
