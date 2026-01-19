//! Integration tests for TypeNameList

use crate::common::*;
use display_as_debug::types::TypeNameList;

mod short {
    use super::*;
    use display_as_debug::types::Short;

    const EXPECTED: &str = "[<String>: 100]";

    test_fmt!(new, TypeNameList::<String, Short>::new(100), "{:?}", EXPECTED);
    test_fmt!(of, TypeNameList::<String, Short>::of(0..100), "{:?}", EXPECTED);
    test_fmt!(from, TypeNameList::<String, Short>::from(0..100), "{:?}", EXPECTED);
    test_get!(len, TypeNameList::<String, Short>::new(100), TypeNameList::len, 100);
}

mod full {
    use super::*;
    use display_as_debug::types::Full;

    const EXPECTED: &str = "[<alloc::string::String>: 100]";

    test_fmt!(new, TypeNameList::<String, Full>::new(100), "{:?}", EXPECTED);
    test_fmt!(of, TypeNameList::<String, Full>::of(0..100), "{:?}", EXPECTED);
    test_fmt!(from, TypeNameList::<String, Full>::from(0..100), "{:?}", EXPECTED);
    test_get!(len, TypeNameList::<String, Full>::new(100), TypeNameList::len, 100);
}
