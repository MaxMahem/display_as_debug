//! Option wrapper types for specialized [`Debug`](core::fmt::Debug) formatting.

mod opaque;
mod type_name;

const STR_NONE: &str = "None";
const STR_SOME: &str = "Some";

pub use opaque::OpaqueOption;
pub use type_name::TypeNameOption;
