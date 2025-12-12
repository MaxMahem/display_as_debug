use std::fmt::{Debug, Display, Formatter};

/// A type adaptor that enable a borrowed types [`Debug`] implementation to be used for its
/// [`Display`] implementation.
///
/// For more information see [`DebugAsDisplay`].
///
/// For an owned implementation see [`DebugWrapper`].
pub struct AsDebugWrapper<'a, T: Debug + ?Sized>(&'a T);

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

/// A owning type adaptor, similar to [`AsDebugWrapper`].
pub struct DebugWrapper<T: Debug>(T);

impl<T: Debug> Display for DebugWrapper<T> {
    /// Formats the owned value using its display implementation.
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

/// A trait to convert a type to a [`DebugAsDisplay`].
pub trait DebugAsDisplay: Debug {
    /// Wraps a borrowed value in an adaptor that enable the values [`Debug`] implementation to be
    /// used for its [`Display`] implementation.
    ///
    /// This can be useful for using the [`Debug`] implementation in a [`Display`] formatter.
    ///
    /// The borrowed value's [`Debug`] implementation is still used for the wrapped type's
    /// [`Debug`] implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::DebugAsDisplay;
    ///
    /// struct TestType;
    ///
    /// impl std::fmt::Debug for TestType {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "debug")
    ///     }
    /// }
    ///
    /// let test_type = TestType;
    /// assert_eq!(format!("{}", test_type.debug_as_display()), "debug");
    ///
    /// // borrowed value's debug implementation is still used for debug
    /// assert_eq!(format!("{:?}", test_type.debug_as_display()), "debug");
    /// ```
    fn debug_as_display(&'_ self) -> AsDebugWrapper<'_, Self> {
        AsDebugWrapper(self)
    }

    /// Wraps a owned value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// See [`DebugAsDisplay::debug_as_display`] for more information.
    fn debug_to_display(self) -> DebugWrapper<Self>
    where
        Self: Sized,
    {
        DebugWrapper(self)
    }
}

/// Implements [`DebugAsDisplay`] for any type that implements [`Debug`].
impl<T: Debug> DebugAsDisplay for T {}

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
    fn debug_as_display() {
        assert_eq!(format!("{:?}", TestType.debug_as_display()), "debug");
    }

    #[test]
    fn debug_as_debug() {
        assert_eq!(format!("{}", TestType.debug_as_display()), "debug");
    }

    #[test]
    fn debug_to_display() {
        assert_eq!(format!("{:?}", TestType.debug_to_display()), "debug");
    }

    #[test]
    fn debug_to_debug() {
        assert_eq!(format!("{}", TestType.debug_to_display()), "debug");
    }
}
