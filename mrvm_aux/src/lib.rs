#![forbid(unsafe_code)]
#![deny(unused_must_use)]

#[cfg(test)]
mod tests;

pub mod storage;
pub mod memory;
pub mod display;
pub mod keyboard;