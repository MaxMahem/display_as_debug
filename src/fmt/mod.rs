//! Extension traits for `fmt::Debug*` builder types.

mod fold_mut;

mod list;
mod map;
mod set;
mod r#struct;
mod tuple;

pub use list::DebugListExt;
pub use map::DebugMapExt;
pub use set::DebugSetExt;
pub use r#struct::DebugStructExt;
pub use tuple::DebugTupleExt;
