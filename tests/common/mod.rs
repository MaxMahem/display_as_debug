//! Common test utilities and macros shared across integration tests

/// Macro for testing format output
///
/// (test_name, ctor, fmt, expected)
macro_rules! test_fmt {
    ($test_name:ident, $ctor:expr, $fmt:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            assert_eq!(format!($fmt, $ctor), $expected, "format should match expected");
        }
    };
}

/// Macro for testing [`std::error::Error::source`].
///
/// (wrapped, expected) - `expected` is [`Option<String>`] using the error's [`Display`]
/// implementation.
#[allow(unused_macros, reason = "Used only in some integration tests")]
macro_rules! test_source {
    ($wrapped:expr, $expected:expr) => {
        #[test]
        fn source() {
            use std::error::Error as StdError;

            let wrapped = $wrapped;
            assert_eq!(wrapped.source().map(|e| e.to_string()), $expected, "source should match expected");
        }
    };
}

/// Macro for testing getter/accessor methods.
///
/// Two forms:
/// - `test_get!(name, ctor, Type::method, expected)` - calls `method(&value)`
/// - `test_get!(name, ctor, move |v| v.into_inner(), expected)` - calls the closure with `value`
#[allow(unused_macros, reason = "Used only in some integration tests")]
macro_rules! test_get {
    // Form for closures (consuming methods) - use `move` keyword to distinguish
    ($test_name:ident, $ctor:expr, move $expr:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let value = $ctor;
            assert_eq!($expr(value), $expected, "result should match expected");
        }
    };
    // Form for method references (takes &self)
    ($test_name:ident, $ctor:expr, $expr:expr, $expected:expr) => {
        #[test]
        fn $test_name() {
            let value = $ctor;
            assert_eq!($expr(&value), $expected, "result should match expected");
        }
    };
}

pub(crate) use test_fmt;
#[allow(unused_imports, reason = "Used only in some integration tests")]
pub(crate) use test_get;
#[allow(unused_imports, reason = "Used only in some integration tests")]
pub(crate) use test_source;
