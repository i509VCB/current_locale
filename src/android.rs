use crate::ErrorKind;
#[cfg(target_os = "android")]
use crate::LocaleError;

pub(crate) fn current_locale() -> Result<String, LocaleError> {
    retrieve_locale_from_jni().map_err(|err| LocaleError {
        kind: ErrorKind::LookupFailed,
        description: Some(err.to_string()),
    })
}

fn retrieve_locale_from_jni() -> Result<String, jni::errors::Error> {
    // Access the current android context
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let mut env = vm.attach_current_thread().unwrap();

    let locale_class = env.find_class("java/util/Locale")?;

    // Call Locale.getDefault() to get the default locale
    let default_locale =
        env.call_static_method(locale_class, "getDefault", "()Ljava/util/Locale;", &[])?;

    let default_locale_obj = default_locale.l()?;

    // Call toLanguageTag() on the default locale
    // This gets an ietf compliant lanuage string https://developer.android.com/reference/java/util/Locale
    let language = env.call_method(
        default_locale_obj,
        "toLanguageTag",
        "()Ljava/lang/String;",
        &[],
    )?;

    // Convert the result to a Rust String
    let language_obj = language.l()?;
    let language_str = env.get_string((&language_obj).into())?;
    let language_rust = language_str.to_string_lossy().to_string();

    Ok(language_rust)
}
