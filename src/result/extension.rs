use super::{OpaqueResultDebug, ResultTypeDebug};
use crate::type_name::DisplayMode;

/// Extension trait providing convenience methods for debugging [`Result`] values.
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
    /// assert_eq!(format!("{:?}", Err::<i32, _>("connection failed").debug_opaque()), "Err(\"connection failed\")");
    /// ```
    fn debug_opaque(&self) -> OpaqueResultDebug<'_, T, E>;

    /// Returns a wrapper that implements [`Debug`], displaying type names instead of values.
    ///
    /// Displays as `"Ok(typename)"` when the result is [`Ok`], for [`Err`] the [`Debug`]
    /// implementation of `E` is used.
    ///
    /// The display mode ([`Full`](crate::type_name::Full) or [`Short`](crate::type_name::Short)) must be
    /// specified as a generic argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::result::ResultDebugExt;
    /// use display_as_debug::type_name::{Full, Short};
    ///
    /// assert_eq!(format!("{:?}", Ok::<_, &str>(vec![1]).debug_type::<Full>()), "Ok(alloc::vec::Vec<i32>)");
    /// assert_eq!(format!("{:?}", Ok::<_, &str>(vec![1]).debug_type::<Short>()), "Ok(Vec<i32>)");
    /// assert_eq!(format!("{:?}", Err::<i32, _>("failed").debug_type::<Full>()), "Err(\"failed\")");
    /// assert_eq!(format!("{:?}", Err::<i32, _>("failed").debug_type::<Short>()), "Err(\"failed\")");
    /// ```
    fn debug_type<M: DisplayMode>(&self) -> ResultTypeDebug<'_, T, E, M>;
}

impl<T, E> ResultDebugExt<T, E> for Result<T, E> {
    fn debug_opaque(&self) -> OpaqueResultDebug<'_, T, E> {
        OpaqueResultDebug(self)
    }

    fn debug_type<M: DisplayMode>(&self) -> ResultTypeDebug<'_, T, E, M> {
        ResultTypeDebug::new::<M>(self)
    }
}
