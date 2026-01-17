use core::fmt::{Debug, Display, Formatter};

use derive_more::{AsMut, AsRef, Deref, From};

/// A wrapper type that formats as `..` when used in `Debug` contexts,
/// obscuring the inner value.
///
/// This is useful for hiding sensitive or verbose data in debug output while
/// still indicating that a value exists.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::types::{OPAQUE, Opaque};
/// assert_eq!(format!("{:?}", OPAQUE), "..", "Debug format should be opaque");
/// assert_eq!(format!("{}", OPAQUE), "..", "Display format should be opaque");
///
/// assert_eq!(format!("{:?}", Opaque("secret")), "..", "Debug format should be opaque");
/// assert_eq!(format!("{}", Opaque("secret")), "..", "Display format should be opaque");
/// ```
#[derive(Copy, Clone, Deref, From, AsMut, AsRef, Default)]
pub struct Opaque<T = ()>(pub T);

/// An obscure marker value that formats as `..` when used in [`Debug`] or [`Display`].
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::types::{OPAQUE, Opaque};
/// assert_eq!(format!("{:?}", OPAQUE), "..", "Debug format should be opaque");
/// assert_eq!(format!("{}", OPAQUE), "..", "Display format should be opaque");
/// ```
pub const OPAQUE: Opaque<()> = Opaque::DEFAULT;

impl Opaque<()> {
    /// The default value for [`Opaque`]
    pub const DEFAULT: Self = Self(());
}

impl<T> Opaque<T> {
    /// The string representation of [`Opaque::DEFAULT`]/[`OPAQUE`]
    pub const OPAQUE_STR: &str = "..";
}

impl<T> Debug for Opaque<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(Self::OPAQUE_STR)
    }
}

impl<T> Display for Opaque<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}
