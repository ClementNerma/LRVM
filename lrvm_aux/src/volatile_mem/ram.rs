//! The RAM component offers a simple RAM that resets with the motherboard.
//! See [`RAM`] for more details.

use lrvm::board::Bus;
use lrvm_tools::metadata::{DeviceMetadata, MemoryType};

/// The RAM component offers a simple non-persistent storage.
/// When it receives a RESET request from the motherboard, all the storage is zeroed.
pub struct Ram {
    storage: Vec<u32>,
    size: u32,
    hw_id: u64,
}

impl Ram {
    /// Create a new RAM component
    /// Returns an error message if the capacity is 0, not a multiple or 4 bytes or too large for the running CPU architecture.
    pub fn new(size: u32, hw_id: u64) -> Result<Self, &'static str> {
        if size == 0 {
            Err("RAM's size cannot be 0")
        } else if size % 4 != 0 {
            Err("RAM's size must be a multiple of 4 bytes")
        } else {
            Ok(Self {
                storage: vec![
                    0;
                    size.try_into().map_err(|_| {
                        "RAM size cannot exceed your CPU architecture's supported size"
                    })?
                ],
                size: size / 4,
                hw_id,
            })
        }
    }

    /// Create a new RAM component from the provided storage
    /// Returns an error message if the capacity is too large for the running CPU architecture.
    pub fn from(storage: Vec<u32>, hw_id: u64) -> Result<Self, &'static str> {
        let size: u32 = storage
            .len()
            .try_into()
            .map_err(|_| "RAM's length cannot be larger than 2^32 words")?;

        Ok(Self {
            storage,
            size: size / 4,
            hw_id,
        })
    }

    /// Create a new RAM component from the provided storage and with a larger size than its storage
    /// Returns an error message in case of fail
    pub fn from_with_size(
        mut storage: Vec<u32>,
        size: u32,
        hw_id: u64,
    ) -> Result<Self, &'static str> {
        let _: u32 = storage
            .len()
            .try_into()
            .map_err(|_| "RAM's length cannot be larger than 2^32 words")?;
        let _: usize = size
            .try_into()
            .map_err(|_| "RAM size cannot exceed your CPU architecture's supported size")?;

        if storage.len() > size as usize {
            return Err("RAM's size cannot be lower than its initial storage's size");
        }

        if size == 0 {
            return Err("RAM's size cannot be 0");
        }

        if size % 4 != 0 {
            return Err("RAM's size must be a multiple of 4 bytes");
        }

        let size = size / 4;

        storage.resize(size as usize, 0);

        Ok(Self {
            storage,
            size,
            hw_id,
        })
    }

    /// Get the RAM's size
    pub fn size(&self) -> u32 {
        self.size
    }
}

impl Bus for Ram {
    fn name(&self) -> &'static str {
        "RAM"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(
            self.hw_id,
            self.size * 4,
            MemoryType::Ram.into(),
            None,
            None,
        )
        .encode()
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
