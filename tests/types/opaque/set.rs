use crate::common::*;
use display_as_debug::types::OpaqueSet;

const EXPECTED: &str = "{..: 100}";

test_fmt!(new, OpaqueSet(100), "{:?}", EXPECTED);
test_fmt!(display, OpaqueSet(100), "{}", EXPECTED);
test_fmt!(of, OpaqueSet::of(0..100), "{:?}", EXPECTED);
test_fmt!(from, OpaqueSet::from(0..100), "{:?}", EXPECTED);
