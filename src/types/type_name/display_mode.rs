/// Sealed marker trait for type name display modes.
#[sealed::sealed]
pub trait DisplayMode {
    /// Returns the type name for the given type according to this display mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use display_as_debug::types::{DisplayMode, Full, Short};
    /// let full_name = Full::type_name::<Vec<i32>>();
    /// assert_eq!(full_name, "alloc::vec::Vec<i32>");
    ///
    /// let short_name = Short::type_name::<Vec<i32>>();
    /// assert_eq!(short_name, "Vec<i32>");
    /// ```
    fn type_name<T: ?Sized>() -> &'static str;
}

/// [`DisplayMode`] that shows the full type name from [`std::any::type_name`](core::any::type_name).
#[derive(Copy, Clone, Debug)]
pub struct Full;

#[sealed::sealed]
impl DisplayMode for Full {
    fn type_name<T: ?Sized>() -> &'static str {
        core::any::type_name::<T>()
    }
}

/// [`DisplayMode`] that shows only the short type name (last component after `::` splitting).
#[derive(Copy, Clone, Debug)]
pub struct Short;

#[sealed::sealed]
impl DisplayMode for Short {
    fn type_name<T: ?Sized>() -> &'static str {
        let type_name = core::any::type_name::<T>();
        type_name.rsplit("::").next().unwrap_or(type_name)
    }
}
