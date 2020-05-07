use std::sync::{Arc, Mutex};
use super::Bus;

pub struct MappedMemory {
    aux: Vec<(Arc<Mutex<Box<dyn Bus>>>, &'static str, u32)>,
    mappings: Vec<Mapping>
}

#[derive(Debug, Clone, Copy)]
pub struct Mapping {
    pub addr: u32,
    pub size: u32,
    pub aux_id: usize
}

#[derive(Debug)]
pub enum MappingError {
    UnknownComponent,
    UnalignedAddress,
    UnalignedBusSize,
    NullBusSize,
    AddressOverlaps(Mapping)
}

#[derive(Debug)]
pub struct MappingStatus {
    pub start_addr: u32,
    pub end_addr: u32
}

#[derive(Debug)]
pub struct ContiguousMappingStatus {
    pub mapping: Result<MappingStatus, usize>,
    pub aux_mapping: Vec<AuxMappingStatus>
}

#[derive(Debug)]
pub struct AuxMappingStatus {
    pub aux_id: usize,
    pub aux_name: &'static str,
    pub aux_mapping: Result<MappingStatus, MappingError>
}

impl MappedMemory {
    pub fn new(aux: Vec<Arc<Mutex<Box<dyn Bus>>>>) -> Self {
        let aux = aux.into_iter().map(|aux| {
            let (aux_name, aux_size) = { let lock = aux.lock().unwrap(); (lock.name(), lock.size()) };
            assert!(aux_name.len() <= 32, "Auxiliary component's name must not exceed 32 bytes!");
            (aux, aux_name, aux_size)
        }).collect();

        Self {
            aux,
            mappings: Vec::new()
        }
    }

    pub fn map(&mut self, addr: u32, aux_id: usize) -> Result<MappingStatus, MappingError> {
        let aux_size = self.aux.get(aux_id).ok_or(MappingError::UnknownComponent)?.2;

        if aux_size == 0 {
            return Err(MappingError::NullBusSize);
        }

        if aux_size % 4 != 0 {
            return Err(MappingError::UnalignedBusSize);
        }

        match self.mappings.iter().find(|mapping| mapping.addr <= addr + aux_size - 1 && addr <= mapping.addr + mapping.size - 1) {
            Some(mapping) => {
                Err(MappingError::AddressOverlaps(mapping.clone()))
            },

            None => {
                self.mappings.push(Mapping { addr, size: aux_size, aux_id });
                Ok(MappingStatus {
                    start_addr: addr,
                    end_addr: addr + aux_size - 1
                })
            }
        }
    }

    pub fn map_contiguous(&mut self, addr: u32, aux_ids: impl AsRef<[usize]>) -> ContiguousMappingStatus {
        let mut aux_mapping = vec![];
        let mut failed = 0;
        let mut last_addr = addr;
        let mut max_addr = addr;
        
        for aux_id in aux_ids.as_ref() {
            let result = self.map(last_addr, *aux_id);

            match result {
                Ok(MappingStatus { start_addr: _, end_addr }) => {
                    if end_addr > max_addr {
                        max_addr = end_addr;
                    }

                    last_addr = end_addr + 1;
                },

                Err(_) => failed += 1
            }

            aux_mapping.push(AuxMappingStatus {
                aux_id: *aux_id,
                aux_name: self.name_of(*aux_id).unwrap(),
                aux_mapping: result
            });
        }

        ContiguousMappingStatus {
            mapping: if failed == 0 { Ok(MappingStatus { start_addr: addr, end_addr: max_addr }) } else { Err(failed) },
            aux_mapping
        }
    }

    pub fn name_of(&self, aux_id: usize) -> Option<&'static str> {
        self.aux.get(aux_id).map(|aux| aux.1)
    }

    pub fn size_of(&self, aux_id: usize) -> Option<u32> {
        self.aux.get(aux_id).map(|aux| aux.2)
    }

    pub fn read(&mut self, addr: u32) -> u32 {
        assert!(addr % 4 == 0, "Memory does not support reading from unaligned addresses");
        
        if let Some(mapping) = self.mappings.iter().find(|mapping| mapping.addr <= addr && addr <= mapping.addr + mapping.size - 1) {
            self.aux[mapping.aux_id].0.lock().unwrap().read(addr - mapping.addr)
        } else {
            if cfg!(debug_assertions) {
                eprintln!("Warning: tried to read non-mapped memory at address {:#010X}", addr);
            }

            0
        }
    }

    pub fn write(&mut self, addr: u32, word: u32) {
        assert!(addr % 4 == 0, "Memory does not support writing to unaligned addresses");
        
        if let Some(mapping) = self.mappings.iter().find(|mapping| mapping.addr <= addr && addr <= mapping.addr + mapping.size - 1) {
            self.aux[mapping.aux_id].0.lock().unwrap().write(addr - mapping.addr, word);
        } else if cfg!(debug_assertions) {
            eprintln!("Warning: tried to read non-mapped memory at address {:#010X}", addr);
        }
    }
}
