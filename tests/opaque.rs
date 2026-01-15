mod common;

use display_as_debug::types::Opaque;

use crate::common::*;

test_fmt!(opaque_debug, Opaque, "{:?}", "..");
