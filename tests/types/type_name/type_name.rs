//! Integration tests for TypeName wrappers

use crate::common::*;
use display_as_debug::types::{Full, Short, TypeName};

mod full {
    use super::*;

    const EXPECTED: &str = "alloc::vec::Vec<i32>";

    test_fmt!(consts, TypeName::<Vec<i32>>::FULL, "{:?}", EXPECTED);
    test_fmt!(empty, TypeName::empty::<Vec<i32>, Full>(), "{:?}", EXPECTED);
    test_fmt!(wrap, TypeName::wrap::<Full>(vec![1]), "{:?}", EXPECTED);
    test_fmt!(from, TypeName::<i32, i32, Full>::from(42), "{:?}", "i32");
    test_get!(into_inner, TypeName::wrap::<Full>(42i32), move TypeName::into_inner, 42);
}

mod short {
    use super::*;

    const EXPECTED: &str = "Vec<i32>";

    test_fmt!(consts, TypeName::<Vec<i32>>::SHORT, "{:?}", EXPECTED);
    test_fmt!(empty, TypeName::empty::<Vec<i32>, Short>(), "{:?}", EXPECTED);
    test_fmt!(wrap, TypeName::wrap::<Short>(vec![1]), "{:?}", EXPECTED);
    test_fmt!(from, TypeName::<i32, i32, Short>::from(42), "{:?}", "i32");
    test_get!(into_inner, TypeName::wrap::<Short>(42i32), move TypeName::into_inner, 42);
}
