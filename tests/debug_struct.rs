use core::fmt::{Debug, Display, Formatter};

use display_as_debug::DebugStructExt;
use display_as_debug::type_name::{Full, Short};

#[test]
fn field_display() {
    struct DisplayValue(u32);

    impl Display for DisplayValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Value({})", self.0)
        }
    }

    struct TestStruct {
        value: DisplayValue,
    }

    impl Debug for TestStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TestStruct").field_display("value", &self.value).finish()
        }
    }

    let test = TestStruct { value: DisplayValue(42) };

    assert_eq!(format!("{:?}", test), "TestStruct { value: Value(42) }");
}

#[test]
fn field_type() {
    struct TestStruct<T> {
        #[allow(dead_code)]
        container: T,
    }

    impl<T> Debug for TestStruct<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TestStruct").field_type::<T, Full>("container").finish()
        }
    }

    let test = TestStruct::<Vec<i32>> { container: vec![1, 2, 3] };

    assert_eq!(format!("{:?}", test), "TestStruct { container: alloc::vec::Vec<i32> }");
}

#[test]
fn field_type_short() {
    struct TestStruct<T> {
        #[allow(dead_code)]
        container: T,
    }

    impl<T> Debug for TestStruct<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TestStruct").field_type::<T, Short>("container").finish()
        }
    }

    let test = TestStruct::<Vec<i32>> { container: vec![1, 2, 3] };

    assert_eq!(format!("{:?}", test), "TestStruct { container: Vec<i32> }");
}

#[test]
fn field_opaque() {
    use core::fmt;

    struct Credentials {
        #[allow(dead_code)]
        password: String,
    }

    impl Debug for Credentials {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_struct("Credentials").field_opaque("password").finish()
        }
    }

    let creds = Credentials { password: "secret123".to_string() };

    assert_eq!(format!("{:?}", creds), "Credentials { password: .. }");
}
