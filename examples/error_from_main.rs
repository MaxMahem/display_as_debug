//! Example showing how to use `display_as_debug` to pass nicely formatted error messages
//! out the top of main.
//!
//! When main returns a `Result<(), E>`, Rust prints the error using its `Debug` implementation.
//! By using `err_debug_to_display()`, we can ensure the nice `Debug` formatting is used for
//! display instead of the default `Debug` representation.

use display_as_debug::DisplayWrapper;
use std::fmt;

/// A custom error type with a nice Display implementation
#[derive(Debug)]
struct AppError {
    message: String,
    code: i32,
}

impl AppError {
    fn new(message: impl Into<String>, code: i32) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Application Error (Code {}): {}",
            self.code, self.message
        )
    }
}

impl std::error::Error for AppError {}

/// Simulates an operation that might fail
fn risky_operation() -> Result<(), AppError> {
    Err(AppError::new("Failed to connect to database", 500))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // DisplayWrapper uses the Display implementation for Debug output.
    // Since the field is now public, we can construct it directly.
    risky_operation().map_err(DisplayWrapper)?;

    println!("Success!");
    Ok(())
}
