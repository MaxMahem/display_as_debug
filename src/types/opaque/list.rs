use core::fmt::{Debug, Display, Formatter, Result};

use super::OPAQUE;

/// A type that formats as `[..: N]` when used with [`Debug`].
///
/// Useful for summarizing large collections by showing only their length, or for hiding sensitive
/// details.
///
/// # Example
///
/// ```
/// # use display_as_debug::types::OpaqueList;
/// assert_eq!(format!("{:?}", OpaqueList(100)), "[..: 100]");
/// assert_eq!(format!("{}", OpaqueList(100)), "[..: 100]");
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OpaqueList(pub usize);

impl OpaqueList {
    /// Creates a new [`OpaqueList`] from an iterator with an exact size.
    ///
    /// # Example
    ///
    /// ```
    /// # use display_as_debug::types::OpaqueList;
    /// let items = vec![1, 2, 3, 4, 5];
    /// assert_eq!(format!("{:?}", OpaqueList::of(&items)), "[..: 5]");
    /// ```
    #[must_use]
    pub fn of<I: IntoIterator<IntoIter: ExactSizeIterator>>(iter: I) -> Self {
        Self(iter.into_iter().len())
    }
}

impl Debug for OpaqueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}: {}]", OPAQUE, self.0)
    }
}

impl Display for OpaqueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

impl<I: IntoIterator<IntoIter: ExactSizeIterator>> From<I> for OpaqueList {
    fn from(iter: I) -> Self {
        Self(iter.into_iter().len())
    }
}
