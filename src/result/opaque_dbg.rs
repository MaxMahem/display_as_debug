use std::fmt::Debug;

/// A [`Result<T, E>`] wrapper that implements [`Debug`] with opaque Ok values.
///
/// Displays as `Ok(..)` when the result is [`Ok`], or `Err(error_value)` when [`Err`].
/// This provides privacy for Ok values while fully debugging errors.
pub struct OpaqueResultDbg<'a, T, E>(pub &'a Result<T, E>);

impl<T, E: Debug> Debug for OpaqueResultDbg<'_, T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Ok(_) => f.write_str("Ok(...)"),
            Err(e) => f.debug_tuple("Err").field(e).finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_result_ok_opaque() {
        let result: Result<i32, String> = Ok(42);
        let debug_output = format!("{:?}", OpaqueResultDbg(&result));
        assert_eq!(debug_output, "Ok(...)");
    }

    #[test]
    fn debug_result_err_full() {
        let result: Result<i32, String> = Err("error message".to_string());
        let debug_output = format!("{:?}", OpaqueResultDbg(&result));
        assert_eq!(debug_output, "Err(\"error message\")");
    }
}
