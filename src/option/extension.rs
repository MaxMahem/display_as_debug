use super::{OpaqueOptionDebug, OptionTypeDebug};

/// Extension trait providing convenience methods for debugging [`Option`] values.
///
/// This trait is automatically implemented for all [`Option`] types and provides ergonomic
/// access to the various debug wrappers without needing to construct them manually.
///
/// # Examples
///
/// ```rust
/// use display_as_debug::option::OptionDebugExt;
///
/// let opt = Some(42);
///
/// // Using the extension methods
/// assert_eq!(format!("{:?}", opt.debug_opaque()), "Some(..)");
/// assert_eq!(format!("{:?}", opt.debug_type()), "Some(i32)");
/// ```
pub trait OptionDebugExt<T> {
    /// Returns a wrapper that implements [`Debug`] with opaque Some values.
    ///
    /// Displays as `Some(..)` when the option is [`Some`], or `None` when [`None`].
    /// This provides privacy for Some values while still indicating the option's state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::option::OptionDebugExt;
    ///
    /// let opt = Some("sensitive data");
    /// assert_eq!(format!("{:?}", opt.debug_opaque()), "Some(..)");
    ///
    /// let none: Option<&str> = None;
    /// assert_eq!(format!("{:?}", none.debug_opaque()), "None");
    /// ```
    fn debug_opaque(&self) -> OpaqueOptionDebug<'_, T>;

    /// Returns a wrapper that implements [`Debug`], displaying type names instead of values.
    ///
    /// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
    /// This avoids requiring `T: Debug` while still providing useful debug output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::option::OptionDebugExt;
    ///
    /// let opt = Some(42);
    /// assert_eq!(format!("{:?}", opt.debug_type()), "Some(i32)");
    ///
    /// let none: Option<i32> = None;
    /// assert_eq!(format!("{:?}", none.debug_type()), "None");
    /// ```
    fn debug_type(&self) -> OptionTypeDebug<'_, T>;
}

impl<T> OptionDebugExt<T> for Option<T> {
    fn debug_opaque(&self) -> OpaqueOptionDebug<'_, T> {
        OpaqueOptionDebug(self)
    }

    fn debug_type(&self) -> OptionTypeDebug<'_, T> {
        OptionTypeDebug(self)
    }
}
