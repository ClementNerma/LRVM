//! The virtual machine is centered around a single motherboard which contains the CPU as well as a memory,
//! which allows to connect components through memory mapping (MMIO) using the [`mem`] function.
//!
//! The motherboard can also emulate a reset button through the [`reset`] function which propagates the even through all connected [`Bus`].

use super::{Bus, HardwareBridge};
use crate::cpu::Cpu;
use crate::mem::MappedMemory;
use std::cell::RefCell;
use std::rc::Rc;

/// Virtual motherboard
pub struct MotherBoard {
    /// Auxiliary components connected to the motherboard
    aux: Vec<Rc<RefCell<Box<dyn Bus>>>>,
    /// Central Processing Unit (CPU)
    cpu: Cpu,
}

impl MotherBoard {
    /// Create a new motherboard with a set of components
    pub fn new(components: impl IntoIterator<Item = Box<dyn Bus>>) -> Self {
        let aux = components
            .into_iter()
            .map(|cp| Rc::new(RefCell::new(cp)))
            .collect::<Vec<_>>();

        assert!(
            aux.len() <= std::u32::MAX as usize,
            "Cannot connect more than 2^32 components!"
        );

        // Instanciate the memory
        let mem = MappedMemory::new(HardwareBridge::new(aux.clone()));

        Self {
            cpu: Cpu::new(HardwareBridge::new(aux.clone()), mem),
            aux,
        }
    }

    /// Perform operations on memory through a handler, example:
    ///
    /// ```no_run
    /// use mrvm::board::MotherBoard;
    /// let mut motherboard = MotherBoard::new(vec![ /* component1 */ ]);
    /// motherboard.map(|mut mem| mem.map(0x10000000, 0).unwrap()); // Map first component (ID 0) to address 0x10000000
    /// ```
    pub fn map<T>(&mut self, mut mapper: impl FnMut(&mut MappedMemory) -> T) -> T {
        mapper(&mut self.cpu.mem)
    }

    /// Get a mutable reference to the CPU (required to make the CPU advance)
    pub fn cpu(&mut self) -> &mut Cpu {
        &mut self.cpu
    }

    /// Emulate a hard reset button on the motherboard.
    /// All components will receive a reset signal through their [`Bus`] interface.
    /// The CPU will also be reset, before every other component. Check [`CPU::reset`] for more informations.
    pub fn reset(&mut self) {
        self.cpu.reset();

        for aux in self.aux.iter() {
            aux.borrow_mut().reset();
        }
    }

    /// Get the number of connected components
    pub fn count(&self) -> usize {
        self.aux.len()
    }
}
