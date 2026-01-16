use super::{OpaqueOption, TypeNameOption};
use crate::types::DisplayMode;

/// Extension trait providing convenience methods for debugging [`Option`] values.
#[sealed::sealed]
pub trait DebugOption<T> {
    /// Returns a wrapper that implements [`Debug`] with opaque [`Some`] values.
    ///
    /// Displays as `Some(..)` when the option is [`Some`], or `None` when [`None`].
    /// This provides privacy for [`Some`] values while still indicating the option's state.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::option::DebugOption;
    /// assert_eq!(format!("{:?}", Some("sensitive data").debug_opaque()), "Some(..)");
    /// assert_eq!(format!("{:?}", None::<&str>.debug_opaque()), "None");
    /// ```
    fn debug_opaque(&self) -> OpaqueOption<'_, T>;

    /// Returns a wrapper that implements [`Debug`], displaying type names instead of values.
    ///
    /// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
    /// This avoids requiring `T: Debug` while still providing useful debug output.
    ///
    /// The display mode ([`Full`](crate::types::Full) or [`Short`](crate::types::Short)) must be
    /// specified as a generic argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use display_as_debug::option::DebugOption;
    /// use display_as_debug::types::{Full, Short};
    ///
    /// assert_eq!(format!("{:?}", Some(vec![1]).debug_type_name::<Full>()), "Some(alloc::vec::Vec<i32>)");
    /// assert_eq!(format!("{:?}", Some(vec![1]).debug_type_name::<Short>()), "Some(Vec<i32>)");
    /// assert_eq!(format!("{:?}", None::<i32>.debug_type_name::<Full>()), "None");
    /// ```
    fn debug_type_name<M: DisplayMode>(&self) -> TypeNameOption<'_, T, M>;
}

#[sealed::sealed]
impl<T> DebugOption<T> for Option<T> {
    fn debug_opaque(&self) -> OpaqueOption<'_, T> {
        OpaqueOption(self)
    }

    fn debug_type_name<M: DisplayMode>(&self) -> TypeNameOption<'_, T, M> {
        TypeNameOption::new::<M>(self)
    }
}
