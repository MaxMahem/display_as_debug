use std::fmt::{Debug, Display, Formatter};

/// A type adaptor that enable a borrowed types [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// This can be useful for using the [`Display`] implementation of information type like a unit
/// type in a debug context, without having to allocate an intemediate string.
///
/// The borrowed types [`Display`] implementation is still used for the wrapped type's [`Display`]
/// implementation.
///
/// For an owned implementation see [`DisplayWrapper`].
///
/// # Example
///
/// ```rust
/// use display_as_debug::DisplayAsDebug;
///
/// struct TestType;
///
/// impl std::fmt::Display for TestType {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "display")
///     }
/// }
///
/// let test_type = TestType;
/// assert_eq!(test_type.display_as_debug().to_string(), "display");
/// assert_eq!(format!("{:?}", test_type.display_as_debug()), "display");
/// ```
pub struct AsDisplayWrapper<'a, T: Display + ?Sized>(pub &'a T);

impl<'a, T: Display> Debug for AsDisplayWrapper<'a, T> {
    /// Formats the borrowed value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<'a, T: Display> Display for AsDisplayWrapper<'a, T> {
    /// Formats the borrowed value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<'a, T: Display + std::error::Error> std::error::Error for AsDisplayWrapper<'a, T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

/// A owning type adaptor, similar to [`AsDisplayWrapper`].
pub struct DisplayWrapper<T: Display>(T);

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

/// A trait to convert a type to a [`DisplayAsDebug`].
pub trait DisplayAsDebug: Display {
    /// Wraps a borrowed value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// This can be useful for using the [`Display`] implementation of information type like a unit
    /// type in a debug formatter, without having to allocate an intemediate string.
    ///
    /// The borrowed value's [`Display`] implementation is still used for the wrapped type's
    /// [`Display`] implementation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::DisplayAsDebug;
    ///
    /// struct TestType;
    ///
    /// impl std::fmt::Display for TestType {
    ///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    ///         write!(f, "display")
    ///     }
    /// }
    ///
    /// let test_type = TestType;
    /// assert_eq!(format!("{:?}", test_type.display_as_debug()), "display");
    ///
    /// // borrowed value's display implementation is still used for display
    /// assert_eq!(format!("{}", TestType.display_as_debug()), "display");
    /// ```
    fn display_as_debug(&'_ self) -> AsDisplayWrapper<'_, Self> {
        AsDisplayWrapper(self)
    }

    /// Wraps a owned value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// See [`DisplayAsDebug::display_as_debug`] for more information.
    fn display_to_debug(self) -> DisplayWrapper<Self>
    where
        Self: Sized,
    {
        DisplayWrapper(self)
    }
}

/// Implements [`DisplayAsDebug`] for any type that implements [`Display`].
impl<T: Display> DisplayAsDebug for T {}

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
    fn display_as_debug() {
        assert_eq!(format!("{:?}", TestType.display_as_debug()), "display");
    }

    #[test]
    fn display_as_display() {
        assert_eq!(format!("{}", TestType.display_as_debug()), "display");
    }

    #[test]
    fn display_to_debug() {
        assert_eq!(format!("{:?}", TestType.display_to_debug()), "display");
    }

    #[test]
    fn display_to_display() {
        assert_eq!(format!("{}", TestType.display_to_debug()), "display");
    }
}
