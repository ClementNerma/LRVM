use std::sync::{Arc, Mutex};
use crate::board::MappedMemory;
use crate::cpu::Registers;

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
    /// If the value of `ex` is not zero when this function returns, a hardware exception occurred with the exception code and data in it.
    pub fn decode_entry(&mut self, regs: &Registers, entry_addr: u32, action: MemAction, ex: &mut u16) -> Option<Result<u32, ()>> {
        // Get the permissions from the provided entry address in memory
        let v_entry = self.memory.lock().unwrap().read(entry_addr, ex);

        // Handle memory errors
        if *ex != 0 {
            return Some(Err(()));
        }

        // We read the mapping status for the current mode
        if v_entry & (0b1 << if regs.smt != 0 { 31 } else { 30 }) == 0b0 {
            return None;
        }

        // 1. Determine the shift for current mode
        let mode_shift = if regs.smt != 0 { 3 } else { 0 };

        // 2. Determine the shift for the provided type of action
        let action_shift = match action {
            MemAction::Read  => 2,
            MemAction::Write => 1,
            MemAction::Exec  => 0,
        };

        // 3. Check if the permission bit is set
        if (v_entry & (0b1 << 24 + action_shift + mode_shift)) == 1 {
            // 4. If so, get the weakest 24 bits as the entry value
            Some(Ok(v_entry & 0b111111111111111111111111))
        } else {
            // 5. Else, return an error
            Some(Err(()))
        }
    }

    /// Translate a virtual address into a physical address.
    /// Returns an error if the requested action cannot be performed on this memory location in current mode.
    /// If the value of `ex` is not zero when this function returns, a hardware exception occurred with the exception code and data in it.
    pub fn translate(&mut self, regs: &Registers, v_addr: u32, action: MemAction, ex: &mut u16) -> Result<u32, ()> {
        // Skip this if the MMU is disabled
        if regs.mtt == 0 {
            return Ok(v_addr);
        }

        // Get the entry number in the VPI
        let vpi_entry_number = v_addr & 0b1111111111;

        // Get the address of the VPI entry to read
        let vpi_entry_addr = regs.pda + (vpi_entry_number * 4);

        // Get the virtual page number from the VPI entry
        let v_page_number = match self.decode_entry(regs, vpi_entry_addr, action, ex) {
            Some(result) => result?,
            None => return Ok(v_addr)
        };

        // Get the address of the virtual page
        let v_page_addr = v_page_number * 16384;

        // Get the address of the virtual page entry to read
        let v_page_entry_addr = v_page_addr + (v_addr.wrapping_shl(10) >> 22) * 4;

        // Get the physical page's number from the virtual page entry
        let p_page_number = match self.decode_entry(regs, v_page_entry_addr, action, ex) {
            Some(result) => result?,
            None => return Ok(v_addr)
        };

        // Translate the virtual address into a physical one
        Ok(p_page_number * 1024 + (v_addr & 0b1111111111))
    }

    /// Translate a virtual address for reading into a physical address.
    /// Returns an error if the requested action cannot be performed on this memory location in current mode.
    /// If the value of `ex` is not zero when this function returns, a hardware exception occurred with the exception code and data in it.
    pub fn read(&mut self, regs: &Registers, v_addr: u32, ex: &mut u16) -> Result<u32, ()> {
        self.translate(regs, v_addr, MemAction::Read, ex).map(|p_addr| self.memory.lock().unwrap().read(p_addr, ex))
    }

    /// Translate a virtual address for writing into a physical address.
    /// Returns an error if the requested action cannot be performed on this memory location in current mode.
    /// If the value of `ex` is not zero when this function returns, a hardware exception occurred with the exception code and data in it.
    pub fn write(&mut self, regs: &Registers, v_addr: u32, word: u32, ex: &mut u16) -> Result<(), ()> {
        self.translate(regs, v_addr, MemAction::Write, ex).map(|p_addr| self.memory.lock().unwrap().write(p_addr, word, ex))
    }

    /// Translate a virtual address for execution into a physical address.
    /// Returns an error if the requested action cannot be performed on this memory location in current mode.
    /// If the value of `ex` is not zero when this function returns, a hardware exception occurred with the exception code and data in it.
    pub fn exec(&mut self, regs: &Registers, v_addr: u32, ex: &mut u16) -> Result<u32, ()> {
        self.translate(regs, v_addr, MemAction::Exec, ex).map(|p_addr| self.memory.lock().unwrap().read(p_addr, ex))
    }
}

/// Memory action
#[derive(Debug, Clone, Copy)]
pub enum MemAction {
    Read,
    Write,
    Exec
}
