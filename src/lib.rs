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

/// Traits and types for treating a [`Display`] implementation as a [`Debug`] implementation.
pub mod debug;

/// Traits and types for treating a [`Debug`] implementation as a [`Display`] implementation.
pub mod display;

/// Traits and types for implementing [`Debug`] for [`Option`].
pub mod option;

/// Traits and types for implementing [`Debug`] for [`Result`].
pub mod result;
