use core::fmt::{Debug, Display, Formatter};
use display_as_debug::fmt::DebugSetExt;
use display_as_debug::types::TestValue;

#[test]
fn entry_display() {
    struct Set<T>(T);

    impl<T: Display> Debug for Set<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_set().entry_display(&self.0).finish()
        }
    }

    assert_eq!(format!("{:?}", Set(TestValue::TEST)), r#"{Display("test")}"#);
}

#[test]
fn entries_display() {
    use std::collections::BTreeSet;

    struct Set<T>(BTreeSet<T>);

    impl<T: Display> Debug for Set<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_set().entries_display(&self.0).finish()
        }
    }

    let set = Set(BTreeSet::from([TestValue(1), TestValue(2)]));

    assert_eq!(format!("{:?}", set), "{Display(1), Display(2)}");
}
