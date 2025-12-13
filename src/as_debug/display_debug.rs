use std::fmt::Display;

use crate::as_debug::{AsDisplayDebug, DisplayDebugged};

/// A trait to convert a type to use its [`Display`] implementation for [`Debug`].
pub trait DisplayDebug: Display {
    /// Wraps a borrowed value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// See [`AsDisplayDebug`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_debug::DisplayDebug;
    /// use std::net::IpAddr;
    ///
    /// let ip = IpAddr::V4("127.0.0.1".parse().unwrap());
    /// assert_eq!(format!("{:?}", ip.as_debug()), "127.0.0.1");
    ///
    /// // borrowed value's display implementation is still used for display
    /// assert_eq!(format!("{}", ip.as_debug()), "127.0.0.1");
    /// ```
    fn as_debug(&'_ self) -> AsDisplayDebug<'_, Self> {
        AsDisplayDebug(self)
    }

    /// Wraps a owned value in an adaptor that enable the values [`Display`] implementation to be
    /// used for its [`Debug`] implementation.
    ///
    /// Unless ownership is not necessary, favor [`DisplayDebug::as_debug`].
    /// See [`DisplayDebugged`] for more information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::as_debug::DisplayDebug;
    /// use std::net::IpAddr;
    ///
    /// let ip = IpAddr::V4("127.0.0.1".parse().unwrap());
    /// assert_eq!(format!("{:?}", ip.to_debug()), "127.0.0.1");
    ///
    /// // owned value's display implementation is still used for display
    /// assert_eq!(format!("{}", ip.to_debug()), "127.0.0.1");
    /// ```
    fn to_debug(self) -> DisplayDebugged<Self>
    where
        Self: Sized,
    {
        DisplayDebugged(self)
    }
}

/// Implements [`DisplayDebug`] for any type that implements [`Display`].
impl<T: Display> DisplayDebug for T {}
