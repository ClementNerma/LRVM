use super::Bus;
use std::cell::RefCell;
use std::rc::Rc;

/// Auxiliary component's bus and internal data cache
struct AuxWithCache {
    /// Auxiliary component's [`Bus`] interface
    shared_bus: Rc<RefCell<Box<dyn Bus>>>,
    /// Data cache for this auxiliary component
    cache: AuxCache,
}

/// Cache of an auxiliary component
#[derive(Debug, Clone)]
pub struct AuxCache {
    /// Auxiliary component's ID
    pub id: usize,
    /// Auxiliary component's hardware identifier
    pub hw_id: u64,
    /// Auxiliary component's generic name
    pub name: String,
    /// Auxiliary component's metadata
    pub metadata: [u32; 8],
    /// Auxiliary component's size
    pub size: u32,
}

impl AuxWithCache {
    /// Create the cache from an auxiliary component
    pub fn create_from_aux(id: usize, shared_bus: Rc<RefCell<Box<dyn Bus>>>) -> Self {
        let bus = shared_bus.borrow();

        let mut name = bus.name().to_string();

        while name.bytes().count() > 32 {
            name.pop();
        }

        let metadata = bus.metadata();
        let hw_id = ((metadata[0] as u64) << 32) + metadata[1] as u64;
        let size = metadata[2];

        std::mem::drop(bus);

        AuxWithCache {
            shared_bus,
            cache: AuxCache {
                id,
                hw_id,
                name,
                metadata,
                size,
            },
        }
    }
}

/// The hardware bridge is an internal component that allows internal components to communicate with auxiliary ones.  
/// It contains a small cache which allows to quickly fetch specific data about components.  
/// Multiple hardware bridges can co-exit on the motheboard, but their cache is not shared.
pub struct HardwareBridge {
    aux: Vec<AuxWithCache>,
}

impl HardwareBridge {
    pub fn new(aux: impl IntoIterator<Item = Rc<RefCell<Box<dyn Bus>>>>) -> Self {
        Self {
            aux: aux
                .into_iter()
                .enumerate()
                .map(|(id, shared_bus)| {
                    assert!(
                        id < u32::MAX as usize,
                        "Hardware bridge cannot handle more than 2^32 components!"
                    );

                    AuxWithCache::create_from_aux(id, shared_bus)
                })
                .collect(),
        }
    }

    /// Count the number of connected compoents
    pub fn count(&self) -> usize {
        self.aux.len()
    }

    /// Get the data cache of an auxiliary component from its ID
    pub fn cache_of(&self, aux_id: usize) -> Option<&AuxCache> {
        self.aux.get(aux_id).map(|entry| &entry.cache)
    }

    /// Get the name of an auxiliary component from its ID
    pub fn name_of(&self, aux_id: usize) -> Option<&String> {
        self.cache_of(aux_id).map(|cache| &cache.name)
    }

    /// Get the metadata of an axuiliary component from its ID
    pub fn metadata_of(&self, aux_id: usize) -> Option<[u32; 8]> {
        self.cache_of(aux_id).map(|cache| cache.metadata)
    }

    /// Get the unique identifier of an auxiliary component from its ID
    pub fn hw_id_of(&self, aux_id: usize) -> Option<u64> {
        self.cache_of(aux_id).map(|cache| cache.hw_id)
    }

    /// Get the size of an auxiliary component from its ID
    pub fn size_of(&self, aux_id: usize) -> Option<u32> {
        self.cache_of(aux_id).map(|cache| cache.size)
    }

    /// Send a READ signal to a component.  
    /// If the `ex` reference contains a non-zero value when this function returns, the component raised an exception
    /// with the provided code and data.
    pub fn read(&mut self, aux_id: usize, addr: u32, ex: &mut u16) -> Option<u32> {
        assert!(
            addr % 4 == 0,
            "Hardware bridge does not support reading from unaligned addresses"
        );

        self.aux
            .get(aux_id)
            .map(|aux| aux.shared_bus.borrow_mut().read(addr, ex))
    }

    /// Send a WRITE signal to a component.  
    /// If the `ex` reference contains a non-zero value when this function returns, the component raised an exception
    /// with the provided code and data.
    pub fn write(&mut self, aux_id: usize, addr: u32, word: u32, ex: &mut u16) -> Option<()> {
        assert!(
            addr % 4 == 0,
            "Hardware bridge does not support writing to unaligned addresses"
        );

        self.aux
            .get(aux_id)
            .map(|aux| aux.shared_bus.borrow_mut().write(addr, word, ex))
    }

    /// Send a RESET signal to a component
    pub fn reset(&mut self, aux_id: usize) -> Option<()> {
        self.aux
            .get(aux_id)
            .map(|aux| aux.shared_bus.borrow_mut().reset())
    }
}
