use core::fmt::Display;

use crate::as_debug::{DisplayAsDebug, DisplayDebugged};

/// A trait to convert a type to use its [`Display`] implementation for [`Debug`].
pub trait DisplayDebug: Display {
    /// Wraps a borrowed value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_debug::DisplayDebug;
    /// use std::net::IpAddr;
    ///
    /// let ip = IpAddr::V4("127.0.0.1".parse().unwrap());
    /// assert_eq!(format!("{:?}", ip.display_as_debug()), "127.0.0.1", "display used for debug");
    /// assert_eq!(format!("{}", ip.display_as_debug()), "127.0.0.1", "display unchanged");
    /// ```
    fn display_as_debug(&'_ self) -> DisplayAsDebug<'_, Self> {
        DisplayAsDebug(self)
    }

    /// Wraps a owned value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// Unless ownership is not necessary, favor [`DisplayDebug::display_as_debug`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_debug::DisplayDebug;
    /// use std::net::IpAddr;
    ///
    /// let ip = IpAddr::V4("127.0.0.1".parse().unwrap());
    /// assert_eq!(format!("{:?}", ip.wrap_display_as_debug()), "127.0.0.1", "display used for debug");
    /// assert_eq!(format!("{}", ip.wrap_display_as_debug()), "127.0.0.1", "display unchanged");
    /// ```
    fn wrap_display_as_debug(self) -> DisplayDebugged<Self>
    where
        Self: Sized,
    {
        DisplayDebugged(self)
    }
}

/// Implements [`DisplayDebug`] for any type that implements [`Display`].
impl<T: Display> DisplayDebug for T {}
