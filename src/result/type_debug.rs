use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use derive_more::{AsMut, AsRef, Deref};

use crate::result::{STR_ERR, STR_OK};
use crate::types::{DisplayMode, Full, TypeName};

/// A [`Result<T, E>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"Ok(typename)"` when the result is [`Ok`], for [`Err`] the [`Debug`]
/// implementation of `E` is used.
///
/// The `M` type parameter controls whether [`Full`](crate::types::Full) or [`Short`](crate::types::Short)
/// type names are displayed.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, AsMut)]
pub struct TypeNameResult<'a, T, E, M: DisplayMode = Full> {
    #[deref]
    #[as_ref]
    #[as_mut]
    inner: &'a Result<T, E>,
    _marker: PhantomData<M>,
}

impl<'a, T, E> TypeNameResult<'a, T, E> {
    /// Create a new `TypeNameResult` wrapper.
    pub const fn new<M: DisplayMode>(result: &'a Result<T, E>) -> TypeNameResult<'a, T, E, M> {
        TypeNameResult { inner: result, _marker: PhantomData }
    }
}

impl<'a, T, E, M: DisplayMode> From<&'a Result<T, E>> for TypeNameResult<'a, T, E, M> {
    fn from(result: &'a Result<T, E>) -> Self {
        Self { inner: result, _marker: PhantomData }
    }
}

impl<T, E: Debug, M: DisplayMode> Debug for TypeNameResult<'_, T, E, M>
where
    TypeName<T, M>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.inner {
            Ok(_) => f.debug_tuple(STR_OK).field(&TypeName::<T, M>::default()).finish(),
            Err(e) => f.debug_tuple(STR_ERR).field(e).finish(),
        }
    }
}
