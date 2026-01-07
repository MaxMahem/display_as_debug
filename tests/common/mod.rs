//! Common test utilities and macros shared across integration tests

/// Macro for testing format output
///
/// Takes a wrapped expression and tests its formatted output.
/// This keeps the macro simple - it just tests formatting, not construction.
macro_rules! test_fmt {
    ($id:ident, $wrapped:expr, $fmt:expr, $expected:expr) => {
        #[test]
        fn $id() {
            assert_eq!(format!($fmt, $wrapped), $expected);
        }
    };
}

pub(crate) use test_fmt;
