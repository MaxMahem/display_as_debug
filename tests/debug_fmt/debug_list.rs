use core::fmt::{Debug, Display, Formatter};
use display_as_debug::fmt::DebugListExt;
use display_as_debug::types::TestValue;
use std::vec;

#[test]
fn entry_display() {
    struct List<T>(T);

    impl<T: Display> Debug for List<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_list().entry_display(&self.0).finish()
        }
    }

    assert_eq!(format!("{:?}", List(TestValue::TEST)), r#"[Display("test")]"#);
}

#[test]
fn entries_display() {
    struct List(Vec<TestValue>);

    impl Debug for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_list().entries_display(&self.0).finish()
        }
    }

    let list = List(vec![TestValue::DEFAULT, TestValue::DEFAULT]);
    assert_eq!(format!("{list:?}"), r#"[Display(()), Display(())]"#);
}
