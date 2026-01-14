<!-- Markdownlint-disable no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 01/14/2026

### Added

- **`Opaque` type**: Introduced `Opaque` type for obscuring values, showing `".."` for privacy.
- **`TypeName` type**: Introduced `TypeName` type for showing type names using the specified `DisplayMode`.
  - **`DisplayMode` trait**: Introduced `DisplayMode` trait with `Full` and `Short` implementations to control type name formatting.
- **`DebugTupleExt` trait**: Extension trait for `core::fmt::DebugTuple` providing convenient field formatting methods:
  - `field_display()` - Uses the `Display` implementation for a field value
  - `field_type()` - Shows the value's type name using the specified `DisplayMode` (e.g., `field_type::<T, Full>("name")`)
  - `field_opaque()` - Obscures the value, showing `".."` for privacy
- **`DebugStructExt` trait**: Extension trait for `core::fmt::DebugStruct` providing convenient field formatting methods:
  - `field_display()` - Uses the `Display` implementation for a field value
  - `field_type()` - Shows the value's type name using the specified `DisplayMode` (e.g., `field_type::<T, Full>("name")`)
  - `field_opaque()` - Obscures the value, showing `".."` for privacy

### Changed

- **BREAKING**: `OptionDebugExt::debug_type` and `ResultDebugExt::debug_type` are now generic over `DisplayMode`. You must specify the mode (e.g., `.debug_type::<Full>()`).
- **BREAKING**: Renamed methods for clarity and consistency:
  - `as_debug()` → `display_as_debug()`
  - `as_display()` → `debug_as_display()`
  - `to_debug()` → `wrap_display_as_debug()`
  - `to_display()` → `wrap_debug_as_display()`
  - `AsDebugDisplay` → `DebugAsDisplay`
  - `AsDisplayDebug` → `DisplayAsDebug`

## [0.3.0] - 01/07/2026

### Added

- **`no_std` support**: The crate now works in `no_std` environments without any feature flags required
- **Extension traits**: Added `OptionDebugExt` and `ResultDebugExt` traits providing ergonomic `.debug_opaque()` and `.debug_type()` methods for `Option` and `Result` types

### Changed

- **BREAKING**: Renamed types to use fully spelled-out names for consistency:
  - `OpaqueResultDbg` → `OpaqueResultDebug`
  - `OpaqueOptionDbg` → `OpaqueOptionDebug`
- **MSRV**: Specified as Rust 1.85.1

## [0.2.0] - 12/12/2025

### Changed

- **BREAKING**: Reorganized module structure. Types are now exported through their respective submodules:
  - `as_debug` module - for wrapping types implementing `Display` to use their `Display` implementation for `Debug`
    - Contains: `DisplayAsDebug` trait, `AsDisplayWrapper`, `DisplayWrapper`
  - `as_display` module - for wrapping types implementing `Debug` to use their `Debug` implementation for `Display`
    - Contains: `DebugAsDisplay` trait, `AsDebugWrapper`, `DebugWrapper`
  - **Migration**: Update imports from `use display_as_debug::DisplayAsDebug` to `use display_as_debug::as_debug::DisplayAsDebug` (and similarly for other types)

- **BREAKING**: Renamed traits for brevity and clarity:
  - `DisplayAsDebug` → `DisplayDebug`
  - `DebugAsDisplay` → `DebugDisplay`
- **BREAKING**: Renamed methods for conciseness:
  - `display_as_debug()` → `as_debug()`
  - `debug_as_display()` → `as_display()`
  - `display_to_debug()` → `to_debug()`
  - `debug_to_display()` → `to_display()`
- **BREAKING**: Renamed wrapper types for consistency:
  - `DisplayWrapper` → `DisplayDebugged` (owned Display-as-Debug)
  - `AsDisplayWrapper` → `AsDisplayDebug` (borrowed Display-as-Debug)
  - `DebugWrapper` → `DebugDisplayed` (owned Debug-as-Display)
  - `AsDebugWrapper` → `AsDebugDisplay` (borrowed Debug-as-Display)
- **Migration**: Update your code:

  ```rust
  // Before:
  use display_as_debug::as_debug::{DisplayAsDebug, DisplayWrapper};
  error.display_to_debug()
  
  // After:
  use display_as_debug::as_debug::{DisplayDebug, DisplayDebugged};
  error.to_debug()
  ```

### Added

- Implemented `std::error::Error` for `DisplayWrapper` and `AsDisplayWrapper` when the wrapped type implements `Error`. This allows the wrappers to be used seamlessly with `Result<T, Box<dyn Error>>` and other error handling patterns.
- Made all wrapper struct fields public for direct access to wrapped values
- Implemented `OpaqueOptionDbg` for opaque debugging of `Option<T>` values.
- Implemented `OpaqueResultDbg` for opaque debugging of `Result<T, E>` values.
- Implemented `OptionTypeDebug` for type debugging of `Option<T>` values.
- Implemented `ResultTypeDebug` for type debugging of `Result<T, E>` values.

## [0.1.1] - 11/12/2025

### Added

- Exposed the struct wrappers for `DisplayAsDebug` and `DebugAsDisplay` traits.

## [0.1.0] - 10/31/2025

Initial release
