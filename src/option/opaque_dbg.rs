use core::fmt::Debug;

/// A [`Option<T>`] wrapper that implements [`Debug`] with opaque Some values.
///
/// Displays as `Some(..)` when the option is [`Some`], or `None` when [`None`].
/// This provides privacy for Some values while still indicating the option's state.
pub struct OpaqueOptionDebug<'a, T>(pub &'a Option<T>);

impl<T> Debug for OpaqueOptionDebug<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            Some(_) => f.write_str("Some(..)"),
            None => f.write_str("None"),
        }
    }
}
