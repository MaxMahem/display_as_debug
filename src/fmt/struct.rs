use core::fmt::{Debug, DebugStruct, Display};

use crate::types::{DisplayMode, OPAQUE, TypeName};
use crate::wrap::DisplayAsDebug;

/// Extension trait for [`DebugStruct`] providing convenient field formatting methods.
#[sealed::sealed]
pub trait DebugStructExt {
    /// Adds a field using the value's [`Display`] implementation instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugStructExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct Data<T> { value: T }
    ///
    /// impl<T: Display> Debug for Data<T> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_struct("Data").field_display("value", &self.value).finish()
    ///     }
    /// }
    ///
    /// let data = Data { value: TestValue::DEFAULT };
    ///
    /// assert_eq!(format!("{:?}", data), "Data { value: Display(()) }");
    /// ```
    fn field_display(&mut self, name: &str, value: &dyn Display) -> &mut Self;

    /// Adds a field showing the type name instead of the value, using the specified [`DisplayMode`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugStructExt;
    /// use display_as_debug::types::{Full, Short};
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct Container<T> { data: T }
    ///
    /// impl<T> Debug for Container<T> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_struct("Container")
    ///             .field_type::<T, Full>("full")
    ///             .field_type::<T, Short>("short")
    ///             .finish()
    ///     }
    /// }
    ///
    /// let c = Container { data: vec![1, 2, 3] };
    ///
    /// assert_eq!(format!("{:?}", c), "Container { full: alloc::vec::Vec<i32>, short: Vec<i32> }");
    /// ```
    fn field_type<T, M: DisplayMode>(&mut self, name: &str) -> &mut Self
    where
        TypeName<T, M>: Debug;

    /// Adds a field with an obscured value, showing `".."` for privacy.
    ///
    /// # Example
    ///
    /// ```rust
    /// use display_as_debug::fmt::DebugStructExt;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct Credentials<T> { password: T }
    ///
    /// impl<T> Debug for Credentials<T> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_struct("Credentials").field_opaque("password").finish()
    ///     }
    /// }
    ///
    /// let creds = Credentials { password: "secret" };
    ///
    /// assert_eq!(format!("{:?}", creds), "Credentials { password: .. }");
    /// ```
    fn field_opaque(&mut self, name: &str) -> &mut Self;
}

#[sealed::sealed]
impl DebugStructExt for DebugStruct<'_, '_> {
    fn field_display(&mut self, name: &str, value: &dyn Display) -> &mut Self {
        self.field(name, &DisplayAsDebug(value))
    }

    fn field_type<T, M: DisplayMode>(&mut self, name: &str) -> &mut Self
    where
        TypeName<T, M>: Debug,
    {
        self.field(name, &TypeName::empty::<T, M>())
    }

    fn field_opaque(&mut self, name: &str) -> &mut Self {
        self.field(name, &OPAQUE)
    }
}
