use std::fmt::{Debug, Display, Formatter};

/// An owning type adaptor that enables a type's [`Debug`] implementation to be used for its
/// [`Display`] implementation.
///
/// This is the owned variant of [`AsDebugWrapper`](crate::as_display::AsDebugWrapper). Unlike
/// the borrowed wrapper, this type takes ownership of the value and is useful when you need to
/// return a wrapped value or store it for later use.
///
/// # Examples
///
/// Some types only implement `Debug` but you need to use them in a context that requires `Display`.
/// `DebugWrapper` allows you to use their `Debug` representation as their `Display`:
///
/// ```rust
/// use display_as_debug::as_display::{DebugWrapper, DebugAsDisplay};
///
/// #[derive(Debug)]
/// struct DebugOnlyType {
///     field: i32,
/// }
///
/// let value = DebugOnlyType { field: 42 };
///
/// // Using debug_to_display() method from DebugAsDisplay trait
/// let wrapped = value.debug_to_display();
/// assert_eq!(wrapped.to_string(), "DebugOnlyType { field: 42 }");
///
/// // Or construct DebugWrapper directly
/// let value2 = DebugOnlyType { field: 99 };
/// let wrapped2 = DebugWrapper(value2);
/// assert_eq!(format!("{}", wrapped2), "DebugOnlyType { field: 99 }");
/// ```
///
/// # Trait Implementations
///
/// - **[`Display`]**: Uses the wrapped value's [`Debug`] implementation
/// - **[`Debug`]**: Forwards to the wrapped value's [`Debug`] implementation
/// - **[`Error`]**: Implements [`Error`] if the wrapped type implements both [`Debug`] and [`Error`]
///
/// # See Also
///
/// - [`AsDebugWrapper`](crate::as_display::AsDebugWrapper) - The borrowed variant
/// - [`DebugAsDisplay`](crate::as_display::DebugAsDisplay) - The trait providing convenience methods
///   [`debug_to_display()`](crate::as_display::DebugAsDisplay::debug_to_display).
pub struct DebugWrapper<T: Debug>(pub T);

impl<T: Debug> Display for DebugWrapper<T> {
    /// Formats the owned value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for DebugWrapper<T> {
    /// Formats the owned value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: Debug + std::error::Error> std::error::Error for DebugWrapper<T> {
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
    fn debug_wrapper_display() {
        assert_eq!(format!("{}", DebugWrapper(TestType)), "debug");
    }

    #[test]
    fn debug_wrapper_debug() {
        assert_eq!(format!("{:?}", DebugWrapper(TestType)), "debug");
    }

    #[test]
    fn debug_wrapper_error_source() {
        use std::error::Error;
        use std::io;

        let error = io::Error::other("test error");
        let wrapped = DebugWrapper(error);
        let _ = wrapped.source();
    }
}
