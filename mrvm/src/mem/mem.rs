use crate::board::HardwareBridge;
use super::{Mapping, MappingRange, MappingError, ContiguousMappingStatus, AuxMappingStatus};

/// Mapped memory
pub struct MappedMemory {
    /// Hardware bridge
    bridge: HardwareBridge,
    /// Components mappings
    mappings: Vec<Mapping>
}

impl MappedMemory {
    /// Create a new mapped memory using an hardware bridge
    pub fn new(hwb: HardwareBridge) -> Self {
        Self { bridge: hwb, mappings: vec![] }
    }

    /// Map an auxiliary component from a specific address.
    /// The end address will be determined through the component's [`Bus::size`] method.
    pub fn map(&mut self, addr: u32, aux_id: usize) -> Result<MappingRange, MappingError> {
        self.internal_map(addr, None, aux_id)
    }

    /// Map an auxiliary component to a specific address range
    /// NOTE: The address range cannot be higher than the component's [`Bus::size`] value.
    pub fn map_abs(&mut self, addr: u32, addr_end: u32, aux_id: usize) -> Result<MappingRange, MappingError> {
        self.internal_map(addr, Some(addr_end), aux_id)
    }

    /// Map a list of components contiguously
    pub fn map_contiguous(&mut self, addr: u32, aux_ids: impl AsRef<[usize]>) -> ContiguousMappingStatus {
        let mut aux_mapping = vec![];
        let mut failed = vec![];
        let mut last_addr = addr;
        let mut max_addr = addr;
        
        for aux_id in aux_ids.as_ref() {
            // Try to map the component
            let result = self.map(last_addr, *aux_id);

            match result {
                Ok(MappingRange { start_addr: _, end_addr }) => {
                    if end_addr > max_addr {
                        max_addr = end_addr;
                    }

                    last_addr = end_addr + 1;
                },

                Err(_) => failed.push(*aux_id)
            }

            aux_mapping.push(AuxMappingStatus {
                aux_id: *aux_id,
                aux_hw_id: self.bridge.hw_id_of(*aux_id).unwrap(),
                aux_name: self.bridge.name_of(*aux_id).unwrap().clone(),
                aux_mapping: result
            });
        }

        ContiguousMappingStatus {
            mapping: if failed.is_empty() { Ok(MappingRange { start_addr: addr, end_addr: max_addr }) } else { Err(failed) },
            aux_mapping
        }
    }

    /// Read an arbitrary address in the mapped memory.
    /// The related component will be contacted through its [`Bus`] if mounted at this address.
    /// If no component is mount at this address, the `0x00000000` value will be returned.
    /// If the value of `ex` is not zero when this function returns, a hardware exception occurred with the exception code and data in it.
    pub fn read(&mut self, addr: u32, ex: &mut u16) -> u32 {
        assert!(addr % 4 == 0, "Memory does not support reading from unaligned addresses");
        
        if let Some(mapping) = self.mappings.iter().find(|mapping| mapping.addr <= addr && addr <= mapping.end_addr()) {
            self.bridge.read(mapping.aux_id, addr - mapping.addr, ex).unwrap()
        } else {
            if cfg!(debug_assertions) {
                eprintln!("Warning: tried to read non-mapped memory at address {:#010X}", addr);
            }

            0
        }
    }

    /// Write an arbitrary address in the mapped memory.
    /// The related component will be contacted through its [`Bus`] if mounted at this address.
    /// If no component is mount at this address, the write will simply be ignored.
    /// If the value of `ex` is not zero when this function returns, a hardware exception occurred with the exception code and data in it.
    pub fn write(&mut self, addr: u32, word: u32, ex: &mut u16) {
        assert!(addr % 4 == 0, "Memory does not support writing to unaligned addresses");
        
        if let Some(mapping) = self.mappings.iter().find(|mapping| mapping.addr <= addr && addr <= mapping.end_addr()) {
            self.bridge.write(mapping.aux_id, addr - mapping.addr, word, ex).unwrap()
        } else if cfg!(debug_assertions) {
            eprintln!("Warning: tried to write non-mapped memory at address {:#010X}", addr);
        }
    }

    /// Get the mapping of a given component
    pub fn get_mapping(&self, aux_id: usize) -> Option<&Mapping> {
        self.mappings.iter().find(|mapping| mapping.aux_id == aux_id)
    }

    /// (Internal) map an auxiliary component to the memory
    fn internal_map(&mut self, addr: u32, addr_end: Option<u32>, aux_id: usize) -> Result<MappingRange, MappingError> {
        let aux_size = self.bridge.size_of(aux_id).ok_or(MappingError::UnknownComponent)?;
        
        let addr_end = addr_end.unwrap_or(addr + aux_size - 4);

        if addr % 4 != 0 {
            return Err(MappingError::UnalignedStartAddress);
        }

        if aux_size == 0 {
            return Err(MappingError::NullBusSize);
        }

        if aux_size % 4 != 0 {
            return Err(MappingError::UnalignedBusSize);
        }

        if addr_end % 4 != 0 {
            return Err(MappingError::UnalignedEndAddress);
        }

        if addr == addr_end + 4 || addr > addr_end {
            return Err(MappingError::NullOrNegAddressRange);
        }

        if self.mappings.iter().any(|mapping| mapping.aux_id == aux_id) {
            return Err(MappingError::AlreadyMapped);
        }

        // Check if a component is already mapped on this address range
        match self.mappings.iter().find(|mapping| mapping.addr <= addr_end && addr <= mapping.end_addr()) {
            Some(mapping) => {
                Err(MappingError::AddressOverlaps(mapping.clone()))
            },

            None => {
                self.mappings.push(Mapping {
                    aux_id,
                    aux_hw_id: self.bridge.hw_id_of(aux_id).expect("Internal error: failed to get HW ID of component after mapping validation"),
                    addr,
                    size: aux_size
                });

                Ok(MappingRange {
                    start_addr: addr,
                    end_addr: addr + aux_size - 1
                })
            }
        }
    }
}
