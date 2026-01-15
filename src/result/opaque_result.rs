use core::fmt::{Debug, Formatter};

use crate::fmt::DebugTupleExt;

/// A [`Result<T, E>`] wrapper that implements [`Debug`] with opaque Ok values.
///
/// Displays as `Ok(..)` when the result is [`Ok`], or `Err(error_value)` when [`Err`].
/// This provides privacy for Ok values while fully debugging errors.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::result::OpaqueResult;
/// assert_eq!(format!("{:?}", OpaqueResult(&Ok::<_, &str>(42))), "Ok(..)");
/// assert_eq!(format!("{:?}", OpaqueResult(&Err::<i32, &str>("fail"))), r#"Err("fail")"#);
/// ```
pub struct OpaqueResult<'a, T, E>(pub &'a Result<T, E>);

impl<T, E: Debug> Debug for OpaqueResult<'_, T, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            Ok(_) => f.debug_tuple("Ok").field_opaque().finish(),
            Err(e) => f.debug_tuple("Err").field(e).finish(),
        }
    }
}
