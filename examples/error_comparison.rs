//! Comparison example showing the difference between using `display_to_debug()` and not using it
//! when returning errors from main.
//! Run with `cargo run --example error_comparison` to see the difference

use display_as_debug::as_debug::DisplayAsDebug;
use std::fmt;

#[derive(Debug)]
struct SimpleError(String);

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for SimpleError {}

fn approach_without_wrapper() -> Result<(), Box<dyn std::error::Error>> {
    Err(Box::new(SimpleError(
        "Application Error (Code 500): Failed to connect to database".to_string(),
    )))
}

fn approach_with_display_to_debug() -> Result<(), Box<dyn std::error::Error>> {
    let error =
        SimpleError("Application Error (Code 500): Failed to connect to database".to_string());
    Err(Box::new(error.display_to_debug()))
}

fn main() {
    println!("=== Without display_to_debug() ===");
    println!("The error with default Debug formatting prints as:");
    if let Err(e) = approach_without_wrapper() {
        println!("Error: {:?}", e);
        println!("(Shows the Debug representation with struct name)\n");
    }

    println!("=== With display_to_debug() ===");
    println!("The error wrapped with display_to_debug() prints as:");
    if let Err(e) = approach_with_display_to_debug() {
        println!("Error: {:?}", e);
        println!("(Shows only the Display representation - cleaner!)\n");
    }

    println!("This is the difference display_as_debug makes when returning errors from main!");
}
