use core::fmt::{Debug, DebugTuple, Display};

use crate::as_debug::DisplayDebug;
use crate::opaque::Opaque;
use crate::type_name::{DisplayMode, TypeName};

/// Extension trait for [`DebugTuple`] providing convenient field formatting methods.
pub trait DebugTupleExt {
    /// Adds a field using the value's [`Display`] implementation instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::DebugTupleExt;
    /// use std::fmt;
    ///
    /// struct UserId(u32);
    ///
    /// impl fmt::Display for UserId {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         write!(f, "user_{}", self.0)
    ///     }
    /// }
    ///
    /// struct Session(UserId);
    ///
    /// impl fmt::Debug for Session {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         f.debug_tuple("Session")
    ///             .field_display(&self.0)
    ///             .finish()
    ///     }
    /// }
    ///
    /// let session = Session(UserId(42));
    ///
    /// assert_eq!(format!("{:?}", session), "Session(user_42)");
    /// ```
    fn field_display<T: Display>(&mut self, value: &T) -> &mut Self;

    /// Adds a field showing the type name instead of the value, using the specified [`DisplayMode`].
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::DebugTupleExt;
    /// use display_as_debug::type_name::{Full, Short};
    /// use std::fmt;
    ///
    /// struct Container<T>(T);
    ///
    /// impl<T> fmt::Debug for Container<T> {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         f.debug_tuple("Container")
    ///             .field_type::<T, Full>()
    ///             .field_type::<T, Short>()
    ///             .finish()
    ///     }
    /// }
    ///
    /// let c = Container(vec![1, 2, 3]);
    ///
    /// assert_eq!(
    ///     format!("{:?}", c),
    ///     "Container(alloc::vec::Vec<i32>, Vec<i32>)"
    /// );
    /// ```
    fn field_type<T, M: DisplayMode>(&mut self) -> &mut Self
    where
        TypeName<T, M>: Debug;

    /// Adds a field with an obscured value, showing `".."` for privacy.
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::DebugTupleExt;
    /// use std::fmt;
    ///
    /// struct Credentials(String);
    ///
    /// impl fmt::Debug for Credentials {
    ///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    ///         f.debug_tuple("Credentials")
    ///             .field_opaque()
    ///             .finish()
    ///     }
    /// }
    ///
    /// let creds = Credentials("secret123".into());
    ///
    /// assert_eq!(format!("{:?}", creds), "Credentials(..)");
    /// ```
    fn field_opaque(&mut self) -> &mut Self;
}

impl DebugTupleExt for DebugTuple<'_, '_> {
    fn field_display<T: Display>(&mut self, value: &T) -> &mut Self {
        self.field(&value.display_as_debug())
    }

    fn field_type<T, M: DisplayMode>(&mut self) -> &mut Self
    where
        TypeName<T, M>: Debug,
    {
        self.field(&TypeName::<T, M>::default())
    }

    fn field_opaque(&mut self) -> &mut Self {
        self.field(&Opaque)
    }
}
