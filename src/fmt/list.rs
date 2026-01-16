use core::fmt::{DebugList, Display};

use super::fold_mut::IteratorFoldMut;
use crate::debug::DisplayAsDebug;

/// Extension trait for [`DebugList`] providing convenient entry formatting methods.
#[sealed::sealed]
pub trait DebugListExt {
    /// Adds an entry using the value's [`Display`] implementation instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugListExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Formatter};
    ///
    /// struct SingleItemList(TestValue);
    ///
    /// impl Debug for SingleItemList {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_list().entry_display(&self.0).finish()
    ///     }
    /// }
    ///
    /// let list = SingleItemList(TestValue::DEFAULT);
    ///
    /// assert_eq!(format!("{list:?}"), "[Display(())]");
    /// ```
    fn entry_display(&mut self, value: &dyn Display) -> &mut Self;

    /// Adds multiple entries using their [`Display`] implementations instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugListExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Formatter};
    ///
    /// struct List(Vec<TestValue>);
    ///
    /// impl Debug for List {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_list().entries_display(&self.0).finish()
    ///     }
    /// }
    ///
    /// let list = List(vec![TestValue::DEFAULT, TestValue::DEFAULT]);
    ///
    /// assert_eq!(format!("{:?}", list), "[Display(()), Display(())]");
    /// ```
    fn entries_display<I: IntoIterator<Item: Display>>(&mut self, iter: I) -> &mut Self;
}

#[sealed::sealed]
impl DebugListExt for DebugList<'_, '_> {
    fn entry_display(&mut self, value: &dyn Display) -> &mut Self {
        self.entry(&DisplayAsDebug(value))
    }

    fn entries_display<I: IntoIterator<Item: Display>>(&mut self, iter: I) -> &mut Self {
        iter.into_iter().fold_mut(self, |this, item| _ = this.entry_display(&item))
    }
}
