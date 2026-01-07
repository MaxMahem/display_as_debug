mod common;

use crate::common::test_fmt;

mod opaque_result {
    use super::*;
    use display_as_debug::result::OpaqueResultDebug;

    test_fmt!(ok, OpaqueResultDebug(&Ok::<i32, String>(42)), "{:?}", "Ok(...)");
    test_fmt!(err, OpaqueResultDebug(&Err::<i32, &str>("error message")), "{:?}", "Err(\"error message\")");
}

mod type_result {
    use super::*;
    use display_as_debug::result::ResultTypeDebug;

    test_fmt!(ok, ResultTypeDebug(&Ok::<i32, i32>(42)), "{:?}", "Ok(i32)");
    test_fmt!(err, ResultTypeDebug(&Err::<i32, i32>(42)), "{:?}", "Err(42)");
}

mod opaque_option {
    use super::*;
    use display_as_debug::option::OpaqueOptionDebug;

    test_fmt!(some, OpaqueOptionDebug(&Some(42)), "{:?}", "Some(..)");
    test_fmt!(none, OpaqueOptionDebug(&None::<i32>), "{:?}", "None");
}

mod type_option {
    use super::*;
    use display_as_debug::option::OptionTypeDebug;

    test_fmt!(some, OptionTypeDebug(&Some(42)), "{:?}", "Some(i32)");
    test_fmt!(none, OptionTypeDebug(&None::<i32>), "{:?}", "None");
}

mod option_extension {
    use super::*;
    use display_as_debug::option::OptionDebugExt;

    test_fmt!(opaque_debug_some, Some(42).debug_opaque(), "{:?}", "Some(..)");
    test_fmt!(opaque_debug_none, None::<i32>.debug_opaque(), "{:?}", "None");
    test_fmt!(type_debug_some, Some(42).debug_type(), "{:?}", "Some(i32)");
    test_fmt!(type_debug_none, None::<i32>.debug_type(), "{:?}", "None");
}

mod result_extension {
    use super::*;
    use display_as_debug::result::ResultDebugExt;

    test_fmt!(opaque_debug_ok, Ok::<i32, &str>(42).debug_opaque(), "{:?}", "Ok(...)");
    test_fmt!(opaque_debug_err, Err::<i32, &str>("error message").debug_opaque(), "{:?}", "Err(\"error message\")");
    test_fmt!(type_debug_ok, Ok::<i32, &str>(42).debug_type(), "{:?}", "Ok(i32)");
    test_fmt!(type_debug_err, Err::<i32, i32>(404).debug_type(), "{:?}", "Err(404)");
}
