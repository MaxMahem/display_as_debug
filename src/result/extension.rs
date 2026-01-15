use super::{OpaqueResult, TypeNameResult};
use crate::types::DisplayMode;

/// Extension trait providing convenience methods for debugging [`Result`] values.
#[sealed::sealed]
pub trait ResultDebugExt<T, E> {
    /// Returns a wrapper that implements [`Debug`] with opaque Ok values.
    ///
    /// Displays as `Ok(..)` when the result is [`Ok`], for [`Err`] the [`Debug`]
    /// implementation of `E` is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::result::ResultDebugExt;
    /// assert_eq!(format!("{:?}", Ok::<_, &str>(42).debug_opaque()), "Ok(..)");
    /// assert_eq!(format!("{:?}", Err::<i32, _>("failed").debug_opaque()), r#"Err("failed")"#);
    /// ```
    fn debug_opaque(&self) -> OpaqueResult<'_, T, E>;

    /// Returns a wrapper that implements [`Debug`], displaying type names instead of values.
    ///
    /// Displays as `"Ok(typename)"` when the result is [`Ok`], for [`Err`] the [`Debug`]
    /// implementation of `E` is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::result::ResultDebugExt;
    /// use display_as_debug::types::{Full, Short};
    ///
    /// assert_eq!(format!("{:?}", Ok::<_, &str>(vec![1]).debug_type_name::<Full>()), "Ok(alloc::vec::Vec<i32>)");
    /// assert_eq!(format!("{:?}", Ok::<_, &str>(vec![1]).debug_type_name::<Short>()), "Ok(Vec<i32>)");
    /// assert_eq!(format!("{:?}", Err::<i32, _>("failed").debug_type_name::<Full>()), r#"Err("failed")"#);
    /// assert_eq!(format!("{:?}", Err::<i32, _>("failed").debug_type_name::<Short>()), r#"Err("failed")"#);
    /// ```
    fn debug_type_name<M: DisplayMode>(&self) -> TypeNameResult<'_, T, E, M>;
}

#[sealed::sealed]
impl<T, E> ResultDebugExt<T, E> for Result<T, E> {
    fn debug_opaque(&self) -> OpaqueResult<'_, T, E> {
        OpaqueResult(self)
    }

    fn debug_type_name<M: DisplayMode>(&self) -> TypeNameResult<'_, T, E, M> {
        TypeNameResult::new::<M>(self)
    }
}
