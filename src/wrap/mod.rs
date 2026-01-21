//! Wrapper types for [`Debug`](core::fmt::Debug) and [`Display`](core::fmt::Display) formatting.

mod debug_as_display;
mod display_as_debug;
mod opaque;
mod option;
mod result;
mod type_name;

pub use crate::types::{DisplayMode, Full, Short};
pub use debug_as_display::DebugAsDisplay;
pub use display_as_debug::DisplayAsDebug;
pub use opaque::Opaque;
pub use option::{OpaqueOption, OpaqueOptionMarker, TypeNameOption};
pub use result::{OpaqueResult, OpaqueResultMarker, TypeNameResult};
pub use type_name::TypeName;
