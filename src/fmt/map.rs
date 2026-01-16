use core::fmt::{Debug, DebugMap, Display};

use super::fold_mut::IteratorFoldMut;
use crate::debug::DisplayAsDebug;
use crate::types::OPAQUE;

/// Extension trait for [`DebugMap`] providing convenient entry formatting methods.
#[sealed::sealed]
pub trait DebugMapExt {
    /// Adds an entry using the value's [`Display`] implementation instead of [`Debug`].
    ///
    /// The key uses its [`Debug`] implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugMapExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct SingletonMap<K, V>(K, V);
    ///
    /// impl<K: Debug, V: Display> Debug for SingletonMap<K, V> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_map().entry_display(&self.0, &self.1).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", SingletonMap(1, TestValue::DEFAULT)), "{1: Display(())}");
    /// ```
    fn entry_display(&mut self, key: &dyn Debug, value: &dyn Display) -> &mut Self;

    /// Adds an entry with the key using [`Display`] and an opaque value showing `".."`.
    ///
    /// Useful for hiding sensitive values while still showing the key.
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugMapExt;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct SingleCred<K, V>(K, V);
    ///
    /// impl<K: Debug, V> Debug for SingleCred<K, V> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_map().entry_opaque(&self.0).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", SingleCred(1, "secret")), "{1: ..}");
    /// ```
    fn entry_opaque(&mut self, key: &dyn Debug) -> &mut Self;

    /// Adds multiple entries using their [`Display`] implementations for values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugMapExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Display, Formatter};
    /// use std::collections::BTreeMap;
    ///
    /// struct Map<K, V>(BTreeMap<K, V>);
    ///
    /// impl<K: Debug, V: Display> Debug for Map<K, V> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_map().entries_display(&self.0).finish()
    ///     }
    /// }
    ///
    /// let map = Map(BTreeMap::from([(1, TestValue::DEFAULT), (2, TestValue::DEFAULT)]));
    ///
    /// assert_eq!(format!("{:?}", map), "{1: Display(()), 2: Display(())}");
    /// ```
    fn entries_display<K: Debug, V: Display, I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>;

    /// Adds multiple entries with opaque values, showing only the keys.
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugMapExt;
    /// use std::fmt::{Debug, Formatter};
    /// use std::collections::BTreeMap;
    ///
    /// struct Credentials(BTreeMap<i32, &'static str>);
    ///
    /// impl Debug for Credentials {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_map().entries_opaque(&self.0).finish()
    ///     }
    /// }
    ///
    /// let credentials = Credentials(BTreeMap::from([(1, "secret1"), (2, "secret2")]));
    ///
    /// assert_eq!(format!("{:?}", credentials), "{1: .., 2: ..}");
    /// ```
    fn entries_opaque<K: Debug, V, I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>;
}

#[sealed::sealed]
impl DebugMapExt for DebugMap<'_, '_> {
    fn entry_display(&mut self, key: &dyn Debug, value: &dyn Display) -> &mut Self {
        self.entry(key, &DisplayAsDebug(value))
    }

    fn entry_opaque(&mut self, key: &dyn Debug) -> &mut Self {
        self.entry(key, &OPAQUE)
    }

    fn entries_display<K: Debug, V: Display, I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        iter.into_iter().fold_mut(self, |this, (key, value)| _ = this.entry_display(&key, &value))
    }

    fn entries_opaque<K: Debug, V, I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        iter.into_iter().fold_mut(self, |this, (key, _)| _ = this.entry_opaque(&key))
    }
}
