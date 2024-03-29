//! LRVM (Lightweight Rust Virtual Machine) is a lightweight virtual machine runtime written in Rust.
//! It supports basic motherboard emulation as well as traditional features like memory mapping (MMIO) and a MMU.
//! Auxiliary components can be connected to the motherboard through a [`Bus`] interface.

#![forbid(unsafe_code)]
#![forbid(unused_must_use)]
#![allow(clippy::module_inception)]

pub mod board;
pub mod cpu;
pub mod mem;
pub mod mmu;
