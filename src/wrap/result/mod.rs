//! Result wrapper types for specialized [`Debug`](core::fmt::Debug) formatting.

mod opaque;
mod type_name;

const STR_ERR: &str = "Err";
const STR_OK: &str = "Ok";

pub use opaque::{OpaqueResult, OpaqueResultMarker};
pub use type_name::TypeNameResult;
