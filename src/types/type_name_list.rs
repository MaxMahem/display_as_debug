use core::fmt::{Debug, Display, Formatter, Result};
use core::marker::PhantomData;

use super::type_name::{DisplayMode, TypeName};

/// A type that formats as `[<Type>: N]` when used with [`Debug`].
///
/// Useful for summarizing large collections or hiding sensitive details, by only showing their
/// element type and length.
///
/// # Example
///
/// ```
/// # use display_as_debug::types::{TypeNameList, Short};
/// assert_eq!(format!("{:?}", TypeNameList::<u8, Short>::new(100)), "[<u8>: 100]");
/// assert_eq!(format!("{}", TypeNameList::<u8, Short>::new(100)), "[<u8>: 100]");
/// ```
pub struct TypeNameList<T, M>(usize, PhantomData<(T, M)>);

impl<T, M> TypeNameList<T, M> {
    /// Creates a new `TypeNameList` with the given count.
    #[must_use]
    pub const fn new(len: usize) -> Self {
        Self(len, PhantomData)
    }

    /// Creates a new `TypeNameList` from an iterator with an exact size.
    ///
    /// # Example
    ///
    /// ```
    /// # use display_as_debug::types::{TypeNameList, Short};
    /// let items = vec![1u8, 2, 3, 4, 5];
    /// assert_eq!(format!("{:?}", TypeNameList::<u8, Short>::of(&items)), "[<u8>: 5]");
    /// ```
    #[must_use]
    pub fn of<I: IntoIterator<IntoIter: ExactSizeIterator>>(iter: I) -> Self {
        Self(iter.into_iter().len(), PhantomData)
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
