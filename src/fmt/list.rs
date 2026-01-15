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
    /// ```
    /// use display_as_debug::fmt::DebugListExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Formatter};
    ///
    /// struct Test(Vec<TestValue>);
    ///
    /// impl Debug for Test {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         let mut list = f.debug_list();
    ///         for value in &self.0 {
    ///             list.entry_display(value);
    ///         }
    ///         list.finish()
    ///     }
    /// }
    ///
    /// let test = Test(vec![TestValue::DEFAULT, TestValue::DEFAULT]);
    ///
    /// assert_eq!(format!("{:?}", test), "[Display(()), Display(())]");
    /// ```
    fn entry_display<T: Display>(&mut self, value: &T) -> &mut Self;

    /// Adds multiple entries using their [`Display`] implementations instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::fmt::DebugListExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Formatter};
    ///
    /// struct Test(Vec<TestValue>);
    ///
    /// impl Debug for Test {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_list()
    ///             .entries_display(&self.0)
    ///             .finish()
    ///     }
    /// }
    ///
    /// let test = Test(vec![TestValue::DEFAULT, TestValue::DEFAULT]);
    ///
    /// assert_eq!(format!("{:?}", test), "[Display(()), Display(())]");
    /// ```
    fn entries_display<'a, T: Display + 'a, I: IntoIterator<Item = &'a T>>(&mut self, iter: I) -> &mut Self;
}

#[sealed::sealed]
impl DebugListExt for DebugList<'_, '_> {
    fn entry_display<T: Display>(&mut self, value: &T) -> &mut Self {
        self.entry(&DisplayAsDebug(value))
    }

    fn entries_display<'a, T, I>(&mut self, iter: I) -> &mut Self
    where
        T: Display + 'a,
        I: IntoIterator<Item = &'a T>,
    {
        iter.into_iter().fold_mut(self, |this, item| _ = this.entry_display(item))
    }
}
