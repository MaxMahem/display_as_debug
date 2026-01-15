use core::fmt::{Debug, Formatter};
use display_as_debug::fmt::DebugSetExt;
use display_as_debug::types::TestValue;

#[test]
fn entry_display() {
    struct SingleValue;

    impl Debug for SingleValue {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_set().entry_display(&TestValue(())).finish()
        }
    }

    let value = SingleValue;
    assert_eq!(format!("{:?}", value), "{Display(())}");
}

#[test]
fn entries_display() {
    struct Values;

    impl Debug for Values {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_set().entries_display(&[TestValue(()), TestValue(())]).finish()
        }
    }

    assert_eq!(format!("{:?}", Values), "{Display(()), Display(())}");
}
