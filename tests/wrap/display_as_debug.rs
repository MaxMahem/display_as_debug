use crate::common::*;
use display_as_debug::types::TestValue;
use display_as_debug::wrap::DisplayAsDebug;

const EXPECTED_DISPLAY: &str = r#"Display("test")"#;

test_fmt!(display, DisplayAsDebug(TestValue::TEST), "{}", EXPECTED_DISPLAY);
test_fmt!(debug, DisplayAsDebug(TestValue::TEST), "{:?}", EXPECTED_DISPLAY);
test_source!(DisplayAsDebug(std::io::Error::other("test")), None::<_>);
