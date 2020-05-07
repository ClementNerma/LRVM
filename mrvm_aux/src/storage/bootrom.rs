use std::convert::TryInto;
use mrvm::board::Bus;

pub struct BootROM {
    storage: Vec<u32>,
    len: u32,
    size: u32,
    pub panic_on_invalid: bool
}

impl BootROM {
    pub fn new(storage: Vec<u32>) -> Self {
        let len: u32 = storage.len().try_into().expect("Storage's length cannot be larger than 2^32 words");

        Self {
            storage,
            len,
            size: len,
            panic_on_invalid: true
        }
    }

    pub fn with_size(storage: Vec<u32>, size: u32) -> Self {
        let len: u32 = storage.len().try_into().expect("Storage's length cannot be larger than 2^32 words");

        assert!(size % 4 == 0, "BootROM's size cannot be unaligned");

        let size = size / 4;

        assert!(size >= len, "BootROM's size cannot be smaller than the storage's actual size");

        Self {
            storage,
            len,
            size,
            panic_on_invalid: false
        }
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn size(&self) -> u32 {
        self.size
    }

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
