#![doc = include_str!("../README.md")]
#![no_std]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(missing_docs, missing_debug_implementations)]
#![forbid(unsafe_code)]

#[cfg(doc)]
use core::fmt::{Debug, Display};

/// Extension traits for [`Debug`] builder types.
pub mod fmt;

/// Helper types that implement [`Debug`] and/or [`Display`] for specialized formatting.
pub mod types;

/// Wrapper types for [`Debug`] and [`Display`] format conversions.
pub mod wrap;
