use core::fmt::{Debug, Display, Formatter};
use core::marker::PhantomData;

use derive_more::{AsMut, AsRef, Deref};

use crate::types::type_name::display_mode::{DisplayMode, Full, Short};

/// A type that formats as a type name when used in [`Debug`] or [`Display`] contexts.
///
/// Can be used as:
/// - **Marker type**: `TypeName::<MyType>::FULL` - zero-sized, just prints the type name
/// - **Value wrapper**: `TypeName::of(value)` - wraps a value, prints its type name
///
/// # Type Parameters
///
/// - `T`: The type to display.
/// - `V`: The value to wrap. Defaults to `PhantomData<T>` for marker types.
/// - `M`: The display mode. Defaults to [`Short`](crate::types::Short).
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
pub struct TypeName<T: ?Sized = (), V = PhantomData<T>, M: DisplayMode = Short>(
    /// The wrapped value, if any. For marker types, this is `PhantomData<T>`.
    #[deref]
    #[as_mut]
    #[as_ref]
    pub V,
    PhantomData<(fn() -> T, M)>,
);

impl<T: ?Sized, M: DisplayMode> TypeName<T, PhantomData<T>, M> {
    /// A constant instance showing the full type name.
    pub const FULL: TypeName<T, PhantomData<T>, Full> = TypeName(PhantomData, PhantomData);

    /// A constant instance showing the short type name.
    pub const SHORT: TypeName<T, PhantomData<T>, Short> = TypeName(PhantomData, PhantomData);
}

impl TypeName {
    /// Creates an empty marker [`TypeName`] for the given type and [`DisplayMode`].
    ///
    /// Prefer to use the [`TypeName::FULL`] and [`TypeName::SHORT`] constants, unless the code has
    /// to be generic over [`DisplayMode`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::types::{TypeName, Full, Short};
    ///
    /// let full = TypeName::empty::<Vec<i32>, Full>();
    /// assert_eq!(format!("{full:?}"), "alloc::vec::Vec<i32>");
    ///
    /// let short = TypeName::empty::<Vec<i32>, Short>();
    /// assert_eq!(format!("{short:?}"), "Vec<i32>");
    /// ```
    #[must_use]
    pub fn empty<T: ?Sized, M: DisplayMode>() -> TypeName<T, PhantomData<T>, M> {
        TypeName(PhantomData, PhantomData)
    }
}

impl<T> TypeName<T, T, Full> {
    /// Wrap a value, displaying its type name in debug output.
    ///
    /// The type is inferred from the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::types::{TypeName, Full, Short};
    ///
    /// let wrapped = TypeName::wrap::<Short>(42i32);
    /// assert_eq!(format!("{:?}", wrapped), "i32");
    /// assert_eq!(*wrapped, 42, "Wrapped value is accessible");
    ///
    /// let wrapped = TypeName::wrap::<Full>(vec![1]);
    /// assert_eq!(format!("{:?}", wrapped), "alloc::vec::Vec<i32>");
    /// ```
    pub fn wrap<M: DisplayMode>(value: T) -> TypeName<T, T, M> {
        TypeName(value, PhantomData)
    }
}

impl<T, V, M: DisplayMode> TypeName<T, V, M> {
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
    pub fn into_inner(self) -> V {
        self.0
    }
}

impl<T> From<T> for TypeName<T, T, Full> {
    fn from(value: T) -> Self {
        Self::wrap(value)
    }
}

impl<T: ?Sized, V, M: DisplayMode> Debug for TypeName<T, V, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(M::type_name::<T>())
    }
}

impl<T: ?Sized, V, M: DisplayMode> Display for TypeName<T, V, M> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(M::type_name::<T>())
    }
}
