//! This crates contains a set of tools to deal more easily with MRVM, including a full-powered assembler.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]

pub mod asm;
pub mod lasm;

#[cfg(test)]
mod tests;
