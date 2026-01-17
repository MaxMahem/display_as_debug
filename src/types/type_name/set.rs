use core::fmt::{Debug, Display, Formatter, Result};
use core::marker::PhantomData;

use crate::types::{DisplayMode, TypeName};

/// A type that formats as `{<Type>: N}` when used with [`Debug`].
///
/// Useful for summarizing large sets or maps, or hiding sensitive details, by only showing their
/// element type and length. Also available as the [`TypeNameMap`] alias.
///
/// # Example
///
/// ```rust
/// use display_as_debug::types::{TypeNameSet, Short, Full};
///
/// let short = TypeNameSet::<u8, Short>::new(100);
/// assert_eq!(format!("{:?}", short), "{<u8>: 100}");
/// assert_eq!(format!("{}", short), "{<u8>: 100}");
///
/// let full = TypeNameSet::<Vec<u8>, Full>::new(100);
/// assert_eq!(format!("{:?}", full), "{<alloc::vec::Vec<u8>>: 100}");
/// assert_eq!(format!("{}", full), "{<alloc::vec::Vec<u8>>: 100}");
/// ```
pub struct TypeNameSet<T, M>(usize, PhantomData<(T, M)>);

/// Type alias for [`TypeNameSet`], useful when representing a map.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::types::{TypeNameMap, Short};
/// # use std::collections::HashMap;
/// let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
/// assert_eq!(
///     format!("{:?}", TypeNameMap::<(&str, i32), Short>::of(&map)),
///     "{<(&str, i32)>: 2}"
/// );
///
/// assert_eq!(
///     format!("{:?}", TypeNameMap::<(&str, i32), Short>::new(100)),
///     "{<(&str, i32)>: 100}"
/// );
/// ```
pub type TypeNameMap<T, M> = TypeNameSet<T, M>;

impl<T, M> TypeNameSet<T, M> {
    /// Creates a new [`TypeNameSet`] with the given `count`.
    #[must_use]
    pub const fn new(count: usize) -> Self {
        Self(count, PhantomData)
    }

    /// Creates a new [`TypeNameSet`] from an iterator with an exact size.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use display_as_debug::types::{TypeNameSet, Short};
    /// # use std::collections::HashSet;
    /// let items: HashSet<u8> = [1, 2, 3, 4, 5].into_iter().collect();
    /// assert_eq!(format!("{:?}", TypeNameSet::<u8, Short>::of(&items)), "{<u8>: 5}");
    /// ```
    #[must_use]
    pub fn of<I: IntoIterator<IntoIter: ExactSizeIterator>>(iter: I) -> Self {
        Self(iter.into_iter().len(), PhantomData)
    }

    /// Returns the descriptive length of the set.
    #[allow(clippy::len_without_is_empty, reason = "This type holds no values")]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0
    }
}

impl<T, M: DisplayMode> Debug for TypeNameSet<T, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{<{}>: {}}}", TypeName::empty::<T, M>(), self.0)
    }
}

impl<T, M: DisplayMode> Display for TypeNameSet<T, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

impl<I: IntoIterator<IntoIter: ExactSizeIterator>, T, M: DisplayMode> From<I> for TypeNameSet<T, M> {
    fn from(iter: I) -> Self {
        Self(iter.into_iter().len(), PhantomData)
    }
}
