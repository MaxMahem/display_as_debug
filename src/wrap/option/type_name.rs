use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use derive_more::{AsMut, AsRef, Deref, Into};

use crate::fmt::DebugTupleExt;
use crate::types::{DisplayMode, Short, TypeName, TypeNameMarker};
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
/// # use display_as_debug::wrap::{TypeNameOption, Full, Short};
/// let short = TypeNameOption::new::<Short>(Some(vec![1]));
/// assert_eq!(format!("{:?}", short), "Some(Vec<i32>)");
///
/// let full = TypeNameOption::new::<Full>(Some(vec![1]));
/// assert_eq!(format!("{:?}", full), "Some(alloc::vec::Vec<i32>)");
///
/// let none = TypeNameOption::new::<Full>(None::<i32>);
/// assert_eq!(format!("{:?}", none), "None");
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, AsMut, Into)]
pub struct TypeNameOption<D: ?Sized = (), T = PhantomData<fn() -> D>, M: DisplayMode = Short>(
    #[deref]
    #[as_ref]
    #[as_mut]
    pub Option<T>,
    #[into(ignore)] pub(crate) TypeNameMarker<D, M>,
);

#[allow(clippy::mismatching_type_param_order, reason = "T is used for both Display and Value")]
impl<T> TypeNameOption<T, T> {
    /// Create a new [`TypeNameOption`] wrapper.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::{TypeNameOption, Full, Short};
    /// let short = TypeNameOption::new::<Short>(Some(vec![1]));
    /// assert_eq!(format!("{:?}", short), "Some(Vec<i32>)");
    ///
    /// let full = TypeNameOption::new::<Full>(Some(vec![1]));
    /// assert_eq!(format!("{:?}", full), "Some(alloc::vec::Vec<i32>)");
    ///
    /// let none = TypeNameOption::new::<Full>(None::<i32>);
    /// assert_eq!(format!("{:?}", none), "None");
    /// ```
    #[must_use]
    pub const fn new<M: DisplayMode>(option: Option<T>) -> TypeNameOption<T, T, M> {
        TypeNameOption(option, TypeName::empty())
    }

    /// Create a new [`TypeNameOption`] wrapper that borrows the value but
    /// displays the inner type name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::{TypeNameOption, Full, Short};
    /// let option = Some(vec![1]);
    ///
    /// let full = TypeNameOption::borrow::<Full>(&option);
    /// assert_eq!(format!("{:?}", full), "Some(alloc::vec::Vec<i32>)");
    ///
    /// let short = TypeNameOption::borrow::<Short>(&option);
    /// assert_eq!(format!("{:?}", short), "Some(Vec<i32>)");
    ///
    /// let none = TypeNameOption::borrow::<Full>(&None::<i32>);
    /// assert_eq!(format!("{:?}", none), "None");
    /// ```
    #[must_use]
    pub const fn borrow<M: DisplayMode>(option: &Option<T>) -> TypeNameOption<T, &T, M> {
        TypeNameOption(option.as_ref(), TypeName::empty())
    }
}

impl<D: ?Sized, T, M: DisplayMode> Debug for TypeNameOption<D, T, M>
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

impl<D: ?Sized, T, M: DisplayMode> From<Option<T>> for TypeNameOption<D, T, M> {
    fn from(option: Option<T>) -> Self {
        Self(option, TypeName::empty())
    }
}
