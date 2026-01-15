use core::fmt::{Debug, Display, Formatter};

/// A marker type that formats as `..` when used in `Debug` contexts.
///
/// This is useful for hiding sensitive or verbose data in debug output while
/// still indicating that a value exists.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::types::Opaque;
/// use std::fmt::{Debug, Formatter};
///
/// struct Credentials {
///     password: String,
/// }
///
/// impl Debug for Credentials {
///     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
///         f.debug_struct("Credentials")
///             .field("password", &Opaque)
///             .finish()
///     }
/// }
///
/// let creds = Credentials { password: "secret".into() };
///
/// assert_eq!(format!("{:?}", creds), "Credentials { password: .. }");
/// ```
#[derive(Copy, Clone)]
pub struct Opaque;

/// The string reprsentation of [`Opaque`]
const OPAQUE: &str = "..";

impl Debug for Opaque {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(OPAQUE)
    }
}

impl Display for Opaque {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}
