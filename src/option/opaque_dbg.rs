use std::fmt::Debug;

/// A [`Option<T>`] wrapper that implements [`Debug`] with opaque Some values.
///
/// Displays as `Some(..)` when the option is [`Some`], or `None` when [`None`].
/// This provides privacy for Some values while still indicating the option's state.
pub struct OpaqueOptionDbg<'a, T>(pub &'a Option<T>);

impl<T> Debug for OpaqueOptionDbg<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(_) => f.write_str("Some(..)"),
            None => f.write_str("None"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_option_some_opaque() {
        let opt = Some(42);
        let debug_output = format!("{:?}", OpaqueOptionDbg(&opt));
        assert_eq!(debug_output, "Some(..)");
    }

    #[test]
    fn debug_option_none() {
        let opt: Option<i32> = None;
        let debug_output = format!("{:?}", OpaqueOptionDbg(&opt));
        assert_eq!(debug_output, "None");
    }
}
