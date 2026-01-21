//! Tests for OpaqueOption marker constants

use crate::common::*;
use display_as_debug::wrap::OpaqueOption;

test_fmt!(some, OpaqueOption::SOME, "{:?}", "Some(..)");
test_fmt!(none, OpaqueOption::NONE, "{:?}", "None");
