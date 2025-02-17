//! The persistent memory component offers a simple non-volatile storage that persists on the disk.
//! See [`FileBackedMem`] for more details.

use std::{
    cmp::Ordering,
    fs::{File, OpenOptions},
    io::{Read, Result as IOResult, Seek, SeekFrom, Write},
    path::Path,
};

use lrvm::board::Bus;
use lrvm_tools::{
    exceptions::AuxHwException,
    metadata::{DeviceMetadata, StorageType},
};

/// The persistent memory component contains a read-only or writable, persistent storage that does not reset with the motherboard.
///
/// It uses a real file to store its data and is perfect for storing data that persists after the VM is destroyed.
pub struct FileBackedMem {
    handler: File,
    size: u32,
    real_size: u32,
    writable: bool,
    hw_id: u64,
}

impl FileBackedMem {
    /// (Internal) open the provided path file in read-only or writable mode
    fn open(path: impl AsRef<Path>, writable: bool, hw_id: u64) -> IOResult<Self> {
        let handler = OpenOptions::new().read(true).write(writable).open(path)?;

        let unaligned_real_size: u32 = handler
            .metadata()?
            .len()
            .try_into()
            .expect("Cannot open files larger than 4 GB due to 32-bit addressing mode");

        let real_size = (unaligned_real_size / 4) * 4;

        if real_size != unaligned_real_size {
            println!("Warning: opened unaligned file as aligned (rounded size to nearest lower multiple of 4 bytes)");
        }

        let _: usize = real_size.try_into().expect(
            "Persistent memory size must not exceed your CPU architecture (e.g. 32-bit size)",
        );

        Ok(Self {
            size: real_size,
            real_size,
            handler,
            writable,
            hw_id,
        })
    }

    /// Create a new writable persistent memory component
    pub fn writable(path: impl AsRef<Path>, hw_id: u64) -> IOResult<Self> {
        Self::open(path, true, hw_id)
    }

    /// Create a new writable persistent memory component with a custom size
    pub fn writable_with_size(path: impl AsRef<Path>, size: u32, hw_id: u64) -> IOResult<Self> {
        let mut mem = Self::writable(path, hw_id)?;

        match mem.real_size.cmp(&size) {
            Ordering::Greater => mem.size = size,
            Ordering::Less => mem.handler.set_len(size.into())?,
            Ordering::Equal => {}
        }

        Ok(mem)
    }

    /// Create a new read-only persistent memory component
    pub fn readonly(path: impl AsRef<Path>, hw_id: u64) -> IOResult<Self> {
        Self::open(path, false, hw_id)
    }

    /// Create a new writable persistent memory component with a custom size
    pub fn readonly_with_size(path: impl AsRef<Path>, size: u32, hw_id: u64) -> IOResult<Self> {
        let mut mem = Self::readonly(path, hw_id)?;
        mem.size = size;
        Ok(mem)
    }
}

impl Bus for FileBackedMem {
    fn name(&self) -> &'static str {
        "Persistent Memory"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(
            self.hw_id,
            self.size * 4,
            StorageType::Persistent.into(),
            None,
            None,
        )
        .encode()
    }

    fn read(&mut self, addr: u32, ex: &mut u16) -> u32 {
        if addr >= self.real_size {
            return 0;
        }

        let mut buffer = [0; 4];

        if self.handler.seek(SeekFrom::Start(addr.into())).is_err() {
            *ex = AuxHwException::GenericPhysicalReadError.into();
            return 0;
        }

        if self.handler.read_exact(&mut buffer).is_err() {
            *ex = AuxHwException::GenericPhysicalReadError.into();
            return 0;
        }

        u32::from_be_bytes(buffer)
    }

    fn write(&mut self, addr: u32, word: u32, ex: &mut u16) {
        if !self.writable {
            *ex = AuxHwException::MemoryNotWritable.into();
        } else if addr < self.real_size {
            self.handler.seek(SeekFrom::Start(addr.into())).unwrap();
            self.handler.write_all(&word.to_be_bytes()).unwrap();
        }
    }

    fn reset(&mut self) {}
}
