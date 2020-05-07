//! The BootROM component offers a simple read-only storage.
//! See [`BootROM`] for more details.

use std::convert::TryInto;
use mrvm::board::Bus;

/// The BootROM component contains a read-only storage that is initialized during its creation.
/// All write requests are invalid but read requests are invalid.
/// The BootROM's size may be larger than its initialization storage. In such case, reading from the unitialized part will return `0x0000000`.
pub struct BootROM {
    storage: Vec<u32>,
    len: u32,
    size: u32,
    pub panic_on_invalid: bool
}

impl BootROM {
    /// Create a new BootROM component
    /// Returns an error message if the capacity is too large for the running CPU architecture.
    pub fn new(storage: Vec<u32>) -> Result<Self, &'static str> {
        let len: u32 = storage.len().try_into().map_err(|_| "Storage's length cannot be larger than 2^32 words")?;

        Ok(Self {
            storage,
            len,
            size: len,
            panic_on_invalid: false
        })
    }

    /// Create a new BootROM component larger than its storage
    /// Returns an error message in case of fail
    pub fn with_size(storage: Vec<u32>, size: u32) -> Result<Self, &'static str> {
        let len: u32 = storage.len().try_into().map_err(|_| "Storage's length cannot be larger than 2^32 words")?;

        if storage.len() > size as usize {
            return Err("Flash memory's size cannot be lower than its initial storage's size");
        }

        if size == 0 {
            return Err("Flash memory's size cannot be 0");
        }

        if size % 4 != 0 {
            return Err("Flash memory's size must be a multiple of 4 bytes");
        }

        Ok(Self {
            storage,
            len,
            size: size / 4,
            panic_on_invalid: false
        })
    }

    /// Get the BootROM's real storage's length
    pub fn len(&self) -> u32 {
        self.len
    }

    /// Get the BootROM's size
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Set if the component must make the program panic on invalid access (writing attempts)
    pub fn set_panic_on_invalid(mut self, value: bool) -> Self {
        self.panic_on_invalid = value;
        self
    }
}

impl Bus for BootROM {
    fn name(&self) -> &'static str {
        "BootROM"
    }

    fn size(&self) -> u32 {
        self.size * 4
    }

    fn read(&mut self, addr: u32) -> u32 {
        let addr = addr / 4;

        if addr < self.len {
            self.storage[addr as usize]
        } else if addr < self.size {
            0
        } else {
            if self.panic_on_invalid && cfg!(debug_assertions) {
                panic!("Error: attempted to read outside the BootROM");
            } else {
                eprintln!("Warning: attempted to read outside the BootROM");
                0
            }
        }
    }

    fn write(&mut self, _addr: u32, _word: u32) {
        if self.panic_on_invalid && cfg!(debug_assertions) {
            panic!("Error: attempted to write the BootROM");
        } else {
            eprintln!("Warning: attempted to write the BootROM");
        }
    }

    fn reset(&mut self) { }
}
