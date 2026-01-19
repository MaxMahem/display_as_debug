use core::error::Error;
use core::fmt::{Debug, Display, Formatter, Result};

use derive_more::{AsMut, AsRef, Deref, From};

/// A type adaptor that enables a type's [`Display`] implementation to be used for its
/// [`Debug`] implementation.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::wrap::DisplayAsDebug;
/// use display_as_debug::types::TestValue;
///
/// assert_eq!(format!("{:?}", DisplayAsDebug(&TestValue::TEST)), r#"Display("test")"#, "display used for debug");
/// assert_eq!(format!("{}", DisplayAsDebug(&TestValue::TEST)), r#"Display("test")"#, "display unchanged");
/// ```
///
/// # Notable Trait Implementations
///
/// - **[`Debug`]**: Uses the wrapped value's [`Display`] implementation
/// - **[`Display`]**: Forwards to the wrapped value's [`Display`] implementation
/// - **[`Error`]**: Implements [`Error`] if the wrapped type implements both [`Display`] and [`Error`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Deref, AsRef, AsMut)]
pub struct DisplayAsDebug<T>(pub T);

impl<T: Display> Debug for DisplayAsDebug<T> {
    /// Formats the value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display> Display for DisplayAsDebug<T> {
    /// Formats the value using its display implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self.0, f)
    }
}

impl<T: Display + Error> Error for DisplayAsDebug<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
