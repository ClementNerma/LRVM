use std::sync::{Arc, Mutex};
use crate::board::MappedMemory;
use crate::cpu::Registers;

static PAGE_SIZE: u32 = 4096;

/// Memory Management Unit (MMU)
pub struct MMU {
    /// Motherboard's mapped memory
    memory: Arc<Mutex<MappedMemory>>
}

impl MMU {
    /// Create a new MMU
    pub fn new(memory: Arc<Mutex<MappedMemory>>) -> Self {
        Self { memory }
    }

    /// Decode an entry from a permission page for a specific action type.
    pub fn decode_entry(&mut self, regs: &Registers, entry_addr: u32, action: MemAction) -> Result<u32, ()> {
        // Get the permissions from the provided entry address in memory
        let v_table_entry = self.memory.lock().unwrap().read(entry_addr);

        // We will know read the permission bit for this action

        // 1. Compute the additional shift for this type of action
        let action_shift = match action {
            MemAction::Exec  => 0,
            MemAction::Read  => 1,
            MemAction::Write => 2
        };

        // 2. Compute the additional shift required for userland mode
        let sv_shift = if regs.smt != 0 { 3 } else { 0 };

        // 3. Check if the permission bit is set
        if ((v_table_entry >> (25 + action_shift + sv_shift)) & 0b1) == 1 {
            // 4. If so, clear the 20 top bits to get the entry's content
            Ok(v_table_entry & 0b11111111111111111111)
        } else {
            // 5. Else, return an error
            Err(())
        }
    }

    /// Translate a virtual address into a physical address.
    /// Returns an error if the requested action cannot be performed on this memory location in current mode.
    pub fn translate(&mut self, regs: &Registers, v_addr: u32, action: MemAction) -> Result<u32, ()> {
        // Skip this if the MMU is disabled
        if regs.mtt == 0 {
            return Ok(v_addr);
        }

        // Get the level 1 page's number
        let v1_page_number = v_addr >> 22;
        let v1_page_addr = regs.pda + (v1_page_number * 4);

        // Get the level 2 page's number
        let v2_page_number = self.decode_entry(regs, v1_page_addr, action)?;

        let v2_page_addr = v2_page_number * PAGE_SIZE + (v_addr << 10 >> 22);

        // Get the final permission page's number
        let p_page_number = self.decode_entry(regs, v2_page_addr, action)?;

        // Translate the address
        Ok(p_page_number * PAGE_SIZE + (v_addr << 20 >> 20))
    }

    /// Translate a virtual address for reading into a physical address.
    /// Returns an error if the requested action cannot be performed on this memory location in current mode.
    pub fn read(&mut self, regs: &Registers, v_addr: u32) -> Result<u32, ()> {
        self.translate(regs, v_addr, MemAction::Read).map(|p_addr| self.memory.lock().unwrap().read(p_addr))
    }

    /// Translate a virtual address for writing into a physical address.
    /// Returns an error if the requested action cannot be performed on this memory location in current mode.
    pub fn write(&mut self, regs: &Registers, v_addr: u32, word: u32) -> Result<(), ()> {
        self.translate(regs, v_addr, MemAction::Write).map(|p_addr| self.memory.lock().unwrap().write(p_addr, word))
    }

    /// Translate a virtual address for execution into a physical address.
    /// Returns an error if the requested action cannot be performed on this memory location in current mode.
    pub fn exec(&mut self, regs: &Registers, v_addr: u32) -> Result<u32, ()> {
        self.translate(regs, v_addr, MemAction::Exec).map(|p_addr| self.memory.lock().unwrap().read(p_addr))
    }
}

/// Memory action
#[derive(Debug, Clone, Copy)]
pub enum MemAction {
    Read,
    Write,
    Exec
}
