use super::{OpaqueOptionDebug, OptionTypeDebug};
use crate::type_name::DisplayMode;

/// Extension trait providing convenience methods for debugging [`Option`] values.
pub trait OptionDebugExt<T> {
    /// Returns a wrapper that implements [`Debug`] with opaque [`Some`] values.
    ///
    /// Displays as `Some(..)` when the option is [`Some`], or `None` when [`None`].
    /// This provides privacy for [`Some`] values while still indicating the option's state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::option::OptionDebugExt;
    /// assert_eq!(format!("{:?}", Some("sensitive data").debug_opaque()), "Some(..)");
    /// assert_eq!(format!("{:?}", None::<&str>.debug_opaque()), "None");
    /// ```
    fn debug_opaque(&self) -> OpaqueOptionDebug<'_, T>;

    /// Returns a wrapper that implements [`Debug`], displaying type names instead of values.
    ///
    /// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
    /// This avoids requiring `T: Debug` while still providing useful debug output.
    ///
    /// The display mode ([`Full`](crate::type_name::Full) or [`Short`](crate::type_name::Short)) must be
    /// specified as a generic argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::option::OptionDebugExt;
    /// use display_as_debug::type_name::{Full, Short};
    ///
    /// assert_eq!(format!("{:?}", Some(vec![1]).debug_type::<Full>()), "Some(alloc::vec::Vec<i32>)");
    /// assert_eq!(format!("{:?}", Some(vec![1]).debug_type::<Short>()), "Some(Vec<i32>)");
    /// assert_eq!(format!("{:?}", None::<i32>.debug_type::<Full>()), "None");
    /// ```
    fn debug_type<M: DisplayMode>(&self) -> OptionTypeDebug<'_, T, M>;
}

impl<T> OptionDebugExt<T> for Option<T> {
    fn debug_opaque(&self) -> OpaqueOptionDebug<'_, T> {
        OpaqueOptionDebug(self)
    }

    fn debug_type<M: DisplayMode>(&self) -> OptionTypeDebug<'_, T, M> {
        OptionTypeDebug::new::<M>(self)
    }
}
