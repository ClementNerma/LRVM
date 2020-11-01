use crate::cpu::Registers;
use crate::mem::MappedMemory;

/// Memory Management Unit (MMU)
#[derive(Default)]
pub struct MMU {}

pub enum EntryDecodingResult {
    Decoded(u32),
    PassThrough,
    PermissionNotSet,
    HwException(u16),
}

impl MMU {
    pub fn new() -> Self {
        Self {}
    }

    pub fn decode_entry(
        &mut self,
        mem: &mut MappedMemory,
        regs: &Registers,
        entry_addr: u32,
        action: MemAction,
    ) -> EntryDecodingResult {
        // Exception receiver
        let mut ex = 0;

        // Get the permissions from the provided entry address in memory
        let v_entry = mem.read(entry_addr, &mut ex);

        // Handle memory errors
        if ex != 0 {
            return EntryDecodingResult::HwException(ex);
        }

        // Check if pass-through is enabled for this entry
        if v_entry & (0b1 << if regs.smt != 0 { 31 } else { 30 }) == 0b0 {
            return EntryDecodingResult::PassThrough;
        }

        // 1. Determine the shift for current mode
        let mode_shift = if regs.smt != 0 { 3 } else { 0 };

        // 2. Determine the shift for the provided type of action
        let action_shift = match action {
            MemAction::Read => 2,
            MemAction::Write => 1,
            MemAction::Exec => 0,
        };

        // 3. Check if the permission bit is set
        if (v_entry & (0b1 << (24 + action_shift + mode_shift))) == 1 {
            // 4. If so, get the weakest 24 bits as the entry value
            EntryDecodingResult::Decoded(v_entry & 0b1111_1111_1111_1111_1111_1111)
        } else {
            // 5. Else, return an error
            EntryDecodingResult::PermissionNotSet
        }
    }

    pub fn translate(
        &mut self,
        mem: &mut MappedMemory,
        regs: &Registers,
        v_addr: u32,
        action: MemAction,
    ) -> Result<u32, Option<u16>> {
        // Skip this if the MMU is disabled
        if regs.mtt == 0 {
            return Ok(v_addr);
        }

        // Get the entry number in the VPI
        let vpi_entry_number = v_addr & 0b11_1111_1111;

        // Get the address of the VPI entry to read
        let vpi_entry_addr = regs.pda + (vpi_entry_number * 4);

        // Get the virtual page number from the VPI entry
        let v_page_number = match self.decode_entry(mem, regs, vpi_entry_addr, action) {
            EntryDecodingResult::Decoded(value) => value,
            EntryDecodingResult::PassThrough => return Ok(v_addr),
            EntryDecodingResult::PermissionNotSet => return Err(None),
            EntryDecodingResult::HwException(ex) => return Err(Some(ex)),
        };

        // Get the address of the virtual page
        let v_page_addr = v_page_number * 16384;

        // Get the address of the virtual page entry to read
        let v_page_entry_addr = v_page_addr + (v_addr.wrapping_shl(10) >> 22) * 4;

        // Get the physical page's number from the virtual page entry
        let p_page_number = match self.decode_entry(mem, regs, v_page_entry_addr, action) {
            EntryDecodingResult::Decoded(value) => value,
            EntryDecodingResult::PassThrough => return Ok(v_addr),
            EntryDecodingResult::PermissionNotSet => return Err(None),
            EntryDecodingResult::HwException(ex) => return Err(Some(ex)),
        };

        // Translate the virtual address into a physical one
        Ok(p_page_number * 1024 + (v_addr & 0b11_1111_1111))
    }
}

/// Memory action
#[derive(Copy, Clone, Debug)]
pub enum MemAction {
    Read,
    Write,
    Exec,
}
