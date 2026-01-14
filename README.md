# `display_as_debug`

[![Build](https://github.com/MaxMahem/display_as_debug/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/display_as_debug/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/display_as_debug/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/display_as_debug/display_as_debug/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/display_as_debug/status.svg)](https://deps.rs/repo/github/MaxMahem/display_as_debug)
[![Crates.io](https://img.shields.io/crates/v/display_as_debug.svg)](https://crates.io/crates/display_as_debug)
[![codecov](https://codecov.io/github/MaxMahem/display_as_debug/graph/badge.svg?token=JezdbWA8pp)](https://codecov.io/github/MaxMahem/display_as_debug)
![GitHub License](https://img.shields.io/github/license/MaxMahem/display_as_debug)

A lightweight utility crate with wrappers that let you swap `Display` and `Debug` implementations around.

This crate provides adaptors for using a type's `Display` implementation as its `Debug` implementation (and vice versa). It's useful when you need human-readable output in debug contexts, want to return friendly error messages from `main()`, or need to work with types that only implement one trait but you need the other.

Also included are specialized wrappers for `Option` and `Result` types that provide a implement `Debug` for these types when the underlying type may not, or when you wish to hide the underlying type's debug implementation.

This crate is `no_std` compatible and contains no `unsafe` code.

## Features

- **Borrowing and owning adaptors** for swapping `Display` ↔ `Debug` implementations
- **Extension traits** (`DisplayDebug`, `DebugDisplay`) providing convenient `.as_debug()` and `.as_display()` methods
- **Specialized wrappers** for `Option<T>` and `Result<T, E>` that work without requiring `T: Debug`
- **Type name adaptors** (`Full`, `Short`) for showing full or short type names
- **`DebugTuple` and `DebugStruct` extensions** for conveniently formatting tuple and struct fields

## Installation

It's on [crates.io](https://crates.io/crates/display_as_debug).

## Examples

Basic Usage

```rust
use display_as_debug::as_debug::DisplayDebug;

struct UserId(u32);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "user_{}", self.0)
    }
}

let id = UserId(42);

// Use Display implementation for Debug output
assert_eq!(format!("{:?}", id.display_as_debug()), "user_42");

// Still uses Display when formatting normally
assert_eq!(format!("{}", id.display_as_debug()), "user_42");
```

### Returning Friendly Errors from `main()`

When `main()` returns a `Result<(), E>`, Rust prints errors using their `Debug` implementation, which can be verbose and technical. Use `DisplayDebugged` to show user-friendly error messages instead:

```rust,no_run
use display_as_debug::as_debug::{DisplayDebugged, DisplayDebug};
use std::fmt;

#[derive(Debug)]
struct AppError {
    message: String,
    code: i32,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

fn risky_operation() -> Result<(), AppError> {
    Err(AppError {
        message: "database connection failed".to_string(),
        code: 500,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Wrap errors to use Display instead of Debug
    risky_operation().map_err(|e| e.wrap_display_as_debug())?;
    Ok(())
}
```

Instead of seeing `AppError { message: "database connection failed", code: 500 }`, users see the clean message: `Error 500: database connection failed`.

See [examples/error_from_main.rs](examples/error_from_main.rs) for a complete working example.

## Structure and Tuple Extensions

The `DebugStructExt` and `DebugTupleExt` extension traits provide a cleaner API for implementing `Debug` manually:

```rust
use display_as_debug::{DebugStructExt, DebugTupleExt};
use display_as_debug::type_name::{Short, Full};

struct Secret {
    id: u32,
    key: String,
    payload: Vec<u8>,
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secret")
            .field_display("id", &self.id)        // Use Display impl
            .field_opaque("key")                  // Hide value: "key: .."
            .field_type::<Vec<u8>, Short>("payload") // Show type: "payload: Vec<u8>"
            .finish()
    }
}

struct Wrapper<T>(T);

impl<T> std::fmt::Debug for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Wrapper")
            .field_type::<T, Full>()  // Show full type path
            .field_opaque()           // Tuple opaque field
            .finish()
    }
}
```

### Using Debug as Display

The inverse wrappers (`DebugDisplay`, `AsDebugDisplayed`, `DebugDisplayed`) let you use `Debug` implementations where `Display` is needed:

```rust
use display_as_debug::as_display::DebugDisplay;

let numbers = vec![1, 2, 3];
let formatted = format!("Numbers: {}", numbers.debug_as_display());
assert_eq!(formatted, "Numbers: [1, 2, 3]");
```

## Option and Result Wrappers

For situations where you need to debug `Option` or `Result` types but don't want to expose sensitive data or the inner type doesn't implement `Debug`:

### Type Name Wrappers

Show the type name instead of the actual value:

```rust
use display_as_debug::option::OptionDebugExt;
use display_as_debug::result::ResultDebugExt;
use display_as_debug::type_name::{Full, Short};

let opt = Some(42);
assert_eq!(format!("{:?}", opt.debug_type::<Full>()), "Some(i32)");

let res: Result<String, i32> = Ok("secret".to_string());
assert_eq!(format!("{:?}", res.debug_type::<Full>()), "Ok(alloc::string::String)");
assert_eq!(format!("{:?}", res.debug_type::<Short>()), "Ok(String)");
```

### Opaque Wrappers

Hide the value completely while preserving the variant information:

```rust
use display_as_debug::option::OptionDebugExt;
use display_as_debug::result::ResultDebugExt;

let opt = Some("sensitive data");
assert_eq!(format!("{:?}", opt.debug_opaque()), "Some(..)");

let res: Result<&str, &str> = Ok("secret");
assert_eq!(format!("{:?}", res.debug_opaque()), "Ok(..)");

// Errors are still shown for debugging
let err: Result<&str, &str> = Err("connection failed");
assert_eq!(format!("{:?}", err.debug_opaque()), "Err(\"connection failed\")");
```

These are especially useful for logging and debugging where you want to know the state of an `Option` or `Result` without exposing potentially sensitive data.
