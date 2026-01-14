use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use crate::type_name::{DisplayMode, Full, TypeName};

/// A [`Result<T, E>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"Ok(typename)"` when the result is [`Ok`], for [`Err`] the [`Debug`]
/// implementation of `E` is used.
///
/// The `M` type parameter controls whether [`Full`](crate::type_name::Full) or [`Short`](crate::type_name::Short)
/// type names are displayed.
#[derive(Copy, Clone)]
pub struct ResultTypeDebug<'a, T, E, M: DisplayMode = Full> {
    inner: &'a Result<T, E>,
    _marker: PhantomData<M>,
}

impl<'a, T, E> ResultTypeDebug<'a, T, E> {
    /// Create a new `ResultTypeDebug` wrapper.
    pub const fn new<M: DisplayMode>(result: &'a Result<T, E>) -> ResultTypeDebug<'a, T, E, M> {
        ResultTypeDebug { inner: result, _marker: PhantomData }
    }
}

impl<T, E: Debug, M: DisplayMode> Debug for ResultTypeDebug<'_, T, E, M>
where
    TypeName<T, M>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.inner {
            Ok(_) => f.debug_tuple("Ok").field(&TypeName::<T, M>::default()).finish(),
            Err(e) => f.debug_tuple("Err").field(e).finish(),
        }
    }
}
