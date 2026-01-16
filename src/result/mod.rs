mod opaque_result;
mod type_name_result;

pub(crate) const STR_ERR: &str = "Err";
pub(crate) const STR_OK: &str = "Ok";

pub use opaque_result::OpaqueResult;
pub use type_name_result::TypeNameResult;
