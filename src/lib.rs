#![no_std]
#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

#[cfg(doc)]
use core::fmt::{Debug, Display};

/// Traits and types for treating a [`Display`] implementation as a [`Debug`] implementation.
pub mod as_debug;

/// Traits and types for treating a [`Debug`] implementation as a [`Display`] implementation.
pub mod as_display;

/// Traits and types for treating implementing [`Debug`] for [`Option`].
pub mod option;

/// Traits and types for treating implementing [`Debug`] for [`Result`].
pub mod result;
