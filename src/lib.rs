#![doc = include_str!("../README.md")]

#[warn(clippy::pedantic)]
#[warn(clippy::cargo)]
mod debug_as_display;
mod display_as_debug;

pub mod option;
pub mod result;

pub use debug_as_display::*;
pub use display_as_debug::*;
