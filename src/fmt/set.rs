use core::fmt::{DebugSet, Display};

use super::fold_mut::IteratorFoldMut;
use crate::debug::DisplayAsDebug;

/// Extension trait for [`DebugSet`] providing convenient entry formatting methods.
#[sealed::sealed]
pub trait DebugSetExt {
    /// Adds an entry using the value's [`Display`] implementation instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugSetExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct SingletonSet<T>(T);
    ///
    /// impl<T: Display> Debug for SingletonSet<T> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_set().entry_display(&self.0).finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", SingletonSet(TestValue(1))), "{Display(1)}");
    /// ```
    fn entry_display(&mut self, value: &dyn Display) -> &mut Self;

    /// Adds multiple entries using their [`Display`] implementations instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugSetExt;
    /// use display_as_debug::types::TestValue;
    /// use std::collections::BTreeSet;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct Set<T>(BTreeSet<T>);
    ///
    /// impl<T: Display> Debug for Set<T> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_set().entries_display(&self.0).finish()
    ///     }
    /// }
    ///
    /// let set = Set(BTreeSet::from([TestValue(1), TestValue(2)]));
    ///
    /// assert_eq!(format!("{:?}", set), "{Display(1), Display(2)}");
    /// ```
    fn entries_display<I: IntoIterator<Item: Display>>(&mut self, iter: I) -> &mut Self;
}

#[sealed::sealed]
impl DebugSetExt for DebugSet<'_, '_> {
    fn entry_display(&mut self, value: &dyn Display) -> &mut Self {
        self.entry(&DisplayAsDebug(value))
    }

    fn entries_display<I: IntoIterator<Item: Display>>(&mut self, iter: I) -> &mut Self {
        iter.into_iter().fold_mut(self, |this, item| _ = this.entry_display(&item))
    }
}
