use core::fmt::{Debug, Formatter};
use display_as_debug::fmt::DebugListExt;
use display_as_debug::types::TestValue;

#[test]
fn entry_display() {
    struct Test;

    impl Debug for Test {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_list().entry_display(&TestValue(())).finish()
        }
    }

    assert_eq!(format!("{:?}", Test), "[Display(())]");
}

#[test]
fn entries_display() {
    struct Test;

    impl Debug for Test {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_list().entries_display(&[TestValue(()), TestValue(())]).finish()
        }
    }

    assert_eq!(format!("{:?}", Test), "[Display(()), Display(())]");
}
