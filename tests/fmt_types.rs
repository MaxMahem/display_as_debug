mod common;

use crate::common::*;

mod type_name {
    use super::*;
    use display_as_debug::types::{Full, Short, TypeName};

    mod full {
        use super::*;

        const EXPECTED: &str = "alloc::vec::Vec<i32>";

        test_fmt!(debug, TypeName::<Vec<i32>>::FULL, "{:?}", EXPECTED);
        test_fmt!(display, TypeName::<Vec<i32>>::FULL, "{}", EXPECTED);

        mod default {
            use super::*;

            test_fmt!(debug, TypeName::<Vec<i32>, Full>::default(), "{:?}", EXPECTED);
            test_fmt!(display, TypeName::<Vec<i32>, Full>::default(), "{}", EXPECTED);
        }
    }

    mod short {
        use super::*;

        const EXPECTED: &str = "Vec<i32>";

        test_fmt!(debug, TypeName::<Vec<i32>>::SHORT, "{:?}", EXPECTED);
        test_fmt!(display, TypeName::<Vec<i32>>::SHORT, "{}", EXPECTED);

        mod default {
            use super::*;

            test_fmt!(debug, TypeName::<Vec<i32>, Short>::default(), "{:?}", EXPECTED);
            test_fmt!(display, TypeName::<Vec<i32>, Short>::default(), "{}", EXPECTED);
        }
    }
}

mod opaque {
    use super::*;
    use display_as_debug::types::{OPAQUE, Opaque};

    const EXPECTED: &str = "..";

    test_fmt!(default, Opaque::DEFAULT, "{:?}", EXPECTED);
    test_fmt!(opaque, OPAQUE, "{:?}", EXPECTED);
    test_fmt!(wrap, Opaque("secret"), "{:?}", EXPECTED);
}

mod opaque_list {
    use super::*;
    use display_as_debug::types::OpaqueList;

    const EXPECTED: &str = "[..: 100]";

    test_fmt!(new, OpaqueList(100), "{:?}", EXPECTED);
    test_fmt!(display, OpaqueList(100), "{}", EXPECTED);
    test_fmt!(of, OpaqueList::of(0..100), "{:?}", EXPECTED);
    test_fmt!(from, OpaqueList::from(0..100), "{:?}", EXPECTED);
}

mod type_name_list {
    use super::*;
    use display_as_debug::types::TypeNameList;

    mod short {
        use super::*;
        use display_as_debug::types::Short;

        const EXPECTED: &str = "[<String>: 100]";

        test_fmt!(new, TypeNameList::<String, Short>::new(100), "{:?}", EXPECTED);
        test_fmt!(of, TypeNameList::<String, Short>::of(0..100), "{:?}", EXPECTED);
        test_fmt!(from, TypeNameList::<String, Short>::from(0..100), "{:?}", EXPECTED);
        test_fmt!(display, TypeNameList::<String, Short>::of(0..100), "{}", EXPECTED);
    }

    mod full {
        use super::*;
        use display_as_debug::types::Full;

        const EXPECTED: &str = "[<alloc::string::String>: 100]";

        test_fmt!(new, TypeNameList::<String, Full>::new(100), "{:?}", EXPECTED);
        test_fmt!(of, TypeNameList::<String, Full>::of(0..100), "{:?}", EXPECTED);
        test_fmt!(from, TypeNameList::<String, Full>::from(0..100), "{:?}", EXPECTED);
    }
}
