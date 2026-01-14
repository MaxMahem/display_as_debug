use core::error::Error;
use core::fmt::{Debug, Display, Formatter, Result};

/// A borrowed type adaptor that enables a type's [`Debug`] implementation to be used for its
/// [`Display`] implementation.
///
/// Useful when an interface requires or assumes a type implements [`Display`] but only a [`Debug`]
/// implementation is available. Such as with most standard library types. [`DebugAsDisplay`] lets
/// you use their debug representation in [`Display`] contexts.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::as_display::DebugAsDisplay;
/// assert_eq!(format!("{}", DebugAsDisplay(&vec![1])), "[1]", "debug used for display");
/// assert_eq!(format!("{:?}", DebugAsDisplay(&vec![1])), "[1]", "debug unchanged");
/// ```
/// # Trait Implementations
///
/// - **[`Display`]**: Uses the borrowed value's [`Debug`] implementation
/// - **[`Debug`]**: Forwards to the borrowed value's [`Debug`] implementation
/// - **[`Error`]**: Implements [`Error`] if the borrowed type implements both [`Debug`] and [`Error`]
pub struct DebugAsDisplay<'a, T: Debug + ?Sized>(pub &'a T);

impl<T: Debug> Display for DebugAsDisplay<'_, T> {
    /// Formats the borrowed value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for DebugAsDisplay<'_, T> {
    /// Formats the borrowed value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug + Error> Error for DebugAsDisplay<'_, T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
