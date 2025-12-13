use std::fmt::{Debug, Display, Formatter};

#[cfg(doc)]
use std::error::Error;

/// An owning type adaptor that enables a type's [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// This is the owned variant of [`AsDisplayWrapper`](crate::as_debug::AsDisplayWrapper). Unlike
/// the borrowed wrapper, this type takes ownership of the value and is particularly useful when
/// you need to return a wrapped value or store it for later use.
///
/// # Use Cases
///
/// ## Returning Errors from `main()`
///
/// When `main()` returns a [`Result<(), E>`](std::result::Result), Rust prints the error using its
/// [`Debug`] implementation. By wrapping error types with [`DisplayWrapper`], you can ensure the
/// user-friendly [`Display`] representation is shown instead of the more technical [`Debug`]
/// output.
///
/// ```no_run
/// use display_as_debug::as_debug::{DisplayWrapper, DisplayAsDebug};
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
///     // Using display_to_debug() method from DisplayAsDebug trait
///     risky_operation().map_err(DisplayWrapper)?;
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
/// - [`AsDisplayWrapper`](crate::as_debug::AsDisplayWrapper) - The borrowed variant
/// - [`DisplayAsDebug`](crate::as_debug::DisplayAsDebug) - The trait providing convenience methods
///   [`display_to_debug()`](crate::as_debug::DisplayAsDebug::display_to_debug).
pub struct DisplayWrapper<T: Display>(pub T);

impl<T: Display> Debug for DisplayWrapper<T> {
    /// Formats the owned value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for DisplayWrapper<T> {
    /// Formats the owned value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display + std::error::Error> std::error::Error for DisplayWrapper<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestType;

    impl std::fmt::Display for TestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "display")
        }
    }

    #[test]
    fn display_wrapper_debug() {
        assert_eq!(format!("{:?}", DisplayWrapper(TestType)), "display");
    }

    #[test]
    fn display_wrapper_display() {
        assert_eq!(format!("{}", DisplayWrapper(TestType)), "display");
    }

    #[test]
    fn display_wrapper_error_source() {
        use std::error::Error;
        use std::io;

        let error = io::Error::other("test error");
        let wrapped = DisplayWrapper(error);
        let _ = wrapped.source();
    }
}
