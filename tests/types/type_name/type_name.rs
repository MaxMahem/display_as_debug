//! Integration tests for TypeName wrappers

use crate::common::*;
use display_as_debug::types::{Full, Short, TypeName};

mod full {
    use super::*;

    const EXPECTED: &str = "alloc::vec::Vec<i32>";

    test_fmt!(consts, TypeName::<Vec<i32>>::FULL, "{:?}", EXPECTED);
    test_fmt!(empty, TypeName::empty::<Vec<i32>, Full>(), "{:?}", EXPECTED);
}

mod short {
    use super::*;

    const EXPECTED: &str = "Vec<i32>";

    test_fmt!(consts, TypeName::<Vec<i32>>::SHORT, "{:?}", EXPECTED);
    test_fmt!(empty, TypeName::empty::<Vec<i32>, Short>(), "{:?}", EXPECTED);
}
