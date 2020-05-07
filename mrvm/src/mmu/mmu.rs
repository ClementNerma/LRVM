use std::sync::{Arc, Mutex};
use crate::board::MappedMemory;
use crate::cpu::Registers;

static PAGE_SIZE: u32 = 4096;

pub struct MMU {
    memory: Arc<Mutex<MappedMemory>>
}

impl MMU {
    pub fn new(memory: Arc<Mutex<MappedMemory>>) -> Self {
        Self { memory }
    }

    pub fn check_entry(&mut self, regs: &Registers, entry_addr: u32, action: MemAction) -> Result<u32, ()> {
        let v_table_entry = self.memory.lock().unwrap().read(entry_addr);

        let shift = match action {
            MemAction::Exec  => 0,
            MemAction::Read  => 1,
            MemAction::Write => 2
        };

        if ((v_table_entry >> (25 + shift + if regs.smt != 0 { 3 } else { 0 })) & 0b1) == 1 {
            Ok(v_table_entry & 0b11111111111111111111)
        } else {
            Err(())
        }
    }

    pub fn translate(&mut self, regs: &Registers, v_addr: u32, action: MemAction) -> Result<u32, ()> {
        if regs.mtt == 0 {
            return Ok(v_addr);
        }

        let v1_page_number = v_addr >> 22;
        let v1_page_addr = regs.pda + (v1_page_number * 4);

        let v2_page_number = self.check_entry(regs, v1_page_addr, action)?;

        let v2_page_addr = v2_page_number * PAGE_SIZE + (v_addr << 10 >> 22);

        let p_page_number = self.check_entry(regs, v2_page_addr, action)?;

        Ok(p_page_number * PAGE_SIZE + (v_addr << 20 >> 20))
    }

    pub fn read(&mut self, regs: &Registers, v_addr: u32) -> Result<u32, ()> {
        self.translate(regs, v_addr, MemAction::Read).map(|p_addr| self.memory.lock().unwrap().read(p_addr))
    }

    pub fn write(&mut self, regs: &Registers, v_addr: u32, word: u32) -> Result<(), ()> {
        self.translate(regs, v_addr, MemAction::Write).map(|p_addr| self.memory.lock().unwrap().write(p_addr, word))
    }

    pub fn exec(&mut self, regs: &Registers, v_addr: u32) -> Result<u32, ()> {
        self.translate(regs, v_addr, MemAction::Exec).map(|p_addr| self.memory.lock().unwrap().read(p_addr))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MemAction {
    Read,
    Write,
    Exec
}
