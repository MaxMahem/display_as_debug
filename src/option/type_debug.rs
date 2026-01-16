use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use derive_more::{AsMut, AsRef, Deref};

use crate::fmt::DebugTupleExt;
use crate::option::{STR_NONE, STR_SOME};
use crate::types::{DisplayMode, Full, TypeName};

/// A [`Option<T>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
/// This avoids requiring `T: Debug` while still providing useful debug output.
///
/// The `M` type parameter controls whether [`Full`](crate::types::Full) or [`Short`](crate::types::Short)
/// type names are displayed.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, AsMut)]
pub struct TypeNameOption<'a, T, M: DisplayMode = Full> {
    #[deref]
    #[as_ref]
    #[as_mut]
    inner: &'a Option<T>,
    _marker: PhantomData<M>,
}

impl<'a, T> TypeNameOption<'a, T> {
    /// Create a new `TypeNameOption` wrapper.
    pub const fn new<M: DisplayMode>(option: &'a Option<T>) -> TypeNameOption<'a, T, M> {
        TypeNameOption { inner: option, _marker: PhantomData }
    }
}

impl<'a, T, M: DisplayMode> From<&'a Option<T>> for TypeNameOption<'a, T, M> {
    fn from(option: &'a Option<T>) -> Self {
        Self { inner: option, _marker: PhantomData }
    }
}

impl<T, M: DisplayMode> Debug for TypeNameOption<'_, T, M>
where
    TypeName<T, M>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.inner {
            Some(_) => f.debug_tuple(STR_SOME).field_type::<T, M>().finish(),
            None => f.write_str(STR_NONE),
        }
    }
}
