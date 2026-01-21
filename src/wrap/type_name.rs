//! Core [`TypeName`] struct and wrapper implementations.

use core::fmt::{Debug, Formatter};
use core::marker::PhantomData;

use derive_more::{AsMut, AsRef, Deref};

use crate::types::{DisplayMode, Full, Short};

/// A type that formats as a type name when used in [`Debug`] contexts.
///
/// Can be used as:
/// - **Marker Type**: via
///   - [`TypeName::FULL`] or [`TypeName::SHORT`] constants,
///   - [`TypeName::empty`] generic implementation
///   - zero-sized, just prints the type name
/// - **Value Wrapper** via:
///   - [`TypeName::wrap`] or (from/into)
///   - wraps a value, prints its type name
///
/// # Type Parameters
///
/// - `D`: The type who's name to display. Same as `V` for value wrappers.
/// - `T`: The value to wrap. [`PhantomData<fn() -> T>`](PhantomData) for marker types.
/// - `M`: The display mode.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::types::{TypeName, Full, Short};
///
/// // Marker usage (no wrapped value)
/// assert_eq!(format!("{:?}", TypeName::<Vec<i32>>::FULL), "alloc::vec::Vec<i32>");
/// assert_eq!(format!("{:?}", TypeName::<Vec<i32>>::SHORT), "Vec<i32>");
///
/// // Value wrapper usage
/// let wrapped = TypeName::wrap::<Full>(vec![1, 2, 3]);
/// assert_eq!(format!("{:?}", wrapped), "alloc::vec::Vec<i32>");
/// assert_eq!(*wrapped, vec![1, 2, 3]); // Can still access the value
/// ```
#[derive(Copy, Clone, Deref, AsMut, AsRef, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeName<D: ?Sized = (), T = PhantomData<fn() -> D>, M: DisplayMode = Short>(
    /// The wrapped value, if any. For marker types, this is [`PhantomData<fn() -> T>`](PhantomData).
    #[deref]
    #[as_mut]
    #[as_ref]
    pub T,
    pub(crate) PhantomData<(fn() -> D, M)>,
);

#[allow(clippy::mismatching_type_param_order, reason = "T is used for both Value and Type")]
impl<T> TypeName<T, T, Full> {
    /// Wrap a value in a [`TypeName`], displaying its type name in debug output.
    ///
    /// The display type is inferred from the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::types::{TypeName, Full, Short};
    ///
    /// let wrapped = TypeName::wrap::<Short>(42i32);
    /// assert_eq!(format!("{:?}", wrapped), "i32");
    ///
    /// let wrapped = TypeName::wrap::<Full>(vec![1]);
    /// assert_eq!(format!("{:?}", wrapped), "alloc::vec::Vec<i32>");
    /// ```
    pub fn wrap<M: DisplayMode>(value: T) -> TypeName<T, T, M> {
        TypeName(value, PhantomData)
    }

    /// Wrap a borrowed value in a [`TypeName`], displaying its type name in debug output.
    ///
    /// The type is inferred from the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::types::{TypeName, Full, Short};
    ///
    /// let vec = vec![1];
    /// let wrapped = TypeName::borrow::<Short>(&vec);
    /// assert_eq!(format!("{:?}", wrapped), "Vec<i32>");
    ///
    /// let wrapped = TypeName::borrow::<Full>(&vec);
    /// assert_eq!(format!("{:?}", wrapped), "alloc::vec::Vec<i32>");
    /// ```
    pub fn borrow<M: DisplayMode>(value: &T) -> TypeName<T, &T, M> {
        TypeName(value, PhantomData)
    }
}

impl<D, T, M: DisplayMode> TypeName<D, T, M> {
    /// Consumes the wrapper, returning the inner value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::types::{TypeName, Full, Short};
    ///
    /// let wrapped = TypeName::wrap::<Short>(42i32);
    /// assert_eq!(wrapped.into_inner(), 42);
    /// ```
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[allow(clippy::mismatching_type_param_order, reason = "T is used for both Display and Value")]
impl<T, M: DisplayMode> From<T> for TypeName<T, T, M> {
    fn from(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<D: ?Sized, T, M: DisplayMode> Debug for TypeName<D, T, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(M::type_name::<D>())
    }
}
