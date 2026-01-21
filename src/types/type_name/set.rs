//! Implementation of [`TypeNameSet`]

use core::fmt::{Debug, Formatter, Result};
use core::marker::PhantomData;

use crate::types::{DisplayMode, TypeName};

/// A type that formats as `{<Type>: N}` when used with [`Debug`].
///
/// This type holds no data, and is only used for formatting. It can be used to summarize large
/// sets or maps, or hide sensitive details, by only showing their element type and length.
///
/// Also available as the [`TypeNameMap`] alias.
///
/// # Example
///
/// ```rust
/// # use display_as_debug::types::{TypeNameSet, Short, Full};
/// let short = TypeNameSet::<u8, Short>::new(100);
/// assert_eq!(format!("{:?}", short), "{<u8>: 100}");
///
/// let full = TypeNameSet::<Vec<u8>, Full>::new(100);
/// assert_eq!(format!("{:?}", full), "{<alloc::vec::Vec<u8>>: 100}");
/// ```
pub struct TypeNameSet<T, M>(usize, PhantomData<(T, M)>);

/// Type alias for [`TypeNameSet`], useful when representing a map.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::types::{TypeNameMap, Short, Full};
/// # use std::collections::HashMap;
/// let map: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
///
/// let short_map = TypeNameMap::<(&str, i32), Short>::of(&map);
/// assert_eq!(format!("{:?}", short_map), "{<(&str, i32)>: 2}");
///
/// let full_map = TypeNameMap::<(&str, i32), Full>::of(&map);
/// assert_eq!(format!("{:?}", full_map), "{<(&str, i32)>: 2}");
/// ```
pub type TypeNameMap<T, M> = TypeNameSet<T, M>;

impl<T, M> TypeNameSet<T, M> {
    /// Creates a new [`TypeNameSet`] with the given `count`.
    ///
    /// ```rust
    /// # use display_as_debug::types::{TypeNameSet, Short, Full};
    /// let short = TypeNameSet::<u8, Short>::new(100);
    /// assert_eq!(format!("{:?}", short), "{<u8>: 100}");
    ///
    /// let full = TypeNameSet::<Vec<u8>, Full>::new(100);
    /// assert_eq!(format!("{:?}", full), "{<alloc::vec::Vec<u8>>: 100}");
    /// ```
    #[must_use]
    pub const fn new(count: usize) -> Self {
        Self(count, PhantomData)
    }

    /// Creates a new [`TypeNameSet`] from an iterator with an exact size.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use display_as_debug::types::{TypeNameSet, Short, Full};
    /// # use std::collections::HashSet;
    /// let items: HashSet<u8> = [1, 2, 3, 4, 5].into_iter().collect();
    ///
    /// let short = TypeNameSet::<u8, Short>::of(&items);
    /// assert_eq!(format!("{:?}", short), "{<u8>: 5}");
    ///
    /// let full = TypeNameSet::<Vec<u8>, Full>::of(&items);
    /// assert_eq!(format!("{:?}", full), "{<alloc::vec::Vec<u8>>: 5}");
    /// ```
    #[must_use]
    pub fn of<I: IntoIterator<IntoIter: ExactSizeIterator>>(iter: I) -> Self {
        Self(iter.into_iter().len(), PhantomData)
    }

    /// Returns the descriptive length of the set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::{TypeNameSet, Short};
    /// let set = TypeNameSet::<u8, Short>::new(100);
    /// assert_eq!(set.len(), 100, "Length should match constructed length")
    /// ```
    #[allow(clippy::len_without_is_empty, reason = "This type holds no values")]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.0
    }
}

impl<T, M: DisplayMode> Debug for TypeNameSet<T, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{<{:?}>: {}}}", TypeName::empty::<T, M>(), self.0)
    }
}

impl<I: IntoIterator<IntoIter: ExactSizeIterator>, T, M: DisplayMode> From<I> for TypeNameSet<T, M> {
    fn from(iter: I) -> Self {
        Self(iter.into_iter().len(), PhantomData)
    }
}
