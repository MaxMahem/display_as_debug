use core::fmt::Debug;

use crate::as_debug::DisplayDebug;

/// A [`Result<T, E>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"Ok(typename)"` when the result is [`Ok`], for [`Err`] the [`Debug`]
/// implementation of `E` is used.
pub struct ResultTypeDebug<'a, T, E>(pub &'a Result<T, E>);

impl<T, E: Debug> Debug for ResultTypeDebug<'_, T, E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            Ok(_) => f.debug_tuple("Ok").field(&core::any::type_name::<T>().as_debug()).finish(),
            Err(e) => f.debug_tuple("Err").field(e).finish(),
        }
    }
}
