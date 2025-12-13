//! Integration tests for wrapper types using std::io::Error
//!
//! This module provides shared test macros for all wrapper types and contains
//! separate test modules for each wrapper implementation.

use std::io::Error;

/// The test error message used across all tests
const TEST_ERROR_MSG: &str = "test error";

/// The expected Debug representation of io::Error::other("test error")
/// Used by DebugDisplayed and AsDebugDisplay wrappers
const EXPECTED_DEBUG_STR: &str = r#"Custom { kind: Other, error: "test error" }"#;

/// The expected Display representation of io::Error::other("test error")
/// Used by DisplayDebugged and AsDisplayDebug wrappers
const EXPECTED_DISPLAY_STR: &str = "test error";

macro_rules! test_fmt {
    ($id:ident, borrow $ctor:expr, $fmt:expr, $expected:expr) => {
        #[test]
        fn $id() {
            let error = Error::other(TEST_ERROR_MSG);
            let wrapped = $ctor(&error);
            assert_eq!(format!($fmt, wrapped), $expected);
        }
    };
    ($id:ident, owned $ctor:expr, $fmt:expr, $expected:expr) => {
        #[test]
        fn $id() {
            let error = Error::other(TEST_ERROR_MSG);
            let wrapped = $ctor(error);
            assert_eq!(format!($fmt, wrapped), $expected);
        }
    };
}

macro_rules! test_source {
    (owned $ctor:expr) => {
        #[test]
        fn source() {
            use std::error::Error as StdError;
            let error = Error::other(TEST_ERROR_MSG);
            let wrapped = $ctor(error);
            assert!(wrapped.source().is_none());
        }
    };
    (borrow $ctor:expr) => {
        #[test]
        fn source() {
            use std::error::Error as StdError;
            let error = Error::other(TEST_ERROR_MSG);
            let wrapped = $ctor(&error);
            assert!(wrapped.source().is_none());
        }
    };
}

mod debug_displayed {
    use super::*;
    use display_as_debug::as_display::DebugDisplayed;

    test_fmt!(display, owned DebugDisplayed, "{}", EXPECTED_DEBUG_STR);
    test_fmt!(debug, owned DebugDisplayed, "{:?}", EXPECTED_DEBUG_STR);
    test_source!(owned DebugDisplayed);
}

mod debug_display {
    use super::*;
    use display_as_debug::as_display::DebugDisplay;

    test_fmt!(borrowed, borrow DebugDisplay::as_display, "{}", EXPECTED_DEBUG_STR);
    test_fmt!(owned, owned DebugDisplay::to_display, "{}", EXPECTED_DEBUG_STR);
}

mod as_debug_display {
    use super::*;
    use display_as_debug::as_display::AsDebugDisplay;

    test_fmt!(display, borrow AsDebugDisplay, "{}", EXPECTED_DEBUG_STR);
    test_fmt!(debug, borrow AsDebugDisplay, "{:?}", EXPECTED_DEBUG_STR);
    test_source!(borrow AsDebugDisplay);
}

mod display_debugged {
    use super::*;
    use display_as_debug::as_debug::DisplayDebugged;

    test_fmt!(display, owned DisplayDebugged, "{}", EXPECTED_DISPLAY_STR);
    test_fmt!(debug, owned DisplayDebugged, "{:?}", EXPECTED_DISPLAY_STR);
    test_source!(owned DisplayDebugged);
}

mod display_debug {
    use super::*;
    use display_as_debug::as_debug::DisplayDebug;

    test_fmt!(borrowed, borrow DisplayDebug::as_debug, "{}", EXPECTED_DISPLAY_STR);
    test_fmt!(owned, owned DisplayDebug::to_debug, "{}", EXPECTED_DISPLAY_STR);
}

mod as_display_debug {
    use super::*;
    use display_as_debug::as_debug::AsDisplayDebug;

    test_fmt!(display, borrow AsDisplayDebug, "{}", EXPECTED_DISPLAY_STR);
    test_fmt!(debug, borrow AsDisplayDebug, "{:?}", EXPECTED_DISPLAY_STR);
    test_source!(borrow AsDisplayDebug);
}
