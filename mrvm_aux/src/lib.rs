//! This crate contains a set of auxiliary components to connect to the virtual motherboard.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]

pub mod storage;
pub mod memory;
pub mod display;
pub mod keyboard;

#[cfg(test)]
mod tests;
