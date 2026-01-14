use core::fmt::{Debug, Formatter};
use display_as_debug::DebugTupleExt;
use display_as_debug::type_name::{Full, Short};

#[test]
fn field_display() {
    struct UserId(u32);
    impl core::fmt::Display for UserId {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            write!(f, "user_{}", self.0)
        }
    }
    struct Session(UserId);
    impl Debug for Session {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_tuple("Session").field_display(&self.0).finish()
        }
    }

    assert_eq!(format!("{:?}", Session(UserId(42))), "Session(user_42)");
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
