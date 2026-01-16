use super::DebugAsDisplay;
use core::fmt::Debug;

#[cfg(doc)]
use core::fmt::Display;

/// A trait to convert a type to use its [`Debug`] implementation for [`Display`].
#[sealed::sealed]
pub trait DebugDisplay: Debug {
    /// Wraps a value in a [`DebugAsDisplay`] adaptor that enables the value's [`Debug`]
    /// implementation to be used as its [`Display`] implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::display::DebugDisplay;
    /// assert_eq!(format!("{}", vec![1].debug_as_display()), "[1]", "debug used for display");
    /// assert_eq!(format!("{:?}", vec![2].debug_as_display()), "[2]", "debug unchanged");
    /// ```
    fn debug_as_display(&self) -> DebugAsDisplay<&Self> {
        DebugAsDisplay(self)
    }
}

/// Implements [`DebugDisplay`] for any type that implements [`Debug`].
#[sealed::sealed]
impl<T: Debug> DebugDisplay for T {}
