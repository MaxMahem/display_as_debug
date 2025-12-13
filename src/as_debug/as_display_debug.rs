use std::fmt::{Debug, Display, Formatter};

#[cfg(doc)]
use std::error::Error;

/// A borrowed type adaptor that enables a type's [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// This is the borrowed variant that wraps a reference to a value, making it ideal for temporary
/// use in debug contexts without taking ownership.
///
/// # Examples
///
/// Useful when a function or trait bound requires [`Debug`] but you have a type that implements
/// [`Display`]. [`AsDisplayDebug`] lets you use the [`Display`] representation in [`Debug`]
/// contexts:
///
/// ```rust
/// use display_as_debug::as_debug::DisplayDebug;
/// use std::net::IpAddr;
///
/// let ip = IpAddr::V4("127.0.0.1".parse().unwrap());
/// let formatted = format!("{:?}", ip.as_debug());
/// assert_eq!(formatted, "127.0.0.1");
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
///
/// # See Also
///
/// - [`DisplayDebugged`](crate::as_debug::DisplayDebugged) - The owned variant for when you need to take ownership
/// - [`DisplayDebug`](crate::as_debug::DisplayDebug) - The trait providing the [`as_debug()`](crate::as_debug::DisplayDebug::as_debug)
///   convenience method
pub struct AsDisplayDebug<'a, T: Display + ?Sized>(pub &'a T);

impl<T: Display> Debug for AsDisplayDebug<'_, T> {
    /// Formats the borrowed value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for AsDisplayDebug<'_, T> {
    /// Formats the borrowed value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display + std::error::Error> std::error::Error for AsDisplayDebug<'_, T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
