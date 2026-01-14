use core::error::Error;
use core::fmt::{Debug, Display, Formatter, Result};

/// A borrowed type adaptor that enables a type's [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// This is the borrowed variant that wraps a reference to a value, making it ideal for temporary
/// use in debug contexts without taking ownership.
///
/// # Examples
///
/// Useful when a function or trait bound requires [`Debug`] but you have a type that implements
/// [`Display`]. [`DisplayAsDebug`] lets you use the [`Display`] representation in [`Debug`]
/// contexts:
///
/// ```rust
/// use display_as_debug::as_debug::DisplayAsDebug;
/// use std::net::IpAddr;
///
/// let ip = IpAddr::V4("127.0.0.1".parse().unwrap());
/// assert_eq!(format!("{:?}", DisplayAsDebug(&ip)), "127.0.0.1", "display used for debug");
/// assert_eq!(format!("{}", DisplayAsDebug(&ip)), "127.0.0.1", "display unchanged");
/// ```
///
/// This is particularly useful when working with structs that have `Debug` bounds but you want to
/// use the cleaner `Display` formatting.
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
