//! Wrapper types for [`Debug`](core::fmt::Debug) and [`Display`](core::fmt::Display) formatting.

mod debug_as_display;
mod display_as_debug;
mod option;
mod result;

pub use debug_as_display::DebugAsDisplay;
pub use display_as_debug::DisplayAsDebug;
pub use option::{OpaqueOption, TypeNameOption};
pub use result::{OpaqueResult, TypeNameResult};
