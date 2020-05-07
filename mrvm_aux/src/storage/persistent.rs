//! The persistent memory component offers a simple non-volatile storage that persists on the disk.
//! See [`PersistentMem`] for more details.

use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{Result as IOResult, Read, Write, Seek, SeekFrom};
use std::convert::TryInto;
use mrvm::board::Bus;

/// The persistent memory component contains a read-only or writable, persistent storage that does not reset with the motherboard.
/// It uses a real file to store its data and is perfect for storing data that persists after the VM is destroyed.
pub struct PersistentMem {
    handler: File,
    size: u32,
    real_size: u32,
    writable: bool,
    pub panic_on_invalid: bool
}

impl PersistentMem {
    /// (Internal) open the provided path file in read-only or writable mode
    fn open(path: impl AsRef<Path>, writable: bool) -> IOResult<Self> {
        let handler = OpenOptions::new().read(true).write(writable).open(path)?;

        let unaligned_real_size: u32 = handler.metadata()?.len()
            .try_into().expect("Cannot open files larger than 4 GB due to 32-bit addressing mode");

        let real_size = (unaligned_real_size / 4) * 4;

        if real_size != unaligned_real_size {
            println!("Warning: opened unaligned file as aligned (rounded size to nearest lower multiple of 4 bytes)");
        }

        let _: usize = real_size.try_into().expect("Persistent memory size must not exceed your CPU architecture (e.g. 32-bit size)");

        Ok(Self {
            size: real_size,
            real_size,
            handler,
            writable,
            panic_on_invalid: false
        })
    }

    /// Create a new writable persistent memory component
    pub fn writable(path: impl AsRef<Path>) -> IOResult<Self> {
        Self::open(path, true)
    }

    /// Create a new writable persistent memory component with a custom size
    pub fn writable_with_size(path: impl AsRef<Path>, size: u32) -> IOResult<Self> {
        let mut mem = Self::writable(path)?;

        if mem.real_size > size {
            mem.size = size;
        } else if mem.real_size < size {
            mem.handler.set_len(size.into())?;
        }

        Ok(mem)
    }

    /// Create a new read-only persistent memory component
    pub fn readonly(path: impl AsRef<Path>) -> IOResult<Self> {
        Self::open(path, false)
    }

    /// Create a new writable persistent memory component with a custom size
    pub fn readonly_with_size(path: impl AsRef<Path>, size: u32) -> IOResult<Self> {
        let mut mem = Self::readonly(path)?;
        mem.size = size;
        Ok(mem)
    }

    /// Set if the component must make the program panic on invalid access (writing attemps on read-only storage)
    pub fn set_panic_on_invalid(mut self, value: bool) -> Self {
        self.panic_on_invalid = value;
        self
    }
}

impl Bus for PersistentMem {
    fn name(&self) -> &'static str {
        "Persistent Memory"
    }

    fn size(&self) -> u32 {
        self.size
    }

    fn read(&mut self, addr: u32) -> u32 {
        if addr >= self.size {
            if self.panic_on_invalid {
                panic!("ERROR: Attempted to read outside persistent memory");
            } else {
                eprintln!("Warning: Attempted to read outside persistent memory");
                0
            }
        } else if addr >= self.real_size {
            0
        } else {
            let mut buffer = [0; 4];
            
            self.handler.seek(SeekFrom::Start(addr.into())).unwrap();
            self.handler.read_exact(&mut buffer).unwrap();

            u32::from_be_bytes(buffer)
        }
    }

    fn write(&mut self, addr: u32, word: u32) {
        if !self.writable {
            if self.panic_on_invalid {
                panic!("ERROR: Attempted to write readonly persistent memory");
            } else {
                eprintln!("Warning: Attempted to write readonly persistent memory");
            }
        } else if addr >= self.size {
            if self.panic_on_invalid {
                panic!("ERROR: Attempted to write outside persistent memory");
            } else {
                eprintln!("Warning: Attempted to write outside persistent memory");
            }
        } else if addr < self.real_size {
            self.handler.seek(SeekFrom::Start(addr.into())).unwrap();
            self.handler.write(&word.to_be_bytes()).unwrap();
        }
    }

    fn reset(&mut self) { }
}
