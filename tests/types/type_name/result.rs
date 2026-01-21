use crate::common::*;

use display_as_debug::types::{Full, Short, TypeNameResult};

const EXPECTED_OK_FULL: &str = "Ok(alloc::vec::Vec<i32>)";
const EXPECTED_OK_SHORT: &str = "Ok(Vec<i32>)";

mod empty {
    use super::*;

    test_fmt!(ok_full, TypeNameResult::ok_empty::<Vec<i32>, Full>(), "{:?}", EXPECTED_OK_FULL);
    test_fmt!(ok_short, TypeNameResult::ok_empty::<Vec<i32>, Short>(), "{:?}", EXPECTED_OK_SHORT);
}

mod consts {
    use super::*;

    test_fmt!(ok_full, TypeNameResult::<Vec<i32>>::OK_FULL, "{:?}", EXPECTED_OK_FULL);
    test_fmt!(ok_short, TypeNameResult::<Vec<i32>>::OK_SHORT, "{:?}", EXPECTED_OK_SHORT);
}
