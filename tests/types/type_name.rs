use crate::common::*;
use display_as_debug::types::{Full, Short, TypeName};

mod full {
    use super::*;

    const EXPECTED: &str = "alloc::vec::Vec<i32>";

    mod consts {
        use super::*;

        test_fmt!(debug, TypeName::<Vec<i32>>::FULL, "{:?}", EXPECTED);
        test_fmt!(display, TypeName::<Vec<i32>>::FULL, "{}", EXPECTED);
    }

    mod empty {
        use super::*;

        test_fmt!(debug, TypeName::empty::<Vec<i32>, Full>(), "{:?}", EXPECTED);
        test_fmt!(display, TypeName::empty::<Vec<i32>, Full>(), "{}", EXPECTED);
    }

    mod wrap {
        use super::*;

        test_fmt!(debug, TypeName::wrap::<Full>(vec![1]), "{:?}", EXPECTED);
        test_fmt!(display, TypeName::wrap::<Full>(vec![1]), "{}", EXPECTED);
        test_get!(into_inner, TypeName::wrap::<Full>(42i32), move TypeName::into_inner, 42);
    }

    mod from {
        use super::*;

        test_fmt!(debug, TypeName::<i32, i32, Full>::from(42), "{:?}", "i32");
        test_get!(into_inner, TypeName::<i32, i32, Full>::from(42), move TypeName::into_inner, 42);
    }
}

mod short {
    use super::*;

    const EXPECTED: &str = "Vec<i32>";

    mod consts {
        use super::*;

        test_fmt!(debug, TypeName::<Vec<i32>>::SHORT, "{:?}", EXPECTED);
        test_fmt!(display, TypeName::<Vec<i32>>::SHORT, "{}", EXPECTED);
    }

    mod empty {
        use super::*;

        test_fmt!(debug, TypeName::empty::<Vec<i32>, Short>(), "{:?}", EXPECTED);
        test_fmt!(display, TypeName::empty::<Vec<i32>, Short>(), "{}", EXPECTED);
    }
}
