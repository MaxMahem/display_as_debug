use core::error::Error;
use core::fmt::{Debug, Display, Formatter, Result};

/// An owning type adaptor that enables a type's [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// # Use Cases
///
/// ## Returning Errors from `main()`
///
/// When `main()` returns a [`Result<(), E>`](core::result::Result), Rust prints the error using its
/// [`Debug`] implementation. By wrapping error types with [`DisplayForDebug`], you can ensure the
/// user-friendly [`Display`] representation is shown instead of the more technical [`Debug`]
/// output.
///
/// See `examples/error_from_main.rs` for a complete example.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::debug::DisplayForDebug;
/// assert_eq!(format!("{:?}", DisplayForDebug("hello")), "hello", "display used for debug");
/// assert_eq!(format!("{}", DisplayForDebug("hello")), "hello", "display unchanged");
/// ```
///
/// # Trait Implementations
///
/// - **[`Debug`]**: Uses the wrapped value's [`Display`] implementation
/// - **[`Display`]**: Forwards to the wrapped value's [`Display`] implementation  
/// - **[`Error`]**: Implements [`Error`] if the wrapped type implements both [`Display`] and [`Error`]
pub struct DisplayForDebug<T: Display>(pub T);

impl<T: Display> Debug for DisplayForDebug<T> {
    /// Formats the owned value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for DisplayForDebug<T> {
    /// Formats the owned value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display + Error> Error for DisplayForDebug<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
