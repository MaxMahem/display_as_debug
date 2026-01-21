use crate::common::*;

use display_as_debug::types::{Full, Short};
use display_as_debug::wrap::TypeNameOption;

const EXPECTED_SOME_FULL: &str = "Some(alloc::vec::Vec<i32>)";
const EXPECTED_SOME_SHORT: &str = "Some(Vec<i32>)";
const EXPECTED_NONE: &str = "None";

mod new {
    use super::*;

    test_fmt!(none_full, TypeNameOption::new::<Full>(None::<i32>), "{:?}", EXPECTED_NONE);
    test_fmt!(none_short, TypeNameOption::new::<Short>(None::<i32>), "{:?}", EXPECTED_NONE);
    test_fmt!(some_full, TypeNameOption::new::<Full>(Some(vec![1])), "{:?}", EXPECTED_SOME_FULL);
    test_fmt!(some_short, TypeNameOption::new::<Short>(Some(vec![1])), "{:?}", EXPECTED_SOME_SHORT);
}

mod from {
    use super::*;

    test_fmt!(none_full, TypeNameOption::<Vec<i32>, Vec<i32>, Full>::from(None), "{:?}", EXPECTED_NONE);
    test_fmt!(none_short, TypeNameOption::<Vec<i32>, Vec<i32>, Short>::from(None), "{:?}", EXPECTED_NONE);
    test_fmt!(some_full, TypeNameOption::<Vec<i32>, Vec<i32>, Full>::from(Some(vec![1])), "{:?}", EXPECTED_SOME_FULL);
    test_fmt!(
        some_short,
        TypeNameOption::<Vec<i32>, Vec<i32>, Short>::from(Some(vec![1])),
        "{:?}",
        EXPECTED_SOME_SHORT
    );
}

mod borrowed {
    use super::*;

    test_fmt!(none_full, TypeNameOption::borrow::<Full>(&None::<Vec<i32>>), "{:?}", EXPECTED_NONE);
    test_fmt!(none_short, TypeNameOption::borrow::<Short>(&None::<Vec<i32>>), "{:?}", EXPECTED_NONE);
    test_fmt!(some_full, TypeNameOption::borrow::<Full>(&Some(vec![1])), "{:?}", EXPECTED_SOME_FULL);
    test_fmt!(some_short, TypeNameOption::borrow::<Short>(&Some(vec![1])), "{:?}", EXPECTED_SOME_SHORT);
}
