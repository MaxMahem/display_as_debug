//! Types using [`Opaque`](crate::wrap::Opaque) for specialized debug formatting.

mod list;
mod set;

pub use list::OpaqueList;
pub use set::{OpaqueMap, OpaqueSet};
