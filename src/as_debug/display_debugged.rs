use core::error::Error;
use core::fmt::{Debug, Display, Formatter, Result};

/// An owning type adaptor that enables a type's [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// This is the owned variant of [`AsDisplayDebug`](crate::as_debug::AsDisplayDebug). Unlike
/// the borrowed wrapper, this type takes ownership of the value and is particularly useful when
/// you need to return a wrapped value or store it for later use.
///
/// # Use Cases
///
/// ## Returning Errors from `main()`
///
/// When `main()` returns a [`Result<(), E>`](core::result::Result), Rust prints the error using its
/// [`Debug`] implementation. By wrapping error types with [`DisplayDebugged`], you can ensure the
/// user-friendly [`Display`] representation is shown instead of the more technical [`Debug`]
/// output.
///
/// ```no_run
/// use display_as_debug::as_debug::{DisplayDebugged, DisplayDebug};
///
/// #[derive(Debug)]
/// struct AppError {
///     message: String,
///     code: i32,
/// }
///
/// impl std::fmt::Display for AppError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Error {}: {}", self.code, self.message)
///     }
/// }
///
/// impl std::error::Error for AppError {}
///
/// fn risky_operation() -> Result<(), AppError> {
///     Err(AppError {
///         message: "database connection failed".to_string(),
///         code: 500,
///     })
/// }
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Using to_debug() method from DisplayDebug trait
///     risky_operation().map_err(DisplayDebugged)?;
///     
///     Ok(())
/// }
/// ```
///
/// # Trait Implementations
///
/// - **[`Debug`]**: Uses the wrapped value's [`Display`] implementation
/// - **[`Display`]**: Forwards to the wrapped value's [`Display`] implementation  
/// - **[`Error`]**: Implements [`Error`] if the wrapped type implements both [`Display`] and [`Error`]
///
/// # See Also
///
/// - [`AsDisplayDebug`](crate::as_debug::AsDisplayDebug) - The borrowed variant
/// - [`DisplayDebug`](crate::as_debug::DisplayDebug) - The trait providing convenience methods
///   [`to_debug()`](crate::as_debug::DisplayDebug::to_debug).
#[derive(PartialEq, Eq)]
pub struct DisplayDebugged<T: Display>(pub T);

impl<T: Display> Debug for DisplayDebugged<T> {
    /// Formats the owned value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for DisplayDebugged<T> {
    /// Formats the owned value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display + Error> Error for DisplayDebugged<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
