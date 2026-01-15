use super::{DebugAsDisplay, DebugForDisplay};
use core::fmt::Debug;

#[cfg(doc)]
use core::fmt::Display;

#[cfg(doc)]
use core::error::Error;

/// A trait to convert a type to use its [`Debug`] implementation for [`Display`].
#[sealed::sealed]
pub trait DebugDisplay: Debug {
    /// Wraps a borrowed value in an adaptor that enable the values [`Debug`] implementation to be
    /// used for its [`Display`] implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::display::DebugDisplay;
    /// assert_eq!(format!("{}", vec![1].debug_as_display()), "[1]", "display unchanged");
    /// assert_eq!(format!("{:?}", vec![2].debug_as_display()), "[2]", "debug used for debug");
    /// ```
    fn debug_as_display(&'_ self) -> DebugAsDisplay<'_, Self> {
        DebugAsDisplay(self)
    }

    /// Wraps a owned value in an adaptor that enable the values [`Debug`] implementation to be
    /// used for its [`Display`] implementation.
    ///
    /// If ownership is not necessary, favor [`DebugDisplay::debug_as_display`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::display::DebugDisplay;
    /// let debug_vec = vec![1];
    /// let display_vec = vec![2];
    ///
    /// assert_eq!(format!("{:?}", debug_vec.wrap_debug_as_display()), "[1]", "debug used for debug");
    /// assert_eq!(format!("{}", display_vec.wrap_debug_as_display()), "[2]", "debug used for display");
    /// ```
    fn wrap_debug_as_display(self) -> DebugForDisplay<Self>
    where
        Self: Sized,
    {
        DebugForDisplay(self)
    }
}

/// Implements [`DebugDisplay`] for any type that implements [`Debug`].
#[sealed::sealed]
impl<T: Debug> DebugDisplay for T {}
