use std::fmt::{Debug, Display, Formatter};

/// An owning type adaptor that enables a type's [`Debug`] implementation to be used for its
/// [`Display`] implementation.
///
/// This is the owned version of [`AsDebugDisplay`](crate::as_display::AsDebugDisplay).
///
/// # Examples
///
/// ```rust
/// use display_as_debug::as_display::{DebugDisplayed, DebugDisplay};
///
/// #[derive(Debug)]
/// struct DebugOnlyType {
///     field: i32,
/// }
///
/// let value = DebugOnlyType { field: 42 };
///
/// // Using to_display() method from DebugDisplay trait
/// let wrapped = value.to_display();
/// assert_eq!(wrapped.to_string(), "DebugOnlyType { field: 42 }");
///
/// // Or construct DebugDisplayed directly
/// let value2 = DebugOnlyType { field: 99 };
/// let wrapped2 = DebugDisplayed(value2);
/// assert_eq!(format!("{}", wrapped2), "DebugOnlyType { field: 99 }");
/// ```
///
/// # Trait Implementations
///
/// - **[`Display`]**: Uses the wrapped value's [`Debug`] implementation
/// - **[`Debug`]**: Forwards to the wrapped value's [`Debug`] implementation
/// - **[`Error`]**: Implements [`Error`] if the wrapped type implements both [`Debug`] and [`Error`]
///
/// # See Also
///
/// - [`AsDebugDisplay`](crate::as_display::AsDebugDisplay) - The borrowed variant
/// - [`DebugDisplay`](crate::as_display::DebugDisplay) - The trait providing convenience methods
///   [`to_display()`](crate::as_display::DebugDisplay::to_display).
#[derive(PartialEq, Eq)]
pub struct DebugDisplayed<T: Debug>(pub T);

impl<T: Debug> Display for DebugDisplayed<T> {
    /// Formats the owned value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: Debug> Debug for DebugDisplayed<T> {
    /// Formats the owned value using its debug implementation.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T: Debug + std::error::Error> std::error::Error for DebugDisplayed<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
