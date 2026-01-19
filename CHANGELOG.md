<!-- Markdownlint-disable no-duplicate-heading -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 01/16/2026

### Added

- **`OpaqueSet` struct**: Shows obscured set/map length as `{..: N}`. Also available as `OpaqueMap` alias.
- **`TypeNameSet` struct**: Shows set/map type and length as `{<Type>: N}`. Also available as `TypeNameMap` alias.
- **`types` module**: Reorganized into a dedicated `types` module for specialized formatting types:
  - **`Opaque` struct**: Now wraps a value (defaulting to `()`) that formats as `..` in `Debug` contexts. The unit type version can be used as a ZST marker.
  - **`TestValue` struct**: Added for demonstrating display modes in tests and examples.
  - **`OpaqueList` struct**: Shows obscured list length as `[..: N]`.
  - **`TypeNameList` struct**: Shows list type and length as `[<Type>: N]`.
- **`fmt` module**: Reorganized formatting extension traits into a dedicated `fmt` module:
  - **`DebugListExt` trait**: Extension trait for `std::fmt::DebugList`:
    - `entry_display()` - Uses the `Display` implementation for an entry
    - `entries_display()` - Uses the `Display` implementation for an iterator of entries
  - **`DebugMapExt` trait**: Extension trait for `std::fmt::DebugMap`:
    - `entry_display()` - Uses the `Display` implementation for a value
    - `entries_display()` - Uses the `Display` implementation for iterator values
    - `entry_opaque()` - Uses an `Opaque` wrapper for the value
    - `entries_opaque()` - Uses `Opaque` wrappers for iterator values
  - **`DebugSetExt` trait**: Extension trait for `std::fmt::DebugSet`:
    - `entry_display()` - Uses the `Display` implementation for an entry
    - `entries_display()` - Uses the `Display` implementation for iterator entries
- **`From<T>` implementations**: Added `From<T>` for all wrapper types enabling `.into()` conversions.
- **`Deref`/`AsRef`/`AsMut` implementations**: Added for all wrapper types for transparent access to wrapped values.
- **Standard trait implementations**: All wrapper types now implement `Copy`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, and `Hash` where applicable.

### Removed

- **BREAKING**: Removed all wrapper extension traits
  - **Migration**: Use `From`/`Into` instead.

### Changed

- **BREAKING**: Reorganized module structure:
  - `wrap` module - Contains all wrapper types:
    - `DisplayAsDebug`, `DebugAsDisplay` - Debug/Display conversion wrappers
    - `OpaqueOption`, `TypeNameOption` - Option formatting wrappers (in `wrap::option` submodule)
    - `OpaqueResult`, `TypeNameResult` - Result formatting wrappers (in `wrap::result` submodule)
  - `types` module - Contains `Opaque`, `TypeName`, `OpaqueList`, `TypeNameList`, `TestValue`
  - `fmt` module - Contains all `DebugXxxExt` extension traits
  - **Migration**: Update imports to use new module paths:
  
    ```rust
    // Before (0.4.x):
    use display_as_debug::debug::DisplayAsDebug;
    use display_as_debug::option::OpaqueOption;
    
    // After (0.5.0):
    use display_as_debug::wrap::DisplayAsDebug;
    use display_as_debug::wrap::OpaqueOption;
    ```

- **BREAKING**: Renamed types for clarity:
  - `debug_type` → `debug_type_name`
  - `OpaqueResultDbg` → `OpaqueResult`
  - `OpaqueOptionDbg` → `OpaqueOption`
  - `OptionTypeDebug` → `TypeNameOption`
  - `ResultTypeDebug` → `TypeNameResult`
  - `OptionDebugExt` → `DebugOption`
  - `ResultDebugExt` → `DebugResult`
- **BREAKING**: Option/Result wrappers now own their values directly instead of borrowing:
  - `OpaqueOption<T>` replaces `OpaqueOption<'a, T>` — now owns `Option<T>`
  - `OpaqueResult<T, E>` replaces `OpaqueResult<'a, T, E>` — now owns `Result<T, E>`
  - `TypeNameOption<T, M>` replaces `TypeNameOption<'a, T, M>` — now owns `Option<T>`
  - `TypeNameResult<T, E, M>` replaces `TypeNameResult<'a, T, E, M>` — now owns `Result<T, E>`
  - **Migration**: Use `opt.as_ref()` or `res.as_ref()` to convert borrowed values:

    ```rust
    // Before:
    OpaqueOption(&some_option)
    TypeNameResult::new::<Full>(&some_result)
    
    // After:
    OpaqueOption(some_option.as_ref())  // for borrowed
    OpaqueOption(some_option)           // for owned
    TypeNameResult::<_, _, Full>::new(some_result)
    ```

- **BREAKING**: `TypeName`, `TypeNameList`, and `TypeNameSet` no longer implement `Display`. Use `{:?}` (Debug format) instead of `{}` (Display format).

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
