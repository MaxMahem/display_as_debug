mod common;

use display_as_debug::types::TypeName;

use crate::common::*;

test_fmt!(full_debug, TypeName::<Vec<i32>>::FULL, "{:?}", "alloc::vec::Vec<i32>");
test_fmt!(full_display, TypeName::<Vec<i32>>::FULL, "{}", "alloc::vec::Vec<i32>");
test_fmt!(short_debug, TypeName::<Vec<i32>>::SHORT, "{:?}", "Vec<i32>");
test_fmt!(short_display, TypeName::<Vec<i32>>::SHORT, "{}", "Vec<i32>");
