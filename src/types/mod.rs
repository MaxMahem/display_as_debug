//! Types that implement [`Debug`](core::fmt::Debug) for specialized formatting.

mod opaque;
mod test_value;
mod type_name;

pub use crate::wrap::Opaque;
pub use opaque::{OpaqueList, OpaqueMap, OpaqueSet};
pub use test_value::TestValue;
pub use type_name::{
    DisplayMode, Full, Short, TypeName, TypeNameList, TypeNameMap, TypeNameMarker, TypeNameOption, TypeNameSet,
};

/// An obscure marker value that formats as `..` when used in [`Debug`](core::fmt::Debug) or [`Display`](core::fmt::Display).
///
/// # Examples
///
/// ```rust
/// # use display_as_debug::types::OPAQUE;
/// assert_eq!(format!("{:?}", OPAQUE), "..", "Debug format should be opaque");
/// assert_eq!(format!("{}", OPAQUE), "..", "Display format should be opaque");
/// ```
pub const OPAQUE: Opaque<()> = Opaque::DEFAULT;
