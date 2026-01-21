use core::fmt::{Debug, Formatter};

use derive_more::{AsMut, AsRef, Deref, From};

use crate::fmt::DebugTupleExt;
use crate::wrap::result::{STR_ERR, STR_OK};

/// A [`Result<T, E>`] wrapper that implements [`Debug`] with opaque Ok values.
///
/// Displays as `Ok(..)` when the result is [`Ok`], or `Err(error_value)` when [`Err`].
/// This provides privacy for Ok values while fully debugging errors.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::wrap::OpaqueResult;
/// assert_eq!(format!("{:?}", OpaqueResult(Ok::<_, &str>(42))), "Ok(..)");
/// assert_eq!(format!("{:?}", OpaqueResult(Err::<i32, &str>("fail"))), r#"Err("fail")"#);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Deref, AsRef, AsMut)]
pub struct OpaqueResult<T, E>(pub Result<T, E>);

/// An alias for an empty [`OpaqueResult`] in the [`Ok`] state that is used as an empty
/// marker/[`Debug`] only type.
///
/// Note, that because the [`Err`] variant of [`OpaqueResult`] is not empty
/// (it displays the error value), a marker type can only represent the [`Ok`] variant.
pub type OpaqueResultMarker = OpaqueResult<(), ()>;

impl OpaqueResult<(), ()> {
    /// An empty marker constant for [`Ok`] state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::OpaqueResult;
    /// assert_eq!(format!("{:?}", OpaqueResult::OK), "Ok(..)");
    /// ```
    pub const OK: Self = Self(Ok(()));
}

impl<T, E> OpaqueResult<T, E> {
    /// Create a new [`OpaqueResult`] wrapper that borrows the wrapped value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::OpaqueResult;
    /// let res: Result<&str, &str> = Ok("secret");
    /// assert_eq!(format!("{:?}", OpaqueResult::borrow(&res)), "Ok(..)");
    /// ```
    #[must_use]
    pub const fn borrow(option: &Result<T, E>) -> OpaqueResult<&T, &E> {
        OpaqueResult(option.as_ref())
    }
}

impl<T, E: Debug> Debug for OpaqueResult<T, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            Ok(_) => f.debug_tuple(STR_OK).field_opaque().finish(),
            Err(e) => f.debug_tuple(STR_ERR).field(e).finish(),
        }
    }
}
