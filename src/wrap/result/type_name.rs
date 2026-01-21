use core::fmt::{Debug, Formatter};

use derive_more::{AsMut, AsRef, Deref, Into};

use crate::fmt::DebugTupleExt;
use crate::types::{DisplayMode, Full, TypeName, TypeNameMarker};
use crate::wrap::result::{STR_ERR, STR_OK};

/// A [`Result<T, E>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"Ok(typename)"` when the result is [`Ok`], for [`Err`] the [`Debug`]
/// implementation of `E` is used.
///
/// The `M` type parameter controls whether [`Full`](crate::types::Full) or [`Short`](crate::types::Short)
/// type names are displayed.
///
/// # Type Parameters
///
/// - `T`: The type of the value in the [`Ok`] variant.
/// - `E`: The type of the value in the [`Err`] variant.
/// - `D`: The type that is displayed in the [`Ok`] variant.
///   - Used internally to display the type name as `T` when borrowing a [`Result`].
/// - `M`: The display mode for the type name.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::wrap::TypeNameResult;
/// # use display_as_debug::types::{Full, Short};
/// let ok = Ok::<_, &str>(vec![1]);
/// let short = TypeNameResult::new::<Short>(ok);
/// assert_eq!(format!("{:?}", short), "Ok(Vec<i32>)");
///
/// let ok = Ok::<_, &str>(vec![1]);
/// let full = TypeNameResult::new::<Full>(ok);
/// assert_eq!(format!("{:?}", full), "Ok(alloc::vec::Vec<i32>)");
///
/// let err = Err::<i32, &str>("error");
/// assert_eq!(format!("{:?}", TypeNameResult::new::<Full>(err)), r#"Err("error")"#);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, AsRef, AsMut, Into)]
pub struct TypeNameResult<T, E, D: ?Sized = T, M: DisplayMode = Full>(
    #[deref]
    #[as_ref]
    #[as_mut]
    pub Result<T, E>,
    #[into(ignore)] TypeNameMarker<D, M>,
);

impl<T, E> TypeNameResult<T, E> {
    /// Create a new [`TypeNameResult`] wrapper.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::TypeNameResult;
    /// # use display_as_debug::types::{Full, Short};
    /// let ok = Ok::<i32, &str>(42);
    /// assert_eq!(format!("{:?}", TypeNameResult::new::<Full>(ok)), "Ok(i32)");
    ///
    /// let err = Err::<i32, &str>("error");
    /// assert_eq!(format!("{:?}", TypeNameResult::new::<Full>(err)), r#"Err("error")"#);
    /// ```
    #[must_use]
    pub const fn new<M: DisplayMode>(result: Result<T, E>) -> TypeNameResult<T, E, T, M> {
        TypeNameResult(result, TypeName::empty())
    }

    /// Create a new [`TypeNameResult`] wrapper that borrows the value but displays the inner type name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::TypeNameResult;
    /// # use display_as_debug::types::{Full, Short};
    /// let ok = Ok::<i32, &str>(42);
    /// assert_eq!(format!("{:?}", TypeNameResult::borrow::<Full>(&ok)), "Ok(i32)");
    ///
    /// let err = Err::<i32, &str>("error");
    /// assert_eq!(format!("{:?}", TypeNameResult::borrow::<Full>(&err)), r#"Err("error")"#);
    /// ```
    #[must_use]
    pub const fn borrow<M: DisplayMode>(result: &Result<T, E>) -> TypeNameResult<&T, &E, T, M> {
        TypeNameResult(result.as_ref(), TypeName::empty())
    }

    /// Consumes the wrapper, returning the inner value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::TypeNameResult;
    /// # use display_as_debug::types::{Short};
    /// let result = Ok::<_, &str>(vec![1]);
    /// let wrapper = TypeNameResult::new::<Short>(result);
    /// assert_eq!(wrapper.into_inner(), Ok(vec![1]));
    /// ```
    #[allow(clippy::missing_errors_doc, reason = "Unwrapping a result, not returning an error")]
    pub fn into_inner(self) -> Result<T, E> {
        self.0
    }
}

impl<T, E: Debug, D: ?Sized, M: DisplayMode> Debug for TypeNameResult<T, E, D, M>
where
    TypeName<D, M>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            Ok(_) => f.debug_tuple(STR_OK).field_type::<D, M>().finish(),
            Err(e) => f.debug_tuple(STR_ERR).field(e).finish(),
        }
    }
}

impl<T, E, D: ?Sized, M: DisplayMode> From<Result<T, E>> for TypeNameResult<T, E, D, M> {
    fn from(result: Result<T, E>) -> Self {
        Self(result, TypeName::empty())
    }
}
