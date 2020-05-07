use std::convert::TryInto;
use mrvm::board::Bus;

pub struct VolatileMem {
    storage: Vec<u32>,
    size: u32,
}

impl VolatileMem {
    pub fn new(storage: Vec<u32>) -> Self {
        let size: u32 = storage.len().try_into().expect("Storage's length cannot be larger than 2^32 words");

        Self {
            storage,
            size
        }
    }

    pub fn with_size(mut storage: Vec<u32>, size: u32) -> Result<Self, ()> {
        let _: u32 = storage.len().try_into().expect("Storage's length cannot be larger than 2^32 words");
        let _: usize = size.try_into().expect("Volatile memory size cannot exceed your CPU architecture's supported size");

        if storage.len() > size as usize || size == 0 {
            return Err(())
        }

        let size = size / 4;

        storage.resize(size as usize, 0);

        Ok(Self {
            storage,
            size,
        })
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Bus for VolatileMem {
    fn name(&self) -> &'static str {
        "Volatile Memory"
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

    fn reset(&mut self) {
        self.storage = vec![0; self.storage.len()];
    }
}
