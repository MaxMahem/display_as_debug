use super::{AsDebugWrapper, DebugWrapper};
use std::fmt::Debug;

/// A trait to convert a type to a [`DebugAsDisplay`] or [`DebugWrapper`].
pub trait DebugAsDisplay: Debug {
    /// Wraps a borrowed value in an adaptor that enable the values [`Debug`] implementation to be
    /// used for its [`Display`] implementation.
    ///
    /// See [`AsDebugWrapper`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_display::DebugAsDisplay;
    ///
    /// let numbers = vec![1, 2, 3];
    /// // debug implementation is used for display
    /// assert_eq!(format!("{}", numbers.debug_as_display()), "[1, 2, 3]");
    ///
    /// // debug implementation is still used for debug
    /// assert_eq!(format!("{:?}", numbers.debug_as_display()), "[1, 2, 3]");
    /// ```
    fn debug_as_display(&'_ self) -> AsDebugWrapper<'_, Self> {
        AsDebugWrapper(self)
    }

    /// Wraps a owned value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// If ownership is not necessary, favor [`DebugAsDisplay::debug_as_display`].
    /// See [`DebugWrapper`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_display::DebugAsDisplay;
    ///
    /// let numbers = vec![1, 2, 3];
    /// assert_eq!(format!("{}", numbers.clone().debug_to_display()), "[1, 2, 3]");
    ///
    /// // borrowed value's debug implementation is still used for debug
    /// assert_eq!(format!("{:?}", numbers.clone().debug_to_display()), "[1, 2, 3]");
    /// ```
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
