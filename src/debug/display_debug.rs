use core::fmt::Display;

use crate::debug::DisplayAsDebug;

/// A trait to convert a type to use its [`Display`] implementation for [`Debug`].
#[sealed::sealed]
pub trait DisplayDebug: Display {
    /// Wraps a value in a [`DisplayAsDebug`] adaptor that enables the value's [`Display`]
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
    fn display_as_debug(&self) -> DisplayAsDebug<&Self> {
        DisplayAsDebug(self)
    }
}

/// Implements [`DisplayDebug`] for any type that implements [`Display`].
#[sealed::sealed]
impl<T: Display> DisplayDebug for T {}
