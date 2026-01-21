//! Const marker implementations for [`TypeName`].

use core::marker::PhantomData;

use crate::types::{DisplayMode, Full, Short};
use crate::wrap::TypeName;

/// An alias for a [`TypeName`] that wraps no value and serves only as a marker.
pub type TypeNameMarker<T, M> = TypeName<T, PhantomData<fn() -> T>, M>;

impl<T: ?Sized, M: DisplayMode> TypeNameMarker<T, M> {
    /// A constant instance showing the full type name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::{TypeName};
    /// let full = TypeName::<Vec<i32>>::FULL;
    /// assert_eq!(format!("{:?}", full), "alloc::vec::Vec<i32>");
    /// ```
    pub const FULL: TypeNameMarker<T, Full> = TypeName(PhantomData, PhantomData);

    /// A constant instance showing the short type name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::{TypeName};
    /// let short = TypeName::<Vec<i32>>::SHORT;
    /// assert_eq!(format!("{:?}", short), "Vec<i32>");
    /// ```
    pub const SHORT: TypeNameMarker<T, Short> = TypeName(PhantomData, PhantomData);
}

impl TypeName {
    /// Creates an empty marker [`TypeName`] for the given type and [`DisplayMode`].
    ///
    /// Prefer to use the [`TypeName::FULL`] and [`TypeName::SHORT`] constants,
    /// unless the code has to be generic over [`DisplayMode`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::{TypeName, Full, Short};
    /// let full = TypeName::empty::<Vec<i32>, Full>();
    /// assert_eq!(format!("{full:?}"), "alloc::vec::Vec<i32>");
    ///
    /// let short = TypeName::empty::<Vec<i32>, Short>();
    /// assert_eq!(format!("{short:?}"), "Vec<i32>");
    /// ```
    #[must_use]
    pub const fn empty<T: ?Sized, M: DisplayMode>() -> TypeNameMarker<T, M> {
        TypeName(PhantomData, PhantomData)
    }
}
