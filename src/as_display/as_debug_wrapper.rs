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
/// implementation is available. Such as with most standard library types. [`AsDebugWrapper`] lets
/// you use their debug representation in [`Display`] contexts:
///
/// ```rust
/// use display_as_debug::as_display::DebugAsDisplay;
///
/// let numbers = vec![1, 2, 3];
/// let formatted = format!("Numbers: {}", numbers.debug_as_display());
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
/// - [`DebugWrapper`](crate::as_display::DebugWrapper) - The owned variant for when you need to take ownership
/// - [`DebugAsDisplay`](crate::as_display::DebugAsDisplay) - The trait providing the [`debug_as_display()`](crate::as_display::DebugAsDisplay::debug_as_display)
///   convenience method
pub struct AsDebugWrapper<'a, T: Debug + ?Sized>(pub &'a T);

impl<T: Debug> Display for AsDebugWrapper<'_, T> {
    /// Formats the borrowed value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for AsDebugWrapper<'_, T> {
    /// Formats the borrowed value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug + std::error::Error> std::error::Error for AsDebugWrapper<'_, T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestType;

    impl std::fmt::Debug for TestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "debug")
        }
    }

    #[test]
    fn as_debug_wrapper_display() {
        assert_eq!(format!("{}", AsDebugWrapper(&TestType)), "debug");
    }

    #[test]
    fn as_debug_wrapper_debug() {
        assert_eq!(format!("{:?}", AsDebugWrapper(&TestType)), "debug");
    }
}
