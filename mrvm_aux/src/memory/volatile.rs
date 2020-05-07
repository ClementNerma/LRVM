//! The volatile memory component offers a simple volatile memory that resets with the motherboard.
//! See [`VolatileMem`] for more details.

use std::convert::TryInto;
use mrvm::board::Bus;

/// The volatile memor component offers a simple non-persistent storage.
/// When it receives a RESET request from the motherboard, all the storage is zeroed.
pub struct VolatileMem {
    storage: Vec<u32>,
    size: u32,
}

impl VolatileMem {
    /// Create a new volatile memory component
    pub fn new(size: u32) -> Result<Self, ()> {
        if size == 0 || size % 4 != 0 {
            Err(())
        } else {
            Ok(Self {
                storage: vec![0; size.try_into().expect("Volatile memory size cannot exceed your CPU architecture's supported size")],
                size
            })
        }
    }

    /// Create a new volatile memory component from the provided storage
    pub fn from(storage: Vec<u32>) -> Self {
        let size: u32 = storage.len().try_into().expect("Storage's length cannot be larger than 2^32 words");

        Self {
            storage,
            size
        }
    }

    /// Create a new volatile memory component from the provided storage and with a larger size than its storage
    pub fn from_with_size(mut storage: Vec<u32>, size: u32) -> Result<Self, ()> {
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

    /// Get the volatile memory's size
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
