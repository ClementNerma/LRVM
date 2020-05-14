//! This crates contains a set of tools to deal more easily with MRVM, including a full-powered assembler.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]

#![allow(clippy::module_inception)]

pub mod asm;
pub mod lasm;
pub mod bytes;
pub mod metadata;
pub mod exceptions;
pub mod debug;

#[cfg(test)]
mod tests;
