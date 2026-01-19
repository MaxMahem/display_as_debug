//! Integration tests for TypeNameSet

use crate::common::*;
use display_as_debug::types::TypeNameSet;

mod short {
    use super::*;
    use display_as_debug::types::Short;

    const EXPECTED: &str = "{<String>: 100}";

    test_fmt!(new, TypeNameSet::<String, Short>::new(100), "{:?}", EXPECTED);
    test_fmt!(of, TypeNameSet::<String, Short>::of(0..100), "{:?}", EXPECTED);
    test_fmt!(from, TypeNameSet::<String, Short>::from(0..100), "{:?}", EXPECTED);
    test_get!(len, TypeNameSet::<String, Short>::new(100), TypeNameSet::len, 100);
}

mod full {
    use super::*;
    use display_as_debug::types::Full;

    const EXPECTED: &str = "{<alloc::string::String>: 100}";

    test_fmt!(new, TypeNameSet::<String, Full>::new(100), "{:?}", EXPECTED);
    test_fmt!(of, TypeNameSet::<String, Full>::of(0..100), "{:?}", EXPECTED);
    test_fmt!(from, TypeNameSet::<String, Full>::from(0..100), "{:?}", EXPECTED);
    test_get!(len, TypeNameSet::<String, Full>::new(100), TypeNameSet::len, 100);
}
