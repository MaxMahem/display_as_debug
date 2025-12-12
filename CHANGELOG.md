# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Implemented `std::error::Error` for `DisplayWrapper` and `AsDisplayWrapper` when the wrapped type implements `Error`. This allows the wrappers to be used seamlessly with `Result<T, Box<dyn Error>>` and other error handling patterns.
- Made all wrapper struct fields public for direct access to wrapped values
- Implemented `OpaqueOptionDbg` for opaque debugging of `Option<T>` values.
- Implemented `OpaqueResultDbg` for opaque debugging of `Result<T, E>` values.
- Implemented `OptionTypeDebug` for type debugging of `Option<T>` values.
- Implemented `ResultTypeDebug` for type debugging of `Result<T, E>` values.

## [1.0.1] - 11/12/2025

### Added

- Exposed the struct wrappers for `DisplayAsDebug` and `DebugAsDisplay` traits.

## [1.0.0] - 10/31/2025

Initial release
