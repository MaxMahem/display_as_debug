use crate::common::*;
use display_as_debug::wrap::OpaqueOption;

const EXPECTED_SOME: &str = "Some(..)";
const EXPECTED_NONE: &str = "None";

mod ctor {
    use super::*;

    test_fmt!(some, OpaqueOption(Some(42)), "{:?}", EXPECTED_SOME);
    test_fmt!(none, OpaqueOption(None::<i32>), "{:?}", EXPECTED_NONE);
}

mod borrow {
    use super::*;

    test_fmt!(some, OpaqueOption::borrow(&Some(42)), "{:?}", EXPECTED_SOME);
    test_fmt!(none, OpaqueOption::borrow(&None::<i32>), "{:?}", EXPECTED_NONE);
}
