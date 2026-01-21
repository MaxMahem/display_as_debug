use crate::common::*;
use display_as_debug::types::{Full, Short};
use display_as_debug::wrap::TypeNameResult;

const EXPECTED_OK_FULL: &str = "Ok(alloc::vec::Vec<i32>)";
const EXPECTED_OK_SHORT: &str = "Ok(Vec<i32>)";
const EXPECTED_ERR: &str = "Err(42)";

test_fmt!(ok_full, TypeNameResult::new::<Full>(Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_FULL);
test_fmt!(ok_short, TypeNameResult::new::<Short>(Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_SHORT);
test_fmt!(err_full, TypeNameResult::new::<Full>(Err::<Vec<i32>, i32>(42)), "{:?}", EXPECTED_ERR);
test_fmt!(err_short, TypeNameResult::new::<Short>(Err::<Vec<i32>, i32>(42)), "{:?}", EXPECTED_ERR);
test_fmt!(from, TypeNameResult::<Vec<i32>, i32, Vec<i32>, Full>::from(Ok(vec![])), "{:?}", EXPECTED_OK_FULL);
test_fmt!(borrow, TypeNameResult::borrow::<Full>(&Ok::<Vec<i32>, i32>(vec![])), "{:?}", EXPECTED_OK_FULL);
