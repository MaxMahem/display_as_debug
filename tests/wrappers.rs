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
    use display_as_debug::display::{DebugAsDisplay, DebugDisplay};

    const EXPECTED_DEBUG: &str = r#"Debug("test")"#;

    test_fmt!(display, DebugAsDisplay(TestValue::TEST), "{}", EXPECTED_DEBUG);
    test_fmt!(debug, DebugAsDisplay(TestValue::TEST), "{:?}", EXPECTED_DEBUG);
    test_source!(DebugAsDisplay);

    test_fmt!(ext_display, TestValue::TEST.debug_as_display(), "{}", EXPECTED_DEBUG);
    test_fmt!(ext_debug, TestValue::TEST.debug_as_display(), "{:?}", EXPECTED_DEBUG);
}

mod display_as_debug_test {
    use super::*;
    use display_as_debug::debug::{DisplayAsDebug, DisplayDebug};

    const EXPECTED_DISPLAY: &str = r#"Display("test")"#;

    test_fmt!(display, DisplayAsDebug(TestValue::TEST), "{}", EXPECTED_DISPLAY);
    test_fmt!(debug, DisplayAsDebug(TestValue::TEST), "{:?}", EXPECTED_DISPLAY);
    test_source!(DisplayAsDebug);

    test_fmt!(ext_display, TestValue::TEST.display_as_debug(), "{}", EXPECTED_DISPLAY);
    test_fmt!(ext_debug, TestValue::TEST.display_as_debug(), "{:?}", EXPECTED_DISPLAY);
}

mod opaque_result {
    use super::*;
    use display_as_debug::result::OpaqueResult;

    test_fmt!(ok, OpaqueResult(&Ok::<i32, String>(42)), "{:?}", "Ok(..)");
    test_fmt!(err, OpaqueResult(&Err::<i32, &str>("error message")), "{:?}", "Err(\"error message\")");
}

mod type_result {
    use super::*;
    use display_as_debug::result::TypeNameResult;

    const EXPECTED_OK_FULL: &str = "Ok(alloc::vec::Vec<i32>)";
    const EXPECTED_OK_SHORT: &str = "Ok(Vec<i32>)";
    const EXPECTED_ERR: &str = "Err(42)";

    test_fmt!(ok_full, TypeNameResult::new::<Full>(&Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_FULL);
    test_fmt!(ok_short, TypeNameResult::new::<Short>(&Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_SHORT);
    test_fmt!(err_full, TypeNameResult::new::<Full>(&Err::<Vec<i32>, i32>(42)), "{:?}", EXPECTED_ERR);
    test_fmt!(err_short, TypeNameResult::new::<Short>(&Err::<Vec<i32>, i32>(42)), "{:?}", EXPECTED_ERR);
    test_fmt!(from, TypeNameResult::from(&Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_FULL);
}

mod opaque_option {
    use super::*;
    use display_as_debug::option::OpaqueOption;

    const EXPECTED_SOME: &str = "Some(..)";
    const EXPECTED_NONE: &str = "None";

    test_fmt!(some, OpaqueOption(&Some(42)), "{:?}", EXPECTED_SOME);
    test_fmt!(none, OpaqueOption(&None::<i32>), "{:?}", EXPECTED_NONE);
}

mod type_option {
    use super::*;
    use display_as_debug::option::TypeNameOption;

    const EXPECTED_SOME_FULL: &str = "Some(alloc::vec::Vec<i32>)";
    const EXPECTED_SOME_SHORT: &str = "Some(Vec<i32>)";
    const EXPECTED_NONE: &str = "None";

    test_fmt!(some_full, TypeNameOption::new::<Full>(&Some(vec![1])), "{:?}", EXPECTED_SOME_FULL);
    test_fmt!(some_short, TypeNameOption::new::<Short>(&Some(vec![1])), "{:?}", EXPECTED_SOME_SHORT);
    test_fmt!(none, TypeNameOption::new::<Full>(&None::<i32>), "{:?}", EXPECTED_NONE);
    test_fmt!(from, TypeNameOption::<_, Full>::from(&Some(vec![1])), "{:?}", EXPECTED_SOME_FULL);
}

mod option_extension {
    use super::*;
    use display_as_debug::option::DebugOption;

    test_fmt!(opaque_debug_some, Some(42).debug_opaque(), "{:?}", "Some(..)");
    test_fmt!(opaque_debug_none, None::<i32>.debug_opaque(), "{:?}", "None");
    test_fmt!(type_debug_some_full, Some(vec![1]).debug_type_name::<Full>(), "{:?}", "Some(alloc::vec::Vec<i32>)");
    test_fmt!(type_debug_some_short, Some(vec![1]).debug_type_name::<Short>(), "{:?}", "Some(Vec<i32>)");
    test_fmt!(type_debug_none_full, None::<i32>.debug_type_name::<Full>(), "{:?}", "None");
    test_fmt!(type_debug_none_short, None::<i32>.debug_type_name::<Short>(), "{:?}", "None");
}

mod result_extension {
    use super::*;
    use display_as_debug::result::DebugResult;

    test_fmt!(opaque_debug_ok, Ok::<i32, &str>(42).debug_opaque(), "{:?}", "Ok(..)");
    test_fmt!(opaque_debug_err, Err::<i32, &str>("error message").debug_opaque(), "{:?}", "Err(\"error message\")");
    test_fmt!(type_debug_ok, Ok::<i32, &str>(42).debug_type_name::<Full>(), "{:?}", "Ok(i32)");
    test_fmt!(type_debug_err, Err::<i32, i32>(404).debug_type_name::<Full>(), "{:?}", "Err(404)");
}
