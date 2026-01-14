use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use crate::type_name::{DisplayMode, Full, TypeName};

/// A [`Option<T>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
/// This avoids requiring `T: Debug` while still providing useful debug output.
///
/// The `M` type parameter controls whether [`Full`](crate::type_name::Full) or [`Short`](crate::type_name::Short)
/// type names are displayed.
#[derive(Copy, Clone)]
pub struct OptionTypeDebug<'a, T, M: DisplayMode = Full> {
    inner: &'a Option<T>,
    _marker: PhantomData<M>,
}

impl<'a, T> OptionTypeDebug<'a, T> {
    /// Create a new `OptionTypeDebug` wrapper.
    pub const fn new<M: DisplayMode>(option: &'a Option<T>) -> OptionTypeDebug<'a, T, M> {
        OptionTypeDebug { inner: option, _marker: PhantomData }
    }
}

impl<T, M: DisplayMode> Debug for OptionTypeDebug<'_, T, M>
where
    TypeName<T, M>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.inner {
            Some(_) => f.debug_tuple("Some").field(&TypeName::<T, M>::default()).finish(),
            None => f.write_str("None"),
        }
    }
}
