use core::fmt::{Debug, Formatter};

use crate::Opaque;

/// A [`Result<T, E>`] wrapper that implements [`Debug`] with opaque Ok values.
///
/// Displays as `Ok(..)` when the result is [`Ok`], or `Err(error_value)` when [`Err`].
/// This provides privacy for Ok values while fully debugging errors.
pub struct OpaqueResultDebug<'a, T, E>(pub &'a Result<T, E>);

impl<T, E: Debug> Debug for OpaqueResultDebug<'_, T, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            Ok(_) => f.debug_tuple("Ok").field(&Opaque).finish(),
            Err(e) => f.debug_tuple("Err").field(e).finish(),
        }
    }
}
