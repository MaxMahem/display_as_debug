#![doc = include_str!("../README.md")]

mod debug_as_display;
mod display_as_debug;

pub use debug_as_display::{AsDebugWrapper, DebugAsDisplay, DebugWrapper};
pub use display_as_debug::{AsDisplayWrapper, DisplayAsDebug, DisplayWrapper};
