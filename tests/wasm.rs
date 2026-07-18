use current_locale::ErrorKind;

#[cfg(test)]
pub fn main() {
    // node does not have a `window` property, as such this test has to be run in the browser
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    use current_locale;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn parse_runtime_locale_wasm() {
        // same as the test in lib, but using console_log! instead of println!
        let locale = current_locale::current_locale();

        match locale {
            Ok(locale) => console_log!("Got locale code {}", locale),
            Err(error) => match error.kind() {
                ErrorKind::NotIetfCompliant(s) => {
                    panic!("Got locale code {} but it is not IETF compliant!", s)
                }

                ErrorKind::LookupFailed => panic!("Failed to look up locale code from the OS!"),
            },
        }
    }
}
