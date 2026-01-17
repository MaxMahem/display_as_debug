//! Types using [`Opaque`] for specialized debug formatting.

mod list;
#[allow(clippy::module_inception)]
mod opaque;
mod set;

pub use list::OpaqueList;
pub use opaque::{OPAQUE, Opaque};
pub use set::{OpaqueMap, OpaqueSet};
