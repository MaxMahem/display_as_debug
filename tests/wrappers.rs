//! Integration tests for wrapper types using std::io::Error

mod common;

use std::io::Error;

use display_as_debug::types::{Full, Short, TestValue};

use crate::common::*;

/// The test error message used across all tests
const ERROR_MSG: &str = "test error";

macro_rules! test_source {
    ($ctor:expr) => {
        #[test]
        fn source() {
            use std::error::Error as StdError;
            let error = Error::other(ERROR_MSG);
            let wrapped = $ctor(error);
            assert!(wrapped.source().is_none());
        }
    };
}

mod debug_as_display_test {
    use super::*;
    use display_as_debug::display::DebugAsDisplay;

    const EXPECTED_DEBUG: &str = r#"Debug("test")"#;

    test_fmt!(display, DebugAsDisplay(TestValue::TEST), "{}", EXPECTED_DEBUG);
    test_fmt!(debug, DebugAsDisplay(TestValue::TEST), "{:?}", EXPECTED_DEBUG);
    test_source!(DebugAsDisplay);
}

mod display_as_debug_test {
    use super::*;
    use display_as_debug::debug::DisplayAsDebug;

    const EXPECTED_DISPLAY: &str = r#"Display("test")"#;

    test_fmt!(display, DisplayAsDebug(TestValue::TEST), "{}", EXPECTED_DISPLAY);
    test_fmt!(debug, DisplayAsDebug(TestValue::TEST), "{:?}", EXPECTED_DISPLAY);
    test_source!(DisplayAsDebug);
}

mod opaque_result {
    use super::*;
    use display_as_debug::result::OpaqueResult;

    const EXPECTED_OK: &str = "Ok(..)";
    const EXPECTED_ERR: &str = r#"Err("error message")"#;

    test_fmt!(ok, OpaqueResult(Ok::<i32, String>(42)), "{:?}", EXPECTED_OK);
    test_fmt!(err, OpaqueResult(Err::<i32, &str>("error message")), "{:?}", EXPECTED_ERR);
}

mod type_result {
    use super::*;
    use display_as_debug::result::TypeNameResult;

    const EXPECTED_OK_FULL: &str = "Ok(alloc::vec::Vec<i32>)";
    const EXPECTED_OK_SHORT: &str = "Ok(Vec<i32>)";
    const EXPECTED_ERR: &str = "Err(42)";

    test_fmt!(ok_full, TypeNameResult::new::<Full>(Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_FULL);
    test_fmt!(ok_short, TypeNameResult::new::<Short>(Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_SHORT);
    test_fmt!(err_full, TypeNameResult::new::<Full>(Err::<Vec<i32>, i32>(42)), "{:?}", EXPECTED_ERR);
    test_fmt!(err_short, TypeNameResult::new::<Short>(Err::<Vec<i32>, i32>(42)), "{:?}", EXPECTED_ERR);
    test_fmt!(from, TypeNameResult::<Vec<i32>, i32, Full>::from(Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_FULL);
}

mod opaque_option {
    use super::*;
    use display_as_debug::option::OpaqueOption;

    const EXPECTED_SOME: &str = "Some(..)";
    const EXPECTED_NONE: &str = "None";

    test_fmt!(some, OpaqueOption(Some(42)), "{:?}", EXPECTED_SOME);
    test_fmt!(none, OpaqueOption(None::<i32>), "{:?}", EXPECTED_NONE);
}

mod type_option {
    use super::*;
    use display_as_debug::option::TypeNameOption;

    const EXPECTED_SOME_FULL: &str = "Some(alloc::vec::Vec<i32>)";
    const EXPECTED_SOME_SHORT: &str = "Some(Vec<i32>)";
    const EXPECTED_NONE: &str = "None";

    test_fmt!(none_full, TypeNameOption::new::<Full>(None::<i32>), "{:?}", EXPECTED_NONE);
    test_fmt!(none_short, TypeNameOption::new::<Short>(None::<i32>), "{:?}", EXPECTED_NONE);
    test_fmt!(some_full, TypeNameOption::new::<Full>(Some(vec![1])), "{:?}", EXPECTED_SOME_FULL);
    test_fmt!(some_short, TypeNameOption::new::<Short>(Some(vec![1])), "{:?}", EXPECTED_SOME_SHORT);
    test_fmt!(from, TypeNameOption::<Vec<i32>, Full>::from(Some(vec![1])), "{:?}", EXPECTED_SOME_FULL);
}
