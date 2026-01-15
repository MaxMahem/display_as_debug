use core::error::Error;
use core::fmt::{Debug, Display, Formatter, Result};

/// A borrowed type adaptor that enables a type's [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// This is particularly useful when working with structs that have `Debug` bounds but you want to
/// use the cleaner `Display` formatting.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::debug::DisplayAsDebug;
/// use display_as_debug::types::TestValue;
///
/// assert_eq!(format!("{:?}", DisplayAsDebug(&TestValue::TEST)), r#"Display("test")"#, "display used for debug");
/// assert_eq!(format!("{}", DisplayAsDebug(&TestValue::TEST)), r#"Display("test")"#, "display unchanged");
/// ```
///
/// # Trait Implementations
///
/// - **[`Debug`]**: Uses the borrowed value's [`Display`] implementation
/// - **[`Display`]**: Forwards to the borrowed value's [`Display`] implementation
/// - **[`Error`]**: Implements [`Error`] if the borrowed type implements both [`Display`] and [`Error`]
pub struct DisplayAsDebug<'a, T: Display + ?Sized>(pub &'a T);

impl<T: Display> Debug for DisplayAsDebug<'_, T> {
    /// Formats the borrowed value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for DisplayAsDebug<'_, T> {
    /// Formats the borrowed value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display + Error> Error for DisplayAsDebug<'_, T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
