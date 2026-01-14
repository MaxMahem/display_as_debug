use core::error::Error;
use core::fmt::{Debug, Display, Formatter, Result};

/// An owning type adaptor that enables a type's [`Debug`] implementation to be used for its
/// [`Display`] implementation.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::as_display::DebugDisplayed;
/// assert_eq!(format!("{}", DebugDisplayed(vec![1])), "[1]", "debug used for display");
/// assert_eq!(format!("{:?}", DebugDisplayed(vec![2])), "[2]", "debug unchanged");
/// ```
///
/// # Trait Implementations
///
/// - **[`Display`]**: Uses the wrapped value's [`Debug`] implementation
/// - **[`Debug`]**: Forwards to the wrapped value's [`Debug`] implementation
/// - **[`Error`]**: Implements [`Error`] if the wrapped type implements both [`Debug`] and [`Error`]
#[derive(PartialEq, Eq)]
pub struct DebugDisplayed<T: Debug>(pub T);

impl<T: Debug> Display for DebugDisplayed<T> {
    /// Formats the owned value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for DebugDisplayed<T> {
    /// Formats the owned value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: Debug + Error> Error for DebugDisplayed<T> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
