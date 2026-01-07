/// A [`Option<T>`] wrapper that implements [`Debug`], displaying type names instead of values.
///
/// Displays as `"None"` when the option is [`None`], or `"Some(typename)"` when [`Some`].
/// This avoids requiring `T: Debug` while still providing useful debug output.
pub struct OptionTypeDebug<'a, T>(pub &'a Option<T>);

use crate::as_debug::DisplayDebug;

impl<T> core::fmt::Debug for OptionTypeDebug<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            Some(_) => f.debug_tuple("Some").field(&core::any::type_name::<T>().as_debug()).finish(),
            None => f.write_str("None"),
        }
    }
}
