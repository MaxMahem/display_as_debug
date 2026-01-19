use core::error::Error;
use core::fmt::{Debug, Display, Formatter, Result};

use derive_more::{AsMut, AsRef, Deref, From};

/// A type adaptor that enables a type's [`Debug`] implementation to be used for its
/// [`Display`] implementation.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::wrap::DebugAsDisplay;
/// assert_eq!(format!("{}", DebugAsDisplay(&vec![1])), "[1]", "debug used for display");
/// assert_eq!(format!("{:?}", DebugAsDisplay(&vec![1])), "[1]", "debug unchanged");
/// ```
/// # Noteable Trait Implementations
///
/// - **[`Display`]**: Uses the wrapped value's [`Debug`] implementation
/// - **[`Debug`]**: Forwards to the wrapped value's [`Debug`] implementation
/// - **[`Error`]**: Implements [`Error`] if the wrapped type implements both [`Debug`] and [`Error`]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Deref, AsRef, AsMut)]
pub struct DebugAsDisplay<T>(pub T);

impl<T: Debug> Display for DebugAsDisplay<T> {
    /// Formats the value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for DebugAsDisplay<T> {
    /// Formats the value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug + Error> Error for DebugAsDisplay<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
