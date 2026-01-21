use crate::common::*;
use display_as_debug::wrap::OpaqueResult;

const EXPECTED_OK: &str = "Ok(..)";
const EXPECTED_ERR: &str = r#"Err("error message")"#;

test_fmt!(ok, OpaqueResult(Ok::<i32, String>(42)), "{:?}", EXPECTED_OK);
test_fmt!(err, OpaqueResult(Err::<i32, &str>("error message")), "{:?}", EXPECTED_ERR);
