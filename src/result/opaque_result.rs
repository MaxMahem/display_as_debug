use core::fmt::{Debug, Formatter};

use derive_more::{AsMut, AsRef, Deref, From};

use crate::fmt::DebugTupleExt;
use crate::result::{STR_ERR, STR_OK};

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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, From, Deref, AsRef, AsMut)]
pub struct OpaqueResult<'a, T, E>(pub &'a Result<T, E>);

impl<T, E: Debug> Debug for OpaqueResult<'_, T, E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            Ok(_) => f.debug_tuple(STR_OK).field_opaque().finish(),
            Err(e) => f.debug_tuple(STR_ERR).field(e).finish(),
        }
    }
}
