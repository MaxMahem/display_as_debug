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
///
/// This wrapper owns the `Option<T>` directly. For borrowed usage, use `Option::as_ref()`:
/// - `TypeNameOption::new(Some(value))` — owned inner value
/// - `TypeNameOption::new(opt.as_ref())` — borrowed inner value
///
/// # Examples
///
/// ```rust
/// use display_as_debug::option::TypeNameOption;
/// use display_as_debug::types::{Full, Short};
///
/// assert_eq!(format!("{:?}", TypeNameOption::new::<Short>(Some(vec![1]))), "Some(Vec<i32>)");
/// assert_eq!(format!("{:?}", TypeNameOption::new::<Full>(Some(vec![1]))), "Some(alloc::vec::Vec<i32>)");
/// assert_eq!(format!("{:?}", TypeNameOption::new::<Full>(None::<i32>)), "None");
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, AsMut)]
pub struct TypeNameOption<T, M: DisplayMode = Full>(
    #[deref]
    #[as_ref]
    #[as_mut]
    pub Option<T>,
    PhantomData<M>,
);

impl<T> TypeNameOption<T, Full> {
    /// Create a new `TypeNameOption` wrapper.
    #[must_use]
    pub const fn new<M: DisplayMode>(option: Option<T>) -> TypeNameOption<T, M> {
        TypeNameOption(option, PhantomData)
    }
}

impl<T, M: DisplayMode> Debug for TypeNameOption<T, M>
where
    TypeName<T, M>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            Some(_) => f.debug_tuple(STR_SOME).field_type::<T, M>().finish(),
            None => f.write_str(STR_NONE),
        }
    }
}

impl<T, M: DisplayMode> From<Option<T>> for TypeNameOption<T, M> {
    fn from(option: Option<T>) -> Self {
        Self(option, PhantomData)
    }
}
