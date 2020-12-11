//! This crates contains a set of tools to deal more easily with MRVM, including a full-powered assembler.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]
#![allow(clippy::module_inception)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::result_unit_err)]

// Re-export the MRVM crate
pub use mrvm;

pub mod asm;
pub mod bytes;
pub mod debug;
pub mod exceptions;
pub mod lasm;
pub mod metadata;

#[cfg(test)]
mod tests;
