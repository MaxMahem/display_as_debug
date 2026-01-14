use super::{DebugAsDisplay, DebugDisplayed};
use core::fmt::Debug;

#[cfg(doc)]
use core::fmt::Display;

#[cfg(doc)]
use core::error::Error;

/// A trait to convert a type to use its [`Debug`] implementation for [`Display`].
pub trait DebugDisplay: Debug {
    /// Wraps a borrowed value in an adaptor that enable the values [`Debug`] implementation to be
    /// used for its [`Display`] implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::as_display::DebugDisplay;
    /// assert_eq!(format!("{}", vec![1].debug_as_display()), "[1]", "display unchanged");
    /// assert_eq!(format!("{:?}", vec![1].debug_as_display()), "[1]", "debug used for debug");
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
    /// # use display_as_debug::as_display::DebugDisplay;
    /// assert_eq!(format!("{:?}", vec![1].clone().wrap_debug_as_display()), "[1]", "debug used for debug");
    /// assert_eq!(format!("{}", vec![1].clone().wrap_debug_as_display()), "[1]", "display unchanged");
    /// ```
    fn wrap_debug_as_display(self) -> DebugDisplayed<Self>
    where
        Self: Sized,
    {
        DebugDisplayed(self)
    }
}

/// Implements [`DebugDisplay`] for any type that implements [`Debug`].
impl<T: Debug> DebugDisplay for T {}
