use crate::common::*;
use display_as_debug::wrap::Opaque;

const EXPECTED: &str = "..";

test_fmt!(default, Opaque::DEFAULT, "{:?}", EXPECTED);
test_fmt!(wrap, Opaque("secret"), "{:?}", EXPECTED);
test_fmt!(display, Opaque("secret"), "{}", EXPECTED);
