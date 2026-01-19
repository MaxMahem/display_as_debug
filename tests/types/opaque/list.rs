use crate::common::*;
use display_as_debug::types::OpaqueList;

const EXPECTED: &str = "[..: 100]";

test_fmt!(new, OpaqueList(100), "{:?}", EXPECTED);
test_fmt!(display, OpaqueList(100), "{}", EXPECTED);
test_fmt!(of, OpaqueList::of(0..100), "{:?}", EXPECTED);
test_fmt!(from, OpaqueList::from(0..100), "{:?}", EXPECTED);
