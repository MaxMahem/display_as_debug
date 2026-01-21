use core::marker::PhantomData;

use crate::types::{DisplayMode, Full, Short, TypeName};
use crate::wrap::TypeNameOption;

/// An alias for an empty [`TypeNameOption`] that is used as a marker.
pub type TypeNameOptionMarker<T, M> = TypeNameOption<T, PhantomData<fn() -> T>, M>;

impl<D: ?Sized> TypeNameOption<D, PhantomData<fn() -> D>, Short> {
    /// A constant for [`None`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::TypeNameOption;
    /// assert_eq!(format!("{:?}", TypeNameOption::<Vec<i32>>::NONE), "None");
    /// ```
    pub const NONE: TypeNameOptionMarker<D, Full> = TypeNameOption(None, TypeName::empty());

    /// A constant for [`Some`] containing a phantom value that uses the [`Full`] display mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::TypeNameOption;
    /// let full_some = TypeNameOption::<Vec<i32>>::SOME_FULL;
    /// assert_eq!(format!("{:?}", full_some), "Some(alloc::vec::Vec<i32>)");
    /// ```
    pub const SOME_FULL: TypeNameOptionMarker<D, Full> = TypeNameOption(Some(PhantomData), TypeName::empty());

    /// A constant for [`Some`] containing a phantom value that uses the [`Short`] display mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::TypeNameOption;
    /// let short_some = TypeNameOption::<Vec<i32>>::SOME_SHORT;
    /// assert_eq!(format!("{:?}", short_some), "Some(Vec<i32>)");
    /// ```
    #[allow(clippy::use_self, reason = "Symmetry with SOME_FULL")]
    pub const SOME_SHORT: TypeNameOptionMarker<D, Short> = TypeNameOption(Some(PhantomData), TypeName::empty());
}

impl TypeNameOption {
    /// Create a new marker [`TypeNameOption`] that is wrapped around no value. `option` is used
    /// only to determine the type and the state of the [`Option`].
    ///
    /// This is useful to produce a [`TypeNameOption`] that is used only for display.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::{Full, Short, TypeNameOption};
    /// let marker = TypeNameOption::empty::<i32, Full>(&None);
    /// assert_eq!(format!("{:?}", marker), "None");
    ///
    /// let marker = TypeNameOption::empty::<i32, Full>(&Some(0));
    /// assert_eq!(format!("{:?}", marker), "Some(i32)");
    /// ```
    #[must_use]
    pub const fn empty<D, M: DisplayMode>(option: &Option<D>) -> TypeNameOptionMarker<D, M> {
        match option {
            Some(_) => TypeNameOption(Some(PhantomData), TypeName::empty()),
            None => TypeNameOption(None, TypeName::empty()),
        }
    }
}
