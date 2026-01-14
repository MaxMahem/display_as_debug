mod common;

use crate::common::test_fmt;

use display_as_debug::type_name::{Full, Short};

mod opaque_result {
    use super::*;
    use display_as_debug::result::OpaqueResultDebug;

    test_fmt!(ok, OpaqueResultDebug(&Ok::<i32, String>(42)), "{:?}", "Ok(..)");
    test_fmt!(err, OpaqueResultDebug(&Err::<i32, &str>("error message")), "{:?}", "Err(\"error message\")");
}

mod type_result {
    use super::*;
    use display_as_debug::result::ResultTypeDebug;

    test_fmt!(ok_full, ResultTypeDebug::new::<Full>(&Ok::<Vec<i32>, i32>(vec![])), "{:?}", "Ok(alloc::vec::Vec<i32>)");
    test_fmt!(ok_short, ResultTypeDebug::new::<Short>(&Ok::<Vec<i32>, i32>(vec![])), "{:?}", "Ok(Vec<i32>)");
    test_fmt!(err_full, ResultTypeDebug::new::<Full>(&Err::<Vec<i32>, i32>(42)), "{:?}", "Err(42)");
    test_fmt!(err_short, ResultTypeDebug::new::<Short>(&Err::<Vec<i32>, i32>(42)), "{:?}", "Err(42)");
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

    test_fmt!(some_full, OptionTypeDebug::new::<Full>(&Some(vec![1])), "{:?}", "Some(alloc::vec::Vec<i32>)");
    test_fmt!(some_short, OptionTypeDebug::new::<Short>(&Some(vec![1])), "{:?}", "Some(Vec<i32>)");
    test_fmt!(none_full, OptionTypeDebug::new::<Full>(&None::<i32>), "{:?}", "None");
    test_fmt!(none_short, OptionTypeDebug::new::<Short>(&None::<i32>), "{:?}", "None");
}

mod option_extension {
    use super::*;
    use display_as_debug::option::OptionDebugExt;

    test_fmt!(opaque_debug_some, Some(42).debug_opaque(), "{:?}", "Some(..)");
    test_fmt!(opaque_debug_none, None::<i32>.debug_opaque(), "{:?}", "None");
    test_fmt!(type_debug_some_full, Some(vec![1]).debug_type::<Full>(), "{:?}", "Some(alloc::vec::Vec<i32>)");
    test_fmt!(type_debug_some_short, Some(vec![1]).debug_type::<Short>(), "{:?}", "Some(Vec<i32>)");
    test_fmt!(type_debug_none_full, None::<i32>.debug_type::<Full>(), "{:?}", "None");
    test_fmt!(type_debug_none_short, None::<i32>.debug_type::<Short>(), "{:?}", "None");
}

mod result_extension {
    use super::*;
    use display_as_debug::result::ResultDebugExt;

    test_fmt!(opaque_debug_ok, Ok::<i32, &str>(42).debug_opaque(), "{:?}", "Ok(..)");
    test_fmt!(opaque_debug_err, Err::<i32, &str>("error message").debug_opaque(), "{:?}", "Err(\"error message\")");
    test_fmt!(type_debug_ok, Ok::<i32, &str>(42).debug_type::<Full>(), "{:?}", "Ok(i32)");
    test_fmt!(type_debug_err, Err::<i32, i32>(404).debug_type::<Full>(), "{:?}", "Err(404)");
}
