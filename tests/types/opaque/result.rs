//! Tests for OpaqueResult marker constants and borrow

use crate::common::*;
use display_as_debug::wrap::OpaqueResult;

mod markers {
    use super::*;

    test_fmt!(ok, OpaqueResult::OK, "{:?}", "Ok(..)");
}
