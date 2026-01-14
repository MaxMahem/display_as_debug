#![doc = include_str!("../README.md")]
#![no_std]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]

#[cfg(doc)]
use core::fmt::{Debug, Display};

mod debug_struct_ext;
mod debug_tuple_ext;
mod opaque;

/// Traits and types for treating a [`Display`] implementation as a [`Debug`] implementation.
pub mod as_debug;

/// Traits and types for treating a [`Debug`] implementation as a [`Display`] implementation.
pub mod as_display;

/// Traits and types for treating implementing [`Debug`] for [`Option`].
pub mod option;

/// Traits and types for treating implementing [`Debug`] for [`Result`].
pub mod result;

/// Traits and types for displaying type names.
pub mod type_name;

pub use debug_struct_ext::DebugStructExt;
pub use debug_tuple_ext::DebugTupleExt;
pub use opaque::Opaque;
