//! The virtual machine is centered around a single motherboard which contains the CPU as well as a memory,
//! which allows to connect components through memory mapping (MMIO) using the [`mem`] function.
//!
//! The motherboard can also emulate a reset button through the [`reset`] function which propagates the even through all connected [`Bus`].

use std::sync::{Arc, Mutex, MutexGuard};
use crate::cpu::CPU;
use super::{Bus, MappedMemory};

/// Virtual motherboard
pub struct MotherBoard {
    /// Auxiliary components connected to the motherboard
    aux: Vec<Arc<Mutex<Box<dyn Bus>>>>,
    /// Memory mappings
    mem: Arc<Mutex<MappedMemory>>,
    /// Central Processing Unit (CPU)
    cpu: CPU
}

impl MotherBoard {
    /// Create a new motherboard with a set of components
    pub fn new(components: impl IntoIterator<Item=Box<dyn Bus>>) -> Self {
        let mut aux = vec![];

        for component in components.into_iter() {
            let component = Arc::new(Mutex::new(component));
            aux.push(Arc::clone(&component));
        }

        // Instanciate the memory
        let mem = Arc::new(Mutex::new(MappedMemory::new(aux.clone())));

        Self {
            aux,
            mem: Arc::clone(&mem),
            cpu: CPU::new(Arc::clone(&mem))
        }
    }

    /// Perform operations on memory through a handler, example:
    ///
    /// ```
    /// let motherboard = MotherBoard::new(vec![ /* component1 */ ]);
    /// motherboard.map(|mem| mem.map(0x10000000, 0)); // Map first component (ID 0) to address 0x10000000
    /// ```
    pub fn map<T>(&mut self, mut mapper: impl FnMut(MutexGuard<MappedMemory>) -> T) -> T {
        mapper(self.mem.lock().unwrap())
    }

    /// Get a mutable reference to the CPU (required to make the CPU advance)
    pub fn cpu(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    /// Emulate a hard reset button on the motherboard.
    /// All components will receive a reset signal through their [`Bus`] interface.
    /// The CPU will also be reset, before every other component. Check [`CPU::reset`] for more informations.
    pub fn reset(&mut self) {
        self.cpu.reset();

        for aux in self.aux.iter() {
            aux.lock().unwrap().reset();
        }
    }
}
