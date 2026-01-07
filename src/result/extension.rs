use super::{OpaqueResultDebug, ResultTypeDebug};

/// Extension trait providing convenience methods for debugging [`Result`] values.
///
/// This trait is automatically implemented for all [`Result`] types and provides ergonomic
/// access to the various debug wrappers without needing to construct them manually.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::result::ResultDebugExt;
///
/// let result: Result<i32, &str> = Ok(42);
///
/// // Using the extension methods
/// assert_eq!(format!("{:?}", result.debug_opaque()), "Ok(...)");
/// assert_eq!(format!("{:?}", result.debug_type()), "Ok(i32)");
/// ```
pub trait ResultDebugExt<T, E> {
    /// Returns a wrapper that implements [`Debug`] with opaque Ok values.
    ///
    /// Displays as `Ok(...)` when the result is [`Ok`], for [`Err`] the [`Debug`]
    /// implementation of `E` is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::result::ResultDebugExt;
    ///
    /// let ok_result: Result<&str, &str> = Ok("secret");
    /// assert_eq!(format!("{:?}", ok_result.debug_opaque()), "Ok(...)");
    ///
    /// let err_result: Result<&str, &str> = Err("connection failed");
    /// assert_eq!(format!("{:?}", err_result.debug_opaque()), "Err(\"connection failed\")");
    /// ```
    fn debug_opaque(&self) -> OpaqueResultDebug<'_, T, E>;

    /// Returns a wrapper that implements [`Debug`], displaying type names instead of values.
    ///
    /// Displays as `"Ok(typename)"` when the result is [`Ok`], for [`Err`] the [`Debug`]
    /// implementation of `E` is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::result::ResultDebugExt;
    ///
    /// let ok_result: Result<String, i32> = Ok("data".to_string());
    /// assert_eq!(format!("{:?}", ok_result.debug_type()), "Ok(alloc::string::String)");
    ///
    /// let err_result: Result<String, i32> = Err(404);
    /// assert_eq!(format!("{:?}", err_result.debug_type()), "Err(404)");
    /// ```
    fn debug_type(&self) -> ResultTypeDebug<'_, T, E>;
}

impl<T, E> ResultDebugExt<T, E> for Result<T, E> {
    fn debug_opaque(&self) -> OpaqueResultDebug<'_, T, E> {
        OpaqueResultDebug(self)
    }

    fn debug_type(&self) -> ResultTypeDebug<'_, T, E> {
        ResultTypeDebug(self)
    }
}
