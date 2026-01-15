use core::fmt::{Debug, Formatter};
use display_as_debug::fmt::DebugTupleExt;
use display_as_debug::types::{Full, Short};

#[test]
fn field_display() {
    use display_as_debug::types::TestValue;

    struct Wrapper(TestValue);

    impl Debug for Wrapper {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_tuple("Wrapper").field_display(&self.0).finish()
        }
    }

    assert_eq!(format!("{:?}", Wrapper(TestValue(()))), "Wrapper(Display(()))");
}

#[test]
fn field_opaque() {
    #[allow(dead_code)]
    struct Secret(i32);

    impl Debug for Secret {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_tuple("Secret").field_opaque().finish()
        }
    }

    assert_eq!(format!("{:?}", Secret(42)), "Secret(..)");
}

#[test]
fn field_type() {
    struct Wrapper<T>(T);

    impl<T> Debug for Wrapper<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_tuple("Wrapper").field_type::<T, Full>().field_type::<T, Short>().finish()
        }
    }

    assert_eq!(format!("{:?}", Wrapper(vec![1, 2, 3])), "Wrapper(alloc::vec::Vec<i32>, Vec<i32>)");
}
