use core::fmt::{Debug, Formatter};

use crate::fmt::DebugTupleExt;

/// A [`Option<T>`] wrapper that implements [`Debug`] with opaque Some values.
///
/// Displays as `Some(..)` when the option is [`Some`], or `None` when [`None`].
/// This provides privacy for Some values while still indicating the option's state.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::option::OpaqueOption;
/// assert_eq!(format!("{:?}", OpaqueOption(&Some(42))), "Some(..)");
/// assert_eq!(format!("{:?}", OpaqueOption(&None::<i32>)), "None");
/// ```
pub struct OpaqueOption<'a, T>(pub &'a Option<T>);

impl<T> Debug for OpaqueOption<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            Some(_) => f.debug_tuple("Some").field_opaque().finish(),
            None => f.write_str("None"),
        }
    }
}
