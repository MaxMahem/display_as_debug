use core::fmt::{Debug, Display, Formatter};
use core::marker::PhantomData;

/// Sealed marker trait for type name display modes.
#[sealed::sealed]
pub trait DisplayMode {}

/// [`DisplayMode`] that shows the full type name from [`std::any::type_name`](core::any::type_name).
#[derive(Copy, Clone, Debug)]
pub struct Full;

#[sealed::sealed]
impl DisplayMode for Full {}

/// [`DisplayMode`] that shows only the short type name (last component after `::` splitting).
#[derive(Copy, Clone, Debug)]
pub struct Short;

#[sealed::sealed]
impl DisplayMode for Short {}

/// A marker type that formats as a type name when used in [`Debug`] or [`Display`] contexts.
///
/// This type should not be used directly, but rather through the associated constants:
/// - [`TypeName::FULL`] for the fully name as returned by [`std::any::type_name`](core::any::type_name)
/// - [`TypeName::SHORT`] for the shortened name (last component only)
///
/// Note: rust does not gurantee the stability or format of any of these values, they should only
/// be used for debugging purposes.
///
/// # Examples
///
/// ```
/// # use display_as_debug::types::TypeName;
/// assert_eq!(format!("{:?}", TypeName::<Vec<i32>>::FULL), "alloc::vec::Vec<i32>");
/// assert_eq!(format!("{}", TypeName::<Vec<i32>>::FULL), "alloc::vec::Vec<i32>");
/// assert_eq!(format!("{:?}", TypeName::<Vec<i32>>::SHORT), "Vec<i32>");
/// assert_eq!(format!("{}", TypeName::<Vec<i32>>::SHORT), "Vec<i32>");
/// ```
#[derive(Copy, Clone)]
pub struct TypeName<T: ?Sized, M: DisplayMode = Full>(PhantomData<fn() -> T>, PhantomData<M>);

impl<T: ?Sized, M: DisplayMode> TypeName<T, M> {
    /// A constant instance showing the full type name from [`std::any::type_name`](core::any::type_name).
    ///
    /// # Examples
    ///
    /// ```
    /// # use display_as_debug::types::TypeName;
    /// assert_eq!(format!("{:?}", TypeName::<Vec<i32>>::FULL), "alloc::vec::Vec<i32>");
    /// assert_eq!(format!("{}", TypeName::<Vec<i32>>::FULL), "alloc::vec::Vec<i32>");
    /// ```
    pub const FULL: TypeName<T, Full> = TypeName(PhantomData, PhantomData);

    /// A constant instance showing only the short type name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use display_as_debug::types::TypeName;
    /// assert_eq!(format!("{:?}", TypeName::<Vec<i32>>::SHORT), "Vec<i32>");
    /// assert_eq!(format!("{}", TypeName::<Vec<i32>>::SHORT), "Vec<i32>");
    /// ```
    pub const SHORT: TypeName<T, Short> = TypeName(PhantomData, PhantomData);
}

impl<T: ?Sized, M: DisplayMode> Default for TypeName<T, M> {
    fn default() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<T: ?Sized> Debug for TypeName<T, Full> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::any::type_name::<T>())
    }
}

impl<T: ?Sized> Display for TypeName<T, Full> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl<T: ?Sized> Debug for TypeName<T, Short> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let type_name = core::any::type_name::<T>();
        let short_name = type_name.rsplit("::").next().unwrap_or(type_name);
        f.write_str(short_name)
    }
}

impl<T: ?Sized> Display for TypeName<T, Short> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self, f)
    }
}
