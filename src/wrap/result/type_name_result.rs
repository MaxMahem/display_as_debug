use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use derive_more::{AsMut, AsRef, Deref};

use crate::types::{DisplayMode, Full, TypeName};
use crate::wrap::result::{STR_ERR, STR_OK};

/// A [`Result<T, E>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"Ok(typename)"` when the result is [`Ok`], for [`Err`] the [`Debug`]
/// implementation of `E` is used.
///
/// The `M` type parameter controls whether [`Full`](crate::types::Full) or [`Short`](crate::types::Short)
/// type names are displayed.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::wrap::TypeNameResult;
/// use display_as_debug::types::{Full, Short};
///
/// let ok = Ok::<i32, &str>(42);
/// let err = Err::<i32, &str>("error");
/// assert_eq!(format!("{:?}", TypeNameResult::new::<Full>(ok)), "Ok(i32)");
/// assert_eq!(format!("{:?}", TypeNameResult::new::<Full>(err)), r#"Err("error")"#);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, AsMut)]
pub struct TypeNameResult<T, E, M: DisplayMode = Full>(
    #[deref]
    #[as_ref]
    #[as_mut]
    pub Result<T, E>,
    PhantomData<M>,
);

impl<T, E> TypeNameResult<T, E> {
    /// Create a new `TypeNameResult` wrapper.
    #[must_use]
    pub const fn new<M: DisplayMode>(result: Result<T, E>) -> TypeNameResult<T, E, M> {
        TypeNameResult(result, PhantomData)
    }
}

impl<T, E: Debug, M: DisplayMode> Debug for TypeNameResult<T, E, M>
where
    TypeName<T, M>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            Ok(_) => f.debug_tuple(STR_OK).field(&TypeName::<T, M>::default()).finish(),
            Err(e) => f.debug_tuple(STR_ERR).field(e).finish(),
        }
    }
}

impl<T, E, M: DisplayMode> From<Result<T, E>> for TypeNameResult<T, E, M> {
    fn from(result: Result<T, E>) -> Self {
        Self(result, PhantomData)
    }
}
