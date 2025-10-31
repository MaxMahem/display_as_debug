# display_as_debug

[![Build](https://github.com/MaxMahem/display_as_debug/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/display_as_debug/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/display_as_debug/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/display_as_debug/display_as_debug/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/display_as_debug/status.svg)](https://deps.rs/repo/github/MaxMahem/display_as_debug)
[![codecov](https://codecov.io/github/MaxMahem/display_as_debug/graph/badge.svg?token=JezdbWA8pp)](https://codecov.io/github/MaxMahem/display_as_debug)
![GitHub License](https://img.shields.io/github/license/MaxMahem/display_as_debug)

A lightweight utility crate that lets a type’s `Display` implementation be used for its `Debug` implementation.

This is useful when a type's `Display` output is already meaningful (e.g., human-readable identifiers, unit types, etc.), and it is used in debugging contexts or logging frameworks that rely on `Debug`. Also included is a reverse implementation, allowing a type's `Debug` implementation to be used for its `Display` implementation, should that be required.

## Features
- Borrowing and owning adaptors for using `Display` implementations as `Debug`.
- Borrowing and owning adaptors for using `Debug` implementations as `Debug`.
- Extension methods on `Display` and `Debug`, respectively, to wrap values in these wrappers.

## Example
```rust
use display_as_debug::DisplayAsDebug;

struct TestType;

impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "display")
    }
}

let test_type = TestType;

// Use Display as Debug
assert_eq!(format!("{:?}", test_type.display_as_debug()), "display");

// Still uses Display when formatting normally
assert_eq!(format!("{}", test_type.display_as_debug()), "display");

// Or consume the owned value
let display_adaptor = test_type.display_to_debug();
assert_eq!(format!("{:?}", display_adaptor), "display");
```

The inverse adaptor, `DebugAsDisplay`, has the same API.

## Use Case
This crate is admittedly mostly a convenience wrapper, but it still may be helpful when:
- Log types using their `Display` format, but are constrained by APIs requiring `Debug`.
- Want to avoid allocating strings (e.g., via `format!` or `to_string()`) just to satisfy `Debug`.

For example, when formatting a struct field within a `Debug` implementation. You can use the adaptor to wrap a value and feed it into `field` or a similar method from a debug builder to avoid needing to call `format!`, `to_string()`, or otherwise contorting the code in order to use the `Display` implementation of a value.
