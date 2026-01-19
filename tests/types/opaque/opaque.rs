//! Trivial test for OPAQUE constant; full Opaque wrapper tests are in wrappers/opaque.rs

use crate::common::*;
use display_as_debug::types::OPAQUE;

test_fmt!(debug, OPAQUE, "{:?}", "..");
test_fmt!(display, OPAQUE, "{}", "..");
