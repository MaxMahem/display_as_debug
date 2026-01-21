use crate::common::*;

use display_as_debug::wrap::{Full, Short, TypeNameOption};

const EXPECTED_NONE: &str = "None";
const EXPECTED_SOME_FULL: &str = "Some(alloc::vec::Vec<i32>)";
const EXPECTED_SOME_SHORT: &str = "Some(Vec<i32>)";

mod empty {
    use super::*;

    test_fmt!(none_full, TypeNameOption::empty::<Vec<i32>, Full>(&None), "{:?}", EXPECTED_NONE);
    test_fmt!(none_short, TypeNameOption::empty::<Vec<i32>, Short>(&None), "{:?}", EXPECTED_NONE);
    test_fmt!(some_full, TypeNameOption::empty::<Vec<i32>, Full>(&Some(vec![])), "{:?}", EXPECTED_SOME_FULL);
    test_fmt!(some_short, TypeNameOption::empty::<Vec<i32>, Short>(&Some(vec![])), "{:?}", EXPECTED_SOME_SHORT);
}

mod consts {
    use super::*;

    test_fmt!(none_full, TypeNameOption::<Vec<i32>>::NONE, "{:?}", EXPECTED_NONE);
    test_fmt!(some_full, TypeNameOption::<Vec<i32>>::SOME_FULL, "{:?}", EXPECTED_SOME_FULL);
    test_fmt!(some_short, TypeNameOption::<Vec<i32>>::SOME_SHORT, "{:?}", EXPECTED_SOME_SHORT);
}
