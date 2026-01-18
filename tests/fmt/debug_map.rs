use std::fmt::{Debug, Display, Formatter};

use display_as_debug::fmt::DebugMapExt;
use display_as_debug::types::TestValue;

#[test]
fn entry_display() {
    struct Map<K, V> {
        key: K,
        value: V,
    }

    impl<K: Debug, V: Display> Debug for Map<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_map().entry_display(&self.key, &self.value).finish()
        }
    }

    assert_eq!(format!("{:?}", Map { key: "key", value: TestValue::TEST }), r#"{"key": Display("test")}"#);
}

#[test]
fn entry_opaque() {
    #[allow(dead_code, reason = "Testing")]
    struct Map<K, V> {
        key: K,
        value: V,
    }

    impl<K: Debug, V> Debug for Map<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
            f.debug_map().entry_opaque(&self.key).finish()
        }
    }

    assert_eq!(format!("{:?}", Map { key: "key", value: TestValue::TEST }), r#"{"key": ..}"#);
}

#[test]
fn entries_display() {
    use std::collections::BTreeMap;

    struct Map<K, V>(BTreeMap<K, V>);

    impl<K: Debug, V: Display> Debug for Map<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_map().entries_display(&self.0).finish()
        }
    }

    let map = Map(BTreeMap::from([("key1", TestValue::TEST), ("key2", TestValue::TEST)]));
    assert_eq!(format!("{:?}", map), r#"{"key1": Display("test"), "key2": Display("test")}"#);
}

#[test]
fn entries_opaque() {
    use std::collections::BTreeMap;

    struct Map<K, V>(BTreeMap<K, V>);

    impl<K: Debug, V> Debug for Map<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_map().entries_opaque(&self.0).finish()
        }
    }

    let map = Map(BTreeMap::from([("key1", TestValue::TEST), ("key2", TestValue::TEST)]));
    assert_eq!(format!("{:?}", map), r#"{"key1": .., "key2": ..}"#);
}
