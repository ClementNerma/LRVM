//! This crate contains a set of auxiliary components to connect to the virtual motherboard.

#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![allow(clippy::len_without_is_empty)]

// Re-export the LRVM crate
pub use lrvm;

pub mod debug;
pub mod display;
pub mod keyboard;
pub mod storage;
pub mod time;
pub mod volatile_mem;

#[cfg(test)]
mod tests;
