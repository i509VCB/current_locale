# current_locale

A rust library for getting the current locale as a language code.
The language code returned is in a BCP47 (IETF) format.

## Platform support

| Platform   | Implemented      |
| ---------- | ---------------- |
| Windows    | Yes              |
| Unix       | Yes              |
| macOS      | Yes              |
| Android    | Yes              |
| iOS        | Not fully tested |
| Web (wasm) | Yes              |

The library exposes a single function to get the user's locale from the OS

```rust
pub fn current_locale() -> Result<String, LocaleError> {
    // Method Implementation...
}
```

The method either returns a string containing the user's locale as a language code or an error when retrieving the
locale from the OS.

## Dependencies

current_locale tries to use a few dependencies as possible. However we do necessarily require dependencies on some platforms:

| Platform    | Dependencies            |
| ----------- | ----------------------- |
| Windows     | winapi, libc            |
| Unix        | None                    |
| macOS & iOS | objc2, objc2-foundation |
| Android     | jni, ndk-context        |
| Web (wasm)  | web-sys                 |

## License

current_locale is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See the LICENSE-APACHE and LICENSE-MIT files in this repository for more information.
