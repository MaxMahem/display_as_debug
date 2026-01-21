use core::marker::PhantomData;

use crate::types::{DisplayMode, Full, Short, TypeName};
use crate::wrap::TypeNameResult;

/// An alias for an empty [`TypeNameResult`] that is used as an empty marker/[`Debug`]
/// only type.
///
/// Note, that because the [`Err`] variant of [`TypeNameResult`] is not empty
/// (it displays the error value), a marker type can only represent the [`Ok`] variant.
pub type TypeNameResultMarker<T, M> = TypeNameResult<PhantomData<fn() -> T>, (), T, M>;

impl<T> TypeNameResult<T, ()> {
    /// An empty marker constant for [`Ok`] containing that uses the [`Full`] display mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::TypeNameResult;
    /// let full_ok = TypeNameResult::<Vec<i32>>::OK_FULL;
    /// assert_eq!(format!("{:?}", full_ok), "Ok(alloc::vec::Vec<i32>)");
    /// ```
    pub const OK_FULL: TypeNameResultMarker<T, Full> = TypeNameResult::ok_empty::<T, Full>();

    /// An empty marker constant for [`Ok`] containing that uses the [`Short`] display mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::TypeNameResult;
    /// let short_ok = TypeNameResult::<Vec<i32>>::OK_SHORT;
    /// assert_eq!(format!("{:?}", short_ok), "Ok(Vec<i32>)");
    /// ```
    #[allow(clippy::use_self, reason = "Symmetry with OK_FULL")]
    pub const OK_SHORT: TypeNameResultMarker<T, Short> = TypeNameResult::ok_empty::<T, Short>();
}

impl TypeNameResult<(), ()> {
    /// Create a new empty marker [`TypeNameResult`] in the [`Ok`] state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::{Full, Short, TypeNameResult};
    /// let marker = TypeNameResult::ok_empty::<i32, Full>();
    /// assert_eq!(format!("{:?}", marker), "Ok(i32)");
    /// ```
    #[must_use]
    pub const fn ok_empty<T, M: DisplayMode>() -> TypeNameResultMarker<T, M> {
        TypeNameResult(Ok(PhantomData), TypeName::empty())
    }
}
