use core::fmt::{Debug, DebugTuple, Display};

use crate::types::{DisplayMode, OPAQUE, TypeName};
use crate::wrap::DisplayAsDebug;

/// Extension trait for [`DebugTuple`] providing convenient field formatting methods.
#[sealed::sealed]
pub trait DebugTupleExt {
    /// Adds a field using the value's [`Display`] implementation instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::fmt::DebugTupleExt;
    /// use display_as_debug::types::TestValue;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct Container<T>(T);
    ///
    /// impl<T: Display> Debug for Container<T> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_tuple("Container").field_display(&self.0).finish()
    ///     }
    /// }
    ///
    /// let container = Container(TestValue::TEST);
    ///
    /// assert_eq!(format!("{:?}", container), r#"Container(Display("test"))"#);
    /// ```
    fn field_display(&mut self, value: &dyn Display) -> &mut Self;

    /// Adds a field showing the type name instead of the value, using the specified [`DisplayMode`].
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::fmt::DebugTupleExt;
    /// use display_as_debug::types::{Full, Short};
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct Container<T>(T);
    ///
    /// impl<T> Debug for Container<T> {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_tuple("Container")
    ///             .field_type::<T, Full>()
    ///             .field_type::<T, Short>()
    ///             .finish()
    ///     }
    /// }
    ///
    /// let container = Container(vec![1, 2, 3]);
    ///
    /// assert_eq!(format!("{:?}", container), "Container(alloc::vec::Vec<i32>, Vec<i32>)");
    /// ```
    fn field_type<T, M: DisplayMode>(&mut self) -> &mut Self
    where
        TypeName<T, M>: Debug;

    /// Adds a field with an obscured value, showing `".."` for privacy.
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::fmt::DebugTupleExt;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct Credentials(&'static str);
    ///
    /// impl Debug for Credentials {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_tuple("Credentials").field_opaque().finish()
    ///     }
    /// }
    ///
    /// assert_eq!(format!("{:?}", Credentials("secret")), "Credentials(..)");
    /// ```
    fn field_opaque(&mut self) -> &mut Self;
}

#[sealed::sealed]
impl DebugTupleExt for DebugTuple<'_, '_> {
    fn field_display(&mut self, value: &dyn Display) -> &mut Self {
        self.field(&DisplayAsDebug(value))
    }

    fn field_type<T, M: DisplayMode>(&mut self) -> &mut Self
    where
        TypeName<T, M>: Debug,
    {
        self.field(&TypeName::empty::<T, M>())
    }

    fn field_opaque(&mut self) -> &mut Self {
        self.field(&OPAQUE)
    }
}
