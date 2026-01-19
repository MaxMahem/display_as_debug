//! Result wrapper types for specialized [`Debug`](core::fmt::Debug) formatting.

mod opaque_result;
mod type_name_result;

const STR_ERR: &str = "Err";
const STR_OK: &str = "Ok";

pub use opaque_result::OpaqueResult;
pub use type_name_result::TypeNameResult;
