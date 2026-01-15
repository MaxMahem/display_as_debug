use core::fmt::{self, Debug, Display};

/// A test type that prints differently for [`Display`] and [`Debug`].
///
/// Shows `Display(value)` for [`Display`] and `Debug(value)` for [`Debug`], where value is the
/// debug representation of the inner value.
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::types::TestValue;
/// assert_eq!(format!("{}", TestValue::TEST), r#"Display("test")"#);
/// assert_eq!(format!("{:?}", TestValue::TEST), r#"Debug("test")"#);
/// assert_eq!(format!("{}", TestValue(42)), "Display(42)");
/// assert_eq!(format!("{:?}", TestValue(42)), "Debug(42)");
/// ```
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TestValue<T = ()>(pub T);

impl TestValue<()> {
    /// The default value for [`TestValue`].
    pub const DEFAULT: Self = Self(());
}

impl TestValue<&'static str> {
    /// A test value with a string for use in examples.
    pub const TEST: Self = Self("test");
}

impl<T: Debug> Debug for TestValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug({:?})", self.0)
    }
}

impl<T: Debug> Display for TestValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Display({:?})", self.0)
    }
}
