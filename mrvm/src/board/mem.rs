use std::sync::{Arc, Mutex};
use super::Bus;

/// Mapped memory
pub struct MappedMemory {
    /// Auxiliary components candidates
    aux: Vec<MemAuxComponent>,
    /// Components mappings
    mappings: Vec<Mapping>
}

/// Auxiliary component candidate to mapping
pub struct MemAuxComponent {
    /// Auxiliary component's [`Bus`] interface
    bus: Arc<Mutex<Box<dyn Bus>>>,
    /// Auxiliary component's generic name
    name: String,
    /// Auxiliary component's size
    size: u32
}

/// A single component mapping.
/// End address can be computed as `mapping.addr + mapping.size - 1`
#[derive(Debug, Clone, Copy)]
pub struct Mapping {
    /// Mapped component's ID
    pub aux_id: usize,
    /// Mapping start address
    pub addr: u32,
    /// Mapping length
    pub size: u32
}

/// Error that occurred during mapping
#[derive(Debug)]
pub enum MappingError {
    UnknownComponent,
    UnalignedStartAddress,
    UnalignedBusSize,
    UnalignedEndAddress,
    NullOrNegAddressRange,
    AlreadyMapped,
    NullBusSize,
    AddressOverlaps(Mapping)
}

/// Mapping range
#[derive(Debug)]
pub struct MappingRange {
    /// Start address
    pub start_addr: u32,
    /// End address
    pub end_addr: u32
}

/// Status of a continguous mapping
#[derive(Debug)]
pub struct ContiguousMappingStatus {
    /// Range of the mapping in case of success, or ID of the faulty components if the mapping failed
    pub mapping: Result<MappingRange, Vec<usize>>,
    /// List of auxiliary components mapping (succeeded or failed)
    pub aux_mapping: Vec<AuxMappingStatus>
}

/// Mapping status of a single auxiliary component
#[derive(Debug)]
pub struct AuxMappingStatus {
    /// Auxiliary component's ID
    pub aux_id: usize,
    /// Auxiliary component's generic name
    pub aux_name: String,
    /// Mapping result
    pub aux_mapping: Result<MappingRange, MappingError>
}

impl MappedMemory {
    /// Create a new mapped memory from a list of auxiliary components' [`Bus`] interface
    pub fn new(aux_list: Vec<Arc<Mutex<Box<dyn Bus>>>>) -> Self {
        assert!(aux_list.len() <= std::u32::MAX as usize, "Cannot connect more than 2^32 components!");

        Self {
            aux: aux_list.into_iter().map(|shared_bus| {
                let bus = shared_bus.lock().unwrap();

                let aux_name = bus.name().chars().take(32).collect::<String>();
                let aux_size = bus.metadata()[2];

                std::mem::drop(bus);

                assert!(aux_name.len() <= 32, "Auxiliary component's name must not exceed 32 bytes!");
                MemAuxComponent { bus: shared_bus, name: aux_name, size: aux_size }
            }).collect(),

            mappings: vec![]
        }
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
                aux_name: self.name_of(*aux_id).unwrap().clone(),
                aux_mapping: result
            });
        }

        ContiguousMappingStatus {
            mapping: if failed.len() == 0 { Ok(MappingRange { start_addr: addr, end_addr: max_addr }) } else { Err(failed) },
            aux_mapping
        }
    }

    /// Get the name of an auxiliary component from its ID
    pub fn name_of(&self, aux_id: usize) -> Option<&String> {
        self.aux.get(aux_id).map(|aux| &aux.name)
    }

    /// Get the size of an auxiliary component from its ID
    pub fn size_of(&self, aux_id: usize) -> Option<u32> {
        self.aux.get(aux_id).map(|aux| aux.size)
    }

    /// Read an arbitrary address in the mapped memory.
    /// The related component will be contacted through its [`Bus`] if mounted at this address.
    /// If no component is mount at this address, the `0x00000000` value will be returned.
    /// If the value of `ex` is not zero when this function returns, a hardware exception occurred with the exception code and data in it.
    pub fn read(&mut self, addr: u32, ex: &mut u16) -> u32 {
        assert!(addr % 4 == 0, "Memory does not support reading from unaligned addresses");
        
        if let Some(mapping) = self.mappings.iter().find(|mapping| mapping.addr <= addr && addr <= mapping.addr + mapping.size - 1) {
            self.aux[mapping.aux_id].bus.lock().unwrap().read(addr - mapping.addr, ex)
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
        
        if let Some(mapping) = self.mappings.iter().find(|mapping| mapping.addr <= addr && addr <= mapping.addr + mapping.size - 1) {
            self.aux[mapping.aux_id].bus.lock().unwrap().write(addr - mapping.addr, word, ex);
        } else if cfg!(debug_assertions) {
            eprintln!("Warning: tried to read non-mapped memory at address {:#010X}", addr);
        }
    }

    /// (Internal) map an auxiliary component to the memory
    fn internal_map(&mut self, addr: u32, addr_end: Option<u32>, aux_id: usize) -> Result<MappingRange, MappingError> {
        let aux_size = self.aux.get(aux_id).ok_or(MappingError::UnknownComponent)?.size;
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

        if self.mappings.iter().find(|mapping| mapping.aux_id == aux_id).is_some() {
            return Err(MappingError::AlreadyMapped);
        }

        // Check if a component is already mapped on this address range
        match self.mappings.iter().find(|mapping| mapping.addr <= addr_end && addr <= mapping.addr + mapping.size - 1) {
            Some(mapping) => {
                Err(MappingError::AddressOverlaps(mapping.clone()))
            },

            None => {
                self.mappings.push(Mapping { addr, size: aux_size, aux_id });
                Ok(MappingRange {
                    start_addr: addr,
                    end_addr: addr + aux_size - 1
                })
            }
        }
    }
}
