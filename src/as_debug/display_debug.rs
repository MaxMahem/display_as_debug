use std::fmt::Display;

use crate::as_debug::{AsDisplayWrapper, DisplayWrapper};

/// A trait to convert a type to a [`DisplayAsDebug`].
pub trait DisplayAsDebug: Display {
    /// Wraps a borrowed value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// See [`AsDisplayWrapper`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_debug::DisplayAsDebug;
    /// use std::net::IpAddr;
    ///
    /// let ip = IpAddr::V4("127.0.0.1".parse().unwrap());
    /// assert_eq!(format!("{:?}", ip.display_as_debug()), "127.0.0.1");
    ///
    /// // borrowed value's display implementation is still used for display
    /// assert_eq!(format!("{}", ip.display_as_debug()), "127.0.0.1");
    /// ```
    fn display_as_debug(&'_ self) -> AsDisplayWrapper<'_, Self> {
        AsDisplayWrapper(self)
    }

    /// Wraps a owned value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// Unless ownership is not necessary, favor [`DisplayAsDebug::display_as_debug`].
    /// See [`DisplayWrapper`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_debug::DisplayAsDebug;
    /// use std::net::IpAddr;
    ///
    /// let ip = IpAddr::V4("127.0.0.1".parse().unwrap());
    /// assert_eq!(format!("{:?}", ip.display_to_debug()), "127.0.0.1");
    ///
    /// // owned value's display implementation is still used for display
    /// assert_eq!(format!("{}", ip.display_to_debug()), "127.0.0.1");
    /// ```
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
