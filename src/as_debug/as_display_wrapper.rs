use std::fmt::{Debug, Display, Formatter};

#[cfg(doc)]
use std::error::Error;

/// A borrowed type adaptor that enables a type's [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// This is the borrowed variant that wraps a reference to a value, making it ideal for temporary
/// use in debug contexts without allocating intermediate strings or taking ownership.
///
/// # Examples
///
/// When building debug output for structs, you can use [`AsDisplayWrapper`] to incorporate
/// [`Display`]-implementing fields without extra ceremony or unnecessary string allocations.
/// The same logic can be applied to any other situation where you need to use a [`Display`]
/// implementation for a [`Debug`] implementation. For example, logging.
///
/// ```rust
/// use display_as_debug::as_debug::DisplayAsDebug;
///
/// struct UserId(u32);
///
/// impl std::fmt::Display for UserId {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "user_{}", self.0)
///     }
/// }
///
/// #[derive(Debug)]
/// struct User {
///     id: UserId,
///     name: String,
/// }
///
/// impl std::fmt::Debug for UserId {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         // Use the Display representation for Debug
///         f.debug_tuple("UserId")
///             .field(&self.display_as_debug())  // No string allocation!
///             .finish()
///     }
/// }
///
/// let user_id = UserId(42);
/// assert_eq!(format!("{:?}", user_id), "UserId(user_42)");
/// ```
///
/// # Trait Implementations
///
/// - **[`Debug`]**: Uses the borrowed value's [`Display`] implementation
/// - **[`Display`]**: Forwards to the borrowed value's [`Display`] implementation
/// - **[`Error`]**: Implements [`Error`] if the borrowed type implements both [`Display`] and [`Error`]
///
/// # See Also
///
/// - [`DisplayWrapper`](crate::as_debug::DisplayWrapper) - The owned variant for when you need to take ownership
/// - [`DisplayAsDebug`](crate::as_debug::DisplayAsDebug) - The trait providing the [`display_as_debug()`](crate::as_debug::DisplayAsDebug::display_as_debug)
///   convenience method
pub struct AsDisplayWrapper<'a, T: Display + ?Sized>(pub &'a T);

impl<T: Display> Debug for AsDisplayWrapper<'_, T> {
    /// Formats the borrowed value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for AsDisplayWrapper<'_, T> {
    /// Formats the borrowed value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display + std::error::Error> std::error::Error for AsDisplayWrapper<'_, T> {
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
    fn as_display_wrapper_debug() {
        assert_eq!(format!("{:?}", AsDisplayWrapper(&TestType)), "display");
    }

    #[test]
    fn as_display_wrapper_display() {
        assert_eq!(format!("{}", AsDisplayWrapper(&TestType)), "display");
    }
}
