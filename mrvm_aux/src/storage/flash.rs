//! The flash memory component offers a simple non-volatile storage.
//! See [`FlashMem`] for more details.

use std::convert::TryInto;
use mrvm::board::Bus;
use mrvm_tools::metadata::{DeviceMetadata, StorageType};

/// The flash memory component contains a writable, persistent storage that does not reset with the motherboard.
/// It is though reset when the VM is destroyed.
pub struct FlashMem {
    storage: Vec<u32>,
    size: u32,
    hw_id: u64
}

impl FlashMem {
    /// Create a new flash memory component
    pub fn new(size: u32, hw_id: u64) -> Result<Self, ()> {
        if size == 0 || size % 4 != 0 {
            Err(())
        } else {
            Ok(Self {
                storage: vec![0; size.try_into().expect("Volatile memory size cannot exceed your CPU architecture's supported size")],
                size,
                hw_id
            })
        }
    }

    /// Create a new flash memory component from an existing storage
    pub fn from(storage: Vec<u32>, hw_id: u64) -> Self {
        let size: u32 = storage.len().try_into().expect("Storage's length cannot be larger than 2^32 words");

        Self {
            storage,
            size,
            hw_id
        }
    }

    /// Create a new flash memory component from an existing storage and a larger size.
    /// The storage's extended part will be zeroed.
    pub fn from_with_size(storage: Vec<u32>, size: u32, hw_id: u64) -> Self {
        Self {
            storage,
            size,
            hw_id
        }
    }
}

impl Bus for FlashMem {
    fn name(&self) -> &'static str {
        "Flash Memory"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(
            self.hw_id,
            self.size * 4,
            StorageType::Flash.into(),
            None,
            None
        ).encode()
    }

    fn read(&mut self, addr: u32, _ex: &mut u16) -> u32 {
        self.storage[addr as usize / 4]
    }

    fn write(&mut self, addr: u32, word: u32, _ex: &mut u16) {
        self.storage[addr as usize / 4] = word;
    }

    fn reset(&mut self) { }
}
