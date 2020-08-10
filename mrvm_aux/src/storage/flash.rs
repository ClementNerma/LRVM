//! The flash memory component offers a simple non-flash storage.
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
    /// Returns an error message if the capacity is 0, not a multiple or 4 bytes or too large for the running CPU architecture.
    pub fn new(size: u32, hw_id: u64) -> Result<Self, &'static str> {
        if size == 0 {
            Err("Flash memory's size cannot be 0")
        } else if size % 4 != 0 {
            Err("Flash memory's size must be a multiple of 4 bytes")
        } else {
            Ok(Self {
                storage: vec![0; size.try_into().map_err(|_| "Flash memory size cannot exceed your CPU architecture's supported size")?],
                size: size / 4,
                hw_id
            })
        }
    }

    /// Create a new flash memory component from the provided storage
    /// Returns an error message if the capacity is too large for the running CPU architecture.
    pub fn from(storage: Vec<u32>, hw_id: u64) -> Result<Self, &'static str> {
        let size: u32 = storage.len().try_into().map_err(|_| "Flash memory's length cannot be larger than 2^32 words")?;

        Ok(Self {
            storage,
            size: size / 4,
            hw_id
        })
    }

    /// Create a new flash memory component from the provided storage and with a larger size than its storage
    /// Returns an error message in case of fail
    pub fn from_with_size(mut storage: Vec<u32>, size: u32, hw_id: u64) -> Result<Self, &'static str> {
        let _: u32 = storage.len().try_into().map_err(|_| "Flash memory's length cannot be larger than 2^32 words")?;
        let _: usize = size.try_into().map_err(|_| "Flash memory size cannot exceed your CPU architecture's supported size")?;

        if storage.len() > size as usize {
            return Err("Flash memory's size cannot be lower than its initial storage's size");
        }

        if size == 0 {
            return Err("Flash memory's size cannot be 0");
        }

        if size % 4 != 0 {
            return Err("Flash memory's size must be a multiple of 4 bytes");
        }

        let size = size / 4;

        storage.resize(size as usize, 0);

        Ok(Self {
            storage,
            size,
            hw_id
        })
    }

    /// Get the flash memory's size
    pub fn size(&self) -> u32 {
        self.size
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

    fn reset(&mut self) {
        self.storage = vec![0; self.storage.len()];
    }
}
