//! MRVM (Minimal Rust Virtual Machine) is a lightweight virtual machine runtime written in Rust. 
//! It supports basic motherboard emulation as well as traditional features like memory mapping (MMIO) and a MMU.
//! Auxiliary components can be connected to the motherboard through a [`Bus`] interface.

#![forbid(unsafe_code)]
#![deny(unused_must_use)]

pub mod board;
pub mod mem;
pub mod cpu;
pub mod mmu;

#[cfg(test)]
mod tests;
