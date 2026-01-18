use core::fmt::{Debug, Display, Formatter};

use display_as_debug::fmt::DebugStructExt;
use display_as_debug::types::{Full, Short, TestValue};

#[test]
fn field_display() {
    struct Struct<T> {
        test: T,
    }

    impl<T: Display> Debug for Struct<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Struct").field_display("test", &self.test).finish()
        }
    }

    assert_eq!(format!("{:?}", Struct { test: TestValue::TEST }), r#"Struct { test: Display("test") }"#);
}

#[test]
fn field_type_full() {
    #[allow(dead_code, reason = "Testing")]
    struct Struct<T> {
        test: T,
    }

    impl<T> Debug for Struct<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Struct").field_type::<T, Full>("test").finish()
        }
    }

    assert_eq!(format!("{:?}", Struct { test: vec![1] }), "Struct { test: alloc::vec::Vec<i32> }");
}

#[test]
fn field_type_short() {
    #[allow(dead_code, reason = "Testing")]
    struct Struct<T> {
        test: T,
    }

    impl<T> Debug for Struct<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Struct").field_type::<T, Short>("test").finish()
        }
    }

    assert_eq!(format!("{:?}", Struct { test: vec![1] }), "Struct { test: Vec<i32> }");
}

#[test]
fn field_opaque() {
    #[allow(dead_code, reason = "Testing")]
    struct Struct<T> {
        test: T,
    }

    impl<T> Debug for Struct<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Struct").field_opaque("test").finish()
        }
    }

    assert_eq!(format!("{:?}", Struct { test: vec![1] }), "Struct { test: .. }");
}
