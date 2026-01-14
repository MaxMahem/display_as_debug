//! Integration tests for wrapper types using std::io::Error
//!
//! This module provides shared test macros for all wrapper types and contains
//! separate test modules for each wrapper implementation.

mod common;

use std::io::Error;

use crate::common::*;

/// The test error message used across all tests
const ERROR_MSG: &str = "test error";

/// The expected Debug representation of io::Error::other("test error")
/// Used by DebugDisplayed and AsDebugDisplay wrappers
const EXPECTED_DEBUG: &str = r#"Custom { kind: Other, error: "test error" }"#;

/// The expected Display representation of io::Error::other("test error")
/// Used by DisplayDebugged and AsDisplayDebug wrappers
const EXPECTED_DISPLAY: &str = "test error";

macro_rules! test_source {
    (owned $ctor:expr) => {
        #[test]
        fn source() {
            use std::error::Error as StdError;
            let error = Error::other(ERROR_MSG);
            let wrapped = $ctor(error);
            assert!(wrapped.source().is_none());
        }
    };
    (borrow $ctor:expr) => {
        #[test]
        fn source() {
            use std::error::Error as StdError;
            let error = Error::other(ERROR_MSG);
            let wrapped = $ctor(&error);
            assert!(wrapped.source().is_none());
        }
    };
}

mod debug_displayed {
    use super::*;
    use display_as_debug::as_display::DebugDisplayed;

    test_fmt!(display, DebugDisplayed(Error::other(ERROR_MSG)), "{}", EXPECTED_DEBUG);
    test_fmt!(debug, DebugDisplayed(Error::other(ERROR_MSG)), "{:?}", EXPECTED_DEBUG);
    test_source!(owned DebugDisplayed);
}

mod debug_display {
    use super::*;
    use display_as_debug::as_display::DebugDisplay;

    test_fmt!(borrowed, Error::other(ERROR_MSG).debug_as_display(), "{}", EXPECTED_DEBUG);
    test_fmt!(owned, Error::other(ERROR_MSG).wrap_debug_as_display(), "{}", EXPECTED_DEBUG);
}

mod as_debug_display {
    use super::*;
    use display_as_debug::as_display::DebugAsDisplay;

    test_fmt!(display, DebugAsDisplay(&Error::other(ERROR_MSG)), "{}", EXPECTED_DEBUG);
    test_fmt!(debug, DebugAsDisplay(&Error::other(ERROR_MSG)), "{:?}", EXPECTED_DEBUG);
    test_source!(borrow DebugAsDisplay);
}

mod display_debugged {
    use super::*;
    use display_as_debug::as_debug::DisplayDebugged;

    test_fmt!(display, DisplayDebugged(Error::other(ERROR_MSG)), "{}", EXPECTED_DISPLAY);
    test_fmt!(debug, DisplayDebugged(Error::other(ERROR_MSG)), "{:?}", EXPECTED_DISPLAY);
    test_source!(owned DisplayDebugged);
}

mod display_debug {
    use super::*;
    use display_as_debug::as_debug::DisplayDebug;

    test_fmt!(borrowed, Error::other(ERROR_MSG).display_as_debug(), "{}", EXPECTED_DISPLAY);
    test_fmt!(owned, Error::other(ERROR_MSG).wrap_display_as_debug(), "{}", EXPECTED_DISPLAY);
}

mod as_display_debug {
    use super::*;
    use display_as_debug::as_debug::DisplayAsDebug;

    test_fmt!(display, DisplayAsDebug(&std::io::Error::other(ERROR_MSG)), "{}", EXPECTED_DISPLAY);
    test_fmt!(debug, DisplayAsDebug(&std::io::Error::other(ERROR_MSG)), "{:?}", EXPECTED_DISPLAY);
    test_source!(borrow DisplayAsDebug);
}
