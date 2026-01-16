# `display_as_debug`

[![Build](https://github.com/MaxMahem/display_as_debug/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/display_as_debug/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/display_as_debug/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/display_as_debug/display_as_debug/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/display_as_debug/status.svg)](https://deps.rs/repo/github/MaxMahem/display_as_debug)
[![Crates.io](https://img.shields.io/crates/v/display_as_debug.svg)](https://crates.io/crates/display_as_debug)
[![codecov](https://codecov.io/github/MaxMahem/display_as_debug/graph/badge.svg?token=JezdbWA8pp)](https://codecov.io/github/MaxMahem/display_as_debug)
![GitHub License](https://img.shields.io/github/license/MaxMahem/display_as_debug)

A lightweight utility crate with wrappers that let you swap `Display` and `Debug` implementations around.

This crate provides a number of utility types, including adaptors for using a type's `Display` implementation as its `Debug` implementation (and vice versa). It's useful when you need human-readable output in debug contexts, want to return friendly error messages from `main()`, or need to work with types that only implement one trait but you need the other.

This crate is `no_std` compatible and contains no `unsafe` code.

## Features

- **Transparent wrapper types** (`DisplayAsDebug`, `DebugAsDisplay`) for swapping `Display` ↔ `Debug` implementations
- **Specialized wrappers** for `Option<T>` and `Result<T, E>` that work without requiring `T: Debug`
- **`DebugXXX` extensions** for conveniently formatting `std::fmt` `DebugXXX` debug helper types
- **Obscuring `Option`/`Result` wrappers** for obscuring values while preserving variant information
- **Various Format Types** for providing information for `Debug` and `Display`

## Installation

It's on [crates.io](https://crates.io/crates/display_as_debug).

## Examples

### Basic Usage - Display as Debug Wrappers

```rust
use display_as_debug::types::TestValue;
use display_as_debug::debug::DisplayAsDebug;
use display_as_debug::display::DebugAsDisplay;

// `TestValue` implements both `Display` and `Debug`, but with different output.
assert_eq!(format!("{}", TestValue::TEST), r#"Display("test")"#, "Should be Display");
assert_eq!(format!("{:?}", TestValue::TEST), r#"Debug("test")"#, "Should be Debug");

assert_eq!(
    format!("{:?}", DisplayAsDebug(&TestValue::TEST)),
    r#"Display("test")"#,
    "Should be Display in Debug context"
);
assert_eq!(
    format!("{}", DebugAsDisplay(&TestValue::TEST)),
    r#"Debug("test")"#,
    "Should be Debug in Display context"
);

// All wrappers implement From, so you can use .into()
let wrapped: DisplayAsDebug<_> = TestValue::DEFAULT.into();
assert_eq!(format!("{:?}", wrapped), "Display(())");
```

> **Tip**: For ergonomic chaining, the [`tap`](https://crates.io/crates/tap) crate's `.pipe()` and `conv` methods works great:
>
> ```rust,ignore
> use tap::{Pipe, Conv};
>
> let display_as_debug = value.pipe(DisplayAsDebug);
> let display_as_debug = value.conv::<DisplayAsDebug>();
> ```

### Returning Friendly Errors from `main()`

When `main()` returns a `Result<(), E>`, Rust prints errors using their `Debug` implementation, which can be verbose and technical. Use `DisplayAsDebug` to show user-friendly error messages instead:

See [examples/error_from_main.rs](examples/error_from_main.rs) for a complete working example.

## Debug extensions

The `fmt` module contains extension traits for the various `std::fmt` `DebugXXX` helper types to extend their functionality.

### `DebugStructExt`

```rust
use display_as_debug::fmt::DebugStructExt;
use display_as_debug::types::{Short, Full};
use std::fmt::{Formatter, Debug};

struct Secret {
    id: u32,
    key: &'static str,
    payload: Vec<u8>,
}

impl Debug for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secret")
            .field_display("id", &self.id)
            .field_opaque("key")
            .field_type::<Vec<u8>, Short>("payload")
            .finish()
    }
}

let secret = Secret { id: 42, key: "secret", payload: vec![1, 2, 3] };

assert_eq!(format!("{:?}", secret), "Secret { id: 42, key: .., payload: Vec<u8> }");
```

`DebugTupleExt` has a similar API.

## Option and Result Wrappers

The `option` and `result` modules provide wrappers for `Option` and `Result` types that work without requiring `T: Debug`.

### Type Name Wrappers

Show the type name instead of the actual value:

```rust
use display_as_debug::option::TypeNameOption;
use display_as_debug::result::TypeNameResult;
use display_as_debug::types::{Full, Short};

assert_eq!(
    format!("{:?}", TypeNameOption::new::<Full>(Some(42))),
    "Some(i32)"
);

let ok: Result<String, i32> = Ok("secret".to_string());
assert_eq!(
    format!("{:?}", TypeNameResult::new::<Full>(ok.as_ref())),
    "Ok(&alloc::string::String)"
);
assert_eq!(
    format!("{:?}", TypeNameResult::new::<Short>(ok.as_ref())),
    "Ok(String)"
);

let err: Result<String, i32> = Err(42);
assert_eq!(
    format!("{:?}", TypeNameResult::new::<Full>(err.as_ref())),
    "Err(42)"
);
```

### Opaque Wrappers

Hide the value completely while preserving the variant information:

```rust
use display_as_debug::option::OpaqueOption;
use display_as_debug::result::OpaqueResult;

let opt = Some("sensitive data");
assert_eq!(format!("{:?}", OpaqueOption(opt)), "Some(..)");

let res: Result<&str, &str> = Ok("secret");
assert_eq!(format!("{:?}", OpaqueResult(res)), "Ok(..)");

// Errors are still shown for debugging
let err: Result<&str, &str> = Err("connection failed");
assert_eq!(format!("{:?}", OpaqueResult(err)), r#"Err("connection failed")"#);
```

## Debug Formatting Types

The `types` module provides a set of types that can be used to convey various debug formatting information, such as type names, opaque values, and lists.
