use std::fmt::{Debug, Display, Formatter};

#[cfg(doc)]
use std::error::Error;

/// A borrowed type adaptor that enables a type's [`Debug`] implementation to be used for its
/// [`Display`] implementation.
///
/// This is the borrowed variant that wraps a reference to a value, making it ideal for temporary
/// use in display contexts without taking ownership.
///
/// # Examples
///
/// Useful when an interface requires or assumes a type implements [`Display`] but only a [`Debug`]
/// implementation is available. Such as with most standard library types. [`AsDebugDisplay`] lets
/// you use their debug representation in [`Display`] contexts:
///
/// ```rust
/// use display_as_debug::as_display::DebugDisplay;
///
/// let numbers = vec![1, 2, 3];
/// let formatted = format!("Numbers: {}", numbers.as_display());
/// assert_eq!(formatted, "Numbers: [1, 2, 3]");
/// ```
///
/// # Trait Implementations
///
/// - **[`Display`]**: Uses the borrowed value's [`Debug`] implementation
/// - **[`Debug`]**: Forwards to the borrowed value's [`Debug`] implementation
/// - **[`Error`]**: Implements [`Error`] if the borrowed type implements both [`Debug`] and [`Error`]
///
/// # See Also
///
/// - [`DebugDisplayed`](crate::as_display::DebugDisplayed) - The owned variant for when you need to take ownership
/// - [`DebugDisplay`](crate::as_display::DebugDisplay) - The trait providing the [`as_display()`](crate::as_display::DebugDisplay::as_display)
///   convenience method
pub struct AsDebugDisplay<'a, T: Debug + ?Sized>(pub &'a T);

impl<T: Debug> Display for AsDebugDisplay<'_, T> {
    /// Formats the borrowed value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for AsDebugDisplay<'_, T> {
    /// Formats the borrowed value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug + std::error::Error> std::error::Error for AsDebugDisplay<'_, T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
