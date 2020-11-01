//! This crate contains a set of auxiliary components to connect to the virtual motherboard.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]
#![allow(clippy::len_without_is_empty)]

pub mod display;
pub mod keyboard;
pub mod storage;
pub mod time;
pub mod volatile_mem;

#[cfg(test)]
mod tests;
