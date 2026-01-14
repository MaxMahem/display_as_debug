use core::fmt::{Debug, DebugStruct, Display};

use crate::as_debug::DisplayDebug;
use crate::opaque::Opaque;
use crate::type_name::{DisplayMode, TypeName};

/// Extension trait for [`DebugStruct`] providing convenient field formatting methods.
pub trait DebugStructExt {
    /// Adds a field using the value's [`Display`] implementation instead of [`Debug`].
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::DebugStructExt;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct UserId(u32);
    ///
    /// impl Display for UserId {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         write!(f, "user_{}", self.0)
    ///     }
    /// }
    ///
    /// struct Session { user_id: UserId }
    ///
    /// impl Debug for Session {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_struct("Session")
    ///             .field_display("user_id", &self.user_id)
    ///             .finish()
    ///     }
    /// }
    ///
    /// let session = Session { user_id: UserId(42) };
    ///
    /// assert_eq!(format!("{:?}", session), "Session { user_id: user_42 }");
    /// ```
    fn field_display<T: Display>(&mut self, name: &str, value: &T) -> &mut Self;

    /// Adds a field showing the type name instead of the value, using the specified [`DisplayMode`].
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::DebugStructExt;
    /// use display_as_debug::type_name::{Full, Short};
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
    /// assert_eq!(
    ///     format!("{:?}", c),
    ///     "Container { full: alloc::vec::Vec<i32>, short: Vec<i32> }"
    /// );
    /// ```
    fn field_type<T, M: DisplayMode>(&mut self, name: &str) -> &mut Self
    where
        TypeName<T, M>: Debug;

    /// Adds a field with an obscured value, showing `".."` for privacy.
    ///
    /// # Example
    ///
    /// ```
    /// use display_as_debug::DebugStructExt;
    /// use std::fmt::{Debug, Display, Formatter};
    ///
    /// struct Credentials { password: String }
    ///
    /// impl Debug for Credentials {
    ///     fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    ///         f.debug_struct("Credentials")
    ///             .field_opaque("password")
    ///             .finish()
    ///     }
    /// }
    ///
    /// let creds = Credentials { password: "secret123".into() };
    ///
    /// assert_eq!(format!("{:?}", creds), "Credentials { password: .. }");
    /// ```
    fn field_opaque(&mut self, name: &str) -> &mut Self;
}

impl DebugStructExt for DebugStruct<'_, '_> {
    fn field_display<T: Display>(&mut self, name: &str, value: &T) -> &mut Self {
        self.field(name, &value.display_as_debug())
    }

    fn field_type<T, M: DisplayMode>(&mut self, name: &str) -> &mut Self
    where
        TypeName<T, M>: Debug,
    {
        self.field(name, &TypeName::<T, M>::default())
    }

    fn field_opaque(&mut self, name: &str) -> &mut Self {
        self.field(name, &Opaque)
    }
}
