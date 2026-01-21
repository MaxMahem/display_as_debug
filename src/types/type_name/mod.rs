//! Types using [`TypeName`] for specialized debug formatting.

mod display_mode;
mod list;
mod option;
mod result;
mod set;
#[allow(clippy::module_inception)]
mod type_name;

pub use crate::wrap::TypeName;
pub use crate::wrap::TypeNameOption;
pub use crate::wrap::TypeNameResult;
pub use display_mode::{DisplayMode, Full, Short};
pub use list::TypeNameList;
pub use set::{TypeNameMap, TypeNameSet};
pub use type_name::TypeNameMarker;
