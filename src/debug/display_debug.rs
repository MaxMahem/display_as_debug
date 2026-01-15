use core::fmt::Display;

use crate::debug::{DisplayAsDebug, DisplayForDebug};

/// A trait to convert a type to use its [`Display`] implementation for [`Debug`].
#[sealed::sealed]
pub trait DisplayDebug: Display {
    /// Wraps a borrowed value in a [`DisplayForDebug`] adaptor that enable the values [`Display`]
    /// implementation to be used for its [`Debug`] implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::debug::DisplayDebug;
    /// use display_as_debug::types::TestValue;
    ///
    /// let test = TestValue::TEST.display_as_debug();
    /// assert_eq!(format!("{:?}", test), r#"Display("test")"#, "display used for debug");
    /// assert_eq!(format!("{}", test), r#"Display("test")"#, "display unchanged");
    /// ```
    fn display_as_debug(&'_ self) -> DisplayAsDebug<'_, Self> {
        DisplayAsDebug(self)
    }

    /// Wraps a owned value in a [`DisplayForDebug`] adaptor that enable the values [`Display`]
    /// implementation to be used for its [`Debug`] implementation.
    ///
    /// Unless ownership is not necessary, favor [`DisplayDebug::display_as_debug`].
    ///
    /// ```rust
    /// use display_as_debug::debug::DisplayDebug;
    /// use display_as_debug::types::TestValue;
    ///
    /// let debug_val = TestValue("test").wrap_display_as_debug();
    /// let display_val = TestValue("test").wrap_display_as_debug();
    ///
    /// assert_eq!(format!("{:?}", debug_val), r#"Display("test")"#, "display used for debug");
    /// assert_eq!(format!("{}", display_val), r#"Display("test")"#, "display unchanged");
    /// ```
    fn wrap_display_as_debug(self) -> DisplayForDebug<Self>
    where
        Self: Sized,
    {
        DisplayForDebug(self)
    }
}

/// Implements [`DisplayDebug`] for any type that implements [`Display`].
#[sealed::sealed]
impl<T: Display> DisplayDebug for T {}
