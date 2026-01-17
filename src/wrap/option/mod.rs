//! Option wrapper types for specialized [`Debug`](core::fmt::Debug) formatting.

mod opaque_option;
mod type_name_option;

const STR_NONE: &str = "None";
const STR_SOME: &str = "Some";

pub use opaque_option::OpaqueOption;
pub use type_name_option::TypeNameOption;
