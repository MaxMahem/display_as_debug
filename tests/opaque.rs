mod common;

use display_as_debug::Opaque;

use crate::common::*;

test_fmt!(opaque_debug, Opaque, "{:?}", "..");
