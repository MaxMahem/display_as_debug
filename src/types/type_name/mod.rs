//! Types using [`TypeName`] for specialized debug formatting.

mod list;
mod set;
#[allow(clippy::module_inception)]
mod type_name;

pub use list::TypeNameList;
pub use set::{TypeNameMap, TypeNameSet};
pub use type_name::{DisplayMode, Full, Short, TypeName};
