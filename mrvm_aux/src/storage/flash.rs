use std::convert::TryInto;
use mrvm::board::Bus;

pub struct FlashMem {
    storage: Vec<u32>,
    size: u32,
}

impl FlashMem {
    pub fn new(storage: Vec<u32>) -> Self {
        let size: u32 = storage.len().try_into().expect("Storage's length cannot be larger than 2^32 words");

        Self {
            storage,
            size
        }
    }

    pub fn with_size(storage: Vec<u32>, size: u32) -> Self {
        let size: u32 = size.try_into().expect("Storage's length cannot be larger than 2^32 words");

        Self {
            storage,
            size,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Bus for FlashMem {
    fn name(&self) -> &'static str {
        "Flash Memory"
    }

    fn size(&self) -> u32 {
        self.size * 4
    }

    fn read(&mut self, addr: u32) -> u32 {
        self.storage[addr as usize / 4]
    }

    fn write(&mut self, addr: u32, word: u32) {
        self.storage[addr as usize / 4] = word;
    }

    fn reset(&mut self) { }
}
