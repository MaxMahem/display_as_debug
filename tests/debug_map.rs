use std::fmt::{Debug, Formatter};

use display_as_debug::fmt::DebugMapExt;

#[test]
fn entry_display() {
    use display_as_debug::types::TestValue;

    struct ValueMap;

    impl Debug for ValueMap {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_map().entry_display(&"key1", &TestValue(())).finish()
        }
    }

    let map = ValueMap;
    assert_eq!(format!("{:?}", map), r#"{"key1": Display(())}"#);
}

#[test]
fn entry_opaque() {
    struct Credentials;

    impl Debug for Credentials {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_map().entry_opaque(&"secret").finish()
        }
    }

    let creds = Credentials;
    assert_eq!(format!("{:?}", creds), r#"{"secret": ..}"#);
}

#[test]
fn entries_display() {
    use display_as_debug::types::TestValue;
    use std::collections::BTreeMap;

    struct ValueMap(BTreeMap<&'static str, TestValue>);

    impl Debug for ValueMap {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_map().entries_display(&self.0).finish()
        }
    }

    let map = ValueMap(BTreeMap::from([("key1", TestValue(())), ("key2", TestValue(()))]));
    assert_eq!(format!("{:?}", map), r#"{"key1": Display(()), "key2": Display(())}"#);
}

#[test]
fn entries_opaque() {
    struct Credentials;

    impl Debug for Credentials {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_map().entries_opaque(&["secret", "password"]).finish()
        }
    }

    let creds = Credentials;
    assert_eq!(format!("{:?}", creds), r#"{"secret": .., "password": ..}"#);
}
