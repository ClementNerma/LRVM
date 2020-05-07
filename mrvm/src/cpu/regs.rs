use std::mem;

/// CPU registers
#[derive(Debug, Default)]
pub struct Registers {
    /// Arithmetic registers
    pub a: [u32; 8],

    /// Comparison registers
    pub c: [u32; 2],

    /// Address computation registers
    pub ac: [u32; 3],

    /// Routine registers
    pub rr: [u32; 8],

    /// Atomic Value Register
    pub avr: u32,

    /// Arithmetic Flags
    pub af: u32,

    /// Program Counter
    pub pc: u32,

    /// Supervisor Stack Pointer
    pub ssp: u32,

    /// Userland Stack Pointer
    pub usp: u32,

    /// Exception Type
    pub et: u32,

    /// Exception Return Address
    pub era: u32,

    /// Exception Vector
    pub ev: u32,

    /// Memory Translation Toggler
    pub mtt: u32,

    /// Page directory Address
    pub pda: u32,

    /// Supervisor Mode Toggler
    pub smt: u32
}

impl Registers {
    /// Create zero-initialied registers
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset all registers
    #[allow(unused_must_use)]
    pub fn reset(&mut self) {
        mem::replace(self, Self::default());
    }
}
