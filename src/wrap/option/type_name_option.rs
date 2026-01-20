use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use derive_more::{AsMut, AsRef, Deref};

use crate::fmt::DebugTupleExt;
use crate::types::{DisplayMode, Short, TypeName};
use crate::wrap::option::{STR_NONE, STR_SOME};

/// A [`Option<T>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
/// This avoids requiring `T: Debug` while still providing useful debug output.
///
/// The `M` type parameter controls whether [`Full`](crate::types::Full) or [`Short`](crate::types::Short)
/// type names are displayed.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::wrap::TypeNameOption;
/// use display_as_debug::types::{Full, Short};
///
/// assert_eq!(format!("{:?}", TypeNameOption::new::<Short>(Some(vec![1]))), "Some(Vec<i32>)");
/// assert_eq!(format!("{:?}", TypeNameOption::new::<Full>(Some(vec![1]))), "Some(alloc::vec::Vec<i32>)");
/// assert_eq!(format!("{:?}", TypeNameOption::new::<Full>(None::<i32>)), "None");
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, AsMut)]
pub struct TypeNameOption<T, D: ?Sized = T, M: DisplayMode = Short>(
    #[deref]
    #[as_ref]
    #[as_mut]
    pub Option<T>,
    PhantomData<M>,
    PhantomData<D>,
);

impl<T> TypeNameOption<T> {
    /// Create a new `TypeNameOption` wrapper.
    #[must_use]
    pub const fn new<M: DisplayMode>(option: Option<T>) -> TypeNameOption<T, T, M> {
        TypeNameOption(option, PhantomData, PhantomData)
    }

    /// Create a new `TypeNameOption` wrapper that borrows the value but displays the inner type name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::wrap::TypeNameOption;
    /// use display_as_debug::types::{Full, Short};
    ///
    /// let option = Some(vec![1]);
    /// let wrapper = TypeNameOption::borrowed::<Full>(&option);
    /// assert_eq!(format!("{:?}", wrapper), "Some(alloc::vec::Vec<i32>)");
    /// ```
    #[must_use]
    pub const fn borrowed<M: DisplayMode>(option: &Option<T>) -> TypeNameOption<&T, T, M> {
        TypeNameOption(option.as_ref(), PhantomData, PhantomData)
    }
}

impl<T, D: ?Sized, M: DisplayMode> Debug for TypeNameOption<T, D, M>
where
    TypeName<D, M>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            Some(_) => f.debug_tuple(STR_SOME).field_type::<D, M>().finish(),
            None => f.write_str(STR_NONE),
        }
    }
}

impl<T, D: ?Sized, M: DisplayMode> From<Option<T>> for TypeNameOption<T, D, M> {
    fn from(option: Option<T>) -> Self {
        Self(option, PhantomData, PhantomData)
    }
}
