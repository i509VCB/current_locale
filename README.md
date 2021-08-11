# os-locale

A rust library for getting the user's locale as a language code from the OS.
The language code returned is in a BCP47 (IETF) format.

os-locale uses semantic versioning. More information about that may be found here: https://semver.org/

## Platform support

| Platform 	| Implemented		|
| -------	| ---			|
| Windows	| Yes			|
| Unix		| Yes			|
| Mac OS	| Not fully tested	|
| Android	| No			|
| iOS		| Not fully tested	|

The library exposes a single function to get the user's locale from the OS

```rust
pub fn current_locale() -> Result<String, OsLocaleError> {
	// Method Implementation...
}
```

The method either returns a string containing the user's locale as a language code or an error when retrieving the 
locale from the OS.

## Dependencies

os-locale tries to use a few dependencies as possible. However we do use some platform dependencies:

| Platform  	| Dependencies		|
| ----------	| ------------		|
| Windows   	| winapi, libc		|
| Unix	     	| None				|
| Mac OS/iOS	| objc, objc-foundation	|

## License

os-locale is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See the LICENSE-APACHE and LICENSE-MIT files in this repository for more information.
