use core::fmt::{Debug, Display, Formatter};
use display_as_debug::fmt::DebugTupleExt;
use display_as_debug::types::{Full, Short, TestValue};

#[test]
fn field_display() {
    struct Tuple<T>(T);

    impl<T: Display> Debug for Tuple<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple(stringify!(Tuple)).field_display(&self.0).finish()
        }
    }

    assert_eq!(format!("{:?}", Tuple(TestValue::TEST)), r#"Tuple(Display("test"))"#);
}

#[test]
fn field_opaque() {
    struct Tuple<T>(T);

    impl<T> Debug for Tuple<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple(stringify!(Tuple)).field_opaque().finish()
        }
    }

    assert_eq!(format!("{:?}", Tuple(TestValue::TEST)), r#"Tuple(..)"#);
}

#[test]
fn field_type_full() {
    struct Tuple<T>(T);

    impl<T> Debug for Tuple<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple(stringify!(Tuple)).field_type::<T, Full>().finish()
        }
    }

    assert_eq!(format!("{:?}", Tuple(vec![1])), r#"Tuple(alloc::vec::Vec<i32>)"#);
}

#[test]
fn field_type_short() {
    struct Tuple<T>(T);

    impl<T> Debug for Tuple<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple(stringify!(Tuple)).field_type::<T, Short>().finish()
        }
    }

    assert_eq!(format!("{:?}", Tuple(vec![1])), r#"Tuple(Vec<i32>)"#);
}
