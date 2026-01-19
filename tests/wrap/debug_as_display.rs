use crate::common::*;
use display_as_debug::types::TestValue;
use display_as_debug::wrap::DebugAsDisplay;

const EXPECTED_DEBUG: &str = r#"Debug("test")"#;

test_fmt!(display, DebugAsDisplay(TestValue::TEST), "{}", EXPECTED_DEBUG);
test_fmt!(debug, DebugAsDisplay(TestValue::TEST), "{:?}", EXPECTED_DEBUG);
test_source!(DebugAsDisplay(std::io::Error::other("test")), None::<_>);
