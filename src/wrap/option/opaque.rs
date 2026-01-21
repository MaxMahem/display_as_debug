use core::fmt::{Debug, Formatter};

use derive_more::{AsMut, AsRef, Deref, From};

use crate::fmt::DebugTupleExt;
use crate::wrap::option::{STR_NONE, STR_SOME};

/// A [`Option<T>`] wrapper that implements [`Debug`] with opaque Some values.
///
/// Displays as `Some(..)` when the option is [`Some`], or `None` when [`None`].
/// This provides privacy for Some values while still indicating the option's state.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::wrap::OpaqueOption;
/// assert_eq!(format!("{:?}", OpaqueOption(Some(42))), "Some(..)");
/// assert_eq!(format!("{:?}", OpaqueOption(None::<i32>)), "None");
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Deref, AsRef, AsMut)]
pub struct OpaqueOption<T>(pub Option<T>);

/// An alias for an empty [`OpaqueOption`] that is used as an empty marker/[`Debug`] only type.
pub type OpaqueOptionMarker = OpaqueOption<()>;

impl OpaqueOption<()> {
    /// An empty marker constant for [`Some`] state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::OpaqueOption;
    /// assert_eq!(format!("{:?}", OpaqueOption::SOME), "Some(..)");
    /// ```
    pub const SOME: Self = Self(Some(()));

    /// An empty marker constant for [`None`] state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::OpaqueOption;
    /// assert_eq!(format!("{:?}", OpaqueOption::NONE), "None");
    /// ```
    pub const NONE: Self = Self(None);
}

impl<T> OpaqueOption<T> {
    /// Create a new [`OpaqueOption`] wrapper that borrows the wrapped value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::wrap::OpaqueOption;
    /// let option = Some("secret");
    /// assert_eq!(format!("{:?}", OpaqueOption::borrow(&option)), "Some(..)");
    /// ```
    #[must_use]
    pub const fn borrow(option: &Option<T>) -> OpaqueOption<&T> {
        OpaqueOption(option.as_ref())
    }
}

impl<T> Debug for OpaqueOption<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            Some(_) => f.debug_tuple(STR_SOME).field_opaque().finish(),
            None => f.write_str(STR_NONE),
        }
    }
}
