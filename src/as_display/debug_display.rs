use super::{AsDebugDisplay, DebugDisplayed};
use std::fmt::Debug;

/// A trait to convert a type to use its [`Debug`] implementation for [`Display`].
pub trait DebugDisplay: Debug {
    /// Wraps a borrowed value in an adaptor that enable the values [`Debug`] implementation to be
    /// used for its [`Display`] implementation.
    ///
    /// See [`AsDebugDisplay`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_display::DebugDisplay;
    ///
    /// let numbers = vec![1, 2, 3];
    /// // debug implementation is used for display
    /// assert_eq!(format!("{}", numbers.as_display()), "[1, 2, 3]");
    ///
    /// // debug implementation is still used for debug
    /// assert_eq!(format!("{:?}", numbers.as_display()), "[1, 2, 3]");
    /// ```
    fn as_display(&'_ self) -> AsDebugDisplay<'_, Self> {
        AsDebugDisplay(self)
    }

    /// Wraps a owned value in an adaptor that enable the values [`Debug`] implementation to be
    /// used for its [`Display`] implementation.
    ///
    /// If ownership is not necessary, favor [`DebugDisplay::as_display`].
    /// See [`DebugDisplayed`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_display::DebugDisplay;
    ///
    /// let numbers = vec![1, 2, 3];
    /// assert_eq!(format!("{}", numbers.clone().to_display()), "[1, 2, 3]");
    ///
    /// // borrowed value's debug implementation is still used for debug
    /// assert_eq!(format!("{:?}", numbers.clone().to_display()), "[1, 2, 3]");
    /// ```
    fn to_display(self) -> DebugDisplayed<Self>
    where
        Self: Sized,
    {
        DebugDisplayed(self)
    }
}

/// Implements [`DebugDisplay`] for any type that implements [`Debug`].
impl<T: Debug> DebugDisplay for T {}
