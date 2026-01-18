use core::fmt::{Debug, Display, Formatter, Result};

use crate::types::OPAQUE;

/// A type that formats as `{..: N}` when used with [`Debug`].
///
/// Useful for summarizing large sets or maps by showing only their length, or for hiding sensitive
/// details. Also available as the [`OpaqueMap`] alias.
///
/// # Example
///
/// ```
/// # use display_as_debug::types::OpaqueSet;
/// assert_eq!(format!("{:?}", OpaqueSet(100)), "{..: 100}");
/// assert_eq!(format!("{}", OpaqueSet(100)), "{..: 100}");
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OpaqueSet(pub usize);

/// Type alias for [`OpaqueSet`], useful when representing a map.
///
/// # Example
///
/// ```
/// # use display_as_debug::types::OpaqueMap;
/// # use std::collections::HashMap;
/// let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
/// assert_eq!(format!("{:?}", OpaqueMap::of(&map)), "{..: 2}");
/// ```
pub type OpaqueMap = OpaqueSet;

impl OpaqueSet {
    /// Creates a new [`OpaqueSet`] from an iterator with an exact size.
    ///
    /// # Example
    ///
    /// ```
    /// # use display_as_debug::types::OpaqueSet;
    /// # use std::collections::HashSet;
    /// let items: HashSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    /// assert_eq!(format!("{:?}", OpaqueSet::of(&items)), "{..: 5}");
    /// ```
    #[must_use]
    pub fn of<I: IntoIterator<IntoIter: ExactSizeIterator>>(iter: I) -> Self {
        Self(iter.into_iter().len())
    }
}

impl Debug for OpaqueSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{{}: {}}}", OPAQUE, self.0)
    }
}

impl Display for OpaqueSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

impl<I: IntoIterator<IntoIter: ExactSizeIterator>> From<I> for OpaqueSet {
    fn from(iter: I) -> Self {
        Self(iter.into_iter().len())
    }
}
