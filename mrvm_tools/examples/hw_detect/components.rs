use std::convert::TryInto;
use mrvm::board::Bus;
use mrvm_tools::metadata::{DeviceMetadata, StorageType, MemoryType};
use mrvm_tools::exceptions::AuxHwException;

pub struct BootROM {
    size: u32,
    storage: Vec<u32>
}

impl BootROM {
    pub fn new(storage: Vec<u32>, size: u32) -> Self {
        assert_eq!(size % 4, 0, "Provided BootROM size is unaligned");

        let size = size / 4;
        assert!(size > storage.len().try_into().unwrap(), "Provided test BootROM size is smaller than the provided storage's size");

        Self { size, storage }
    }
}

impl Bus for BootROM {
    fn name(&self) -> &'static str { "BootROM" }
    fn metadata(&self) -> [u32; 8] { DeviceMetadata::new(0xA1, self.size * 4, StorageType::Readonly.into(), 0, None).encode() }
    fn read(&mut self, addr: u32, _ex: &mut u16) -> u32 { self.storage.get((addr / 4) as usize).map(|b| *b).unwrap_or(0) }
    fn write(&mut self, _addr: u32, _word: u32, ex: &mut u16) { *ex = AuxHwException::MemoryNotWritable.into() }
    fn reset(&mut self) { }
}

pub struct RAM {
    size: u32,
    storage: Vec<u32>
}

impl RAM {
    pub fn new(size: u32) -> Self {
        assert_eq!(size % 4, 0, "Provided RAM size is unaligned");
        
        let size = size / 4;

        Self { size, storage: vec![0; size.try_into().unwrap()] }
    }
}

impl Bus for RAM {
    fn name(&self) -> &'static str { "RAM" }
    fn metadata(&self) -> [u32; 8] { DeviceMetadata::new(0xB1, self.size * 4, MemoryType::Volatile.into(), 0, None).encode() }
    fn read(&mut self, addr: u32, _ex: &mut u16) -> u32 { self.storage[(addr / 4) as usize] }
    fn write(&mut self, addr: u32, word: u32, _ex: &mut u16) { self.storage[(addr / 4) as usize] = word; }
    fn reset(&mut self) { }
}
