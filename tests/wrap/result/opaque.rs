use crate::common::*;
use display_as_debug::wrap::OpaqueResult;

const EXPECTED_OK: &str = "Ok(..)";
const EXPECTED_ERR: &str = r#"Err("error")"#;

mod ctor {
    use super::*;

    test_fmt!(ok, OpaqueResult(Ok::<i32, String>(42)), "{:?}", EXPECTED_OK);
    test_fmt!(err, OpaqueResult(Err::<i32, &str>("error")), "{:?}", EXPECTED_ERR);
}

mod borrow {
    use super::*;

    test_fmt!(ok, OpaqueResult::borrow(&Ok::<i32, &str>(42)), "{:?}", EXPECTED_OK);
    test_fmt!(err, OpaqueResult::borrow(&Err::<i32, &str>("error")), "{:?}", EXPECTED_ERR);
}
