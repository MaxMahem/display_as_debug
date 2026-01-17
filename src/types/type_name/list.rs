use core::fmt::{Debug, Display, Formatter, Result};
use core::marker::PhantomData;

use super::{DisplayMode, TypeName};

/// A type that formats as `[<Type>: N]` when used with [`Debug`].
///
/// Useful for summarizing large collections or hiding sensitive details, by only showing their
/// element type and length.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::types::{TypeNameList, Short, Full};
///
/// let short = TypeNameList::<u8, Short>::new(100);
/// assert_eq!(format!("{:?}", short), "[<u8>: 100]");
/// assert_eq!(format!("{}", short), "[<u8>: 100]");
///
/// let full = TypeNameList::<Vec<u8>, Full>::new(100);
/// assert_eq!(format!("{:?}", full), "[<alloc::vec::Vec<u8>>: 100]");
/// assert_eq!(format!("{}", full), "[<alloc::vec::Vec<u8>>: 100]");
/// ```
pub struct TypeNameList<T, M>(usize, PhantomData<(T, M)>);

impl<T, M> TypeNameList<T, M> {
    /// Creates a new [`TypeNameList`] with the given `count`.
    #[must_use]
    pub const fn new(count: usize) -> Self {
        Self(count, PhantomData)
    }

    /// Creates a new [`TypeNameList`] from an iterator with an exact size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::{TypeNameList, Short};
    /// let items = vec![1u8, 2, 3, 4, 5];
    /// assert_eq!(
    ///     format!("{:?}", TypeNameList::<u8, Short>::of(&items)),
    ///     "[<u8>: 5]"
    /// );
    /// ```
    #[must_use]
    pub fn of<I: IntoIterator<IntoIter: ExactSizeIterator>>(iter: I) -> Self {
        Self(iter.into_iter().len(), PhantomData)
    }

    /// Returns the descriptive length of the list.
    #[must_use]
    #[allow(clippy::len_without_is_empty, reason = "This type holds no values")]
    pub const fn len(&self) -> usize {
        self.0
    }
}

impl<T, M: DisplayMode> Debug for TypeNameList<T, M>
where
    TypeName<T, M>: Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[<{}>: {}]", TypeName::<T, M>::default(), self.0)
    }
}

impl<T, M: DisplayMode> Display for TypeNameList<T, M>
where
    TypeName<T, M>: Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(self, f)
    }
}

impl<I, T, M: DisplayMode> From<I> for TypeNameList<T, M>
where
    I: IntoIterator<IntoIter: ExactSizeIterator>,
{
    fn from(iter: I) -> Self {
        Self(iter.into_iter().len(), PhantomData)
    }
}
