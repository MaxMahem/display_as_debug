#![doc = include_str!("../README.md")]

#[warn(clippy::pedantic)]
#[warn(clippy::cargo)]
pub mod as_debug;
pub mod as_display;

pub mod option;
pub mod result;
