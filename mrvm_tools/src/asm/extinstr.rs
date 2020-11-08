//! Extended instructions (ExtInstr) are a set of powerful instructions that compile into several sub-instructions.

use super::{Instr, Program, Reg};

/// Extended instruction
#[derive(Debug, Copy, Clone)]
pub enum ExtInstr {
    SetReg(Reg, u32),
    ReadAddr(u32),
    ReadAddrTo(Reg, u32),
    WriteAddr(u32, Reg),
    WriteAddrLit(u32, u32),
}

impl ExtInstr {
    /// Convert the extended instruction into a set of native instructions
    pub fn to_instr(&self) -> Vec<Instr> {
        match self {
            ExtInstr::SetReg(reg, value) => vec![
                Instr::Cpy(*reg, ((value >> 16) as u16).into()),
                Instr::Shl(*reg, 16_u8.into()),
                Instr::Add(*reg, (*value as u16).into()),
            ],

            ExtInstr::ReadAddr(addr) => vec![
                Instr::Cpy(Reg::avr, ((addr >> 16) as u16).into()),
                Instr::Shl(Reg::avr, 16_u8.into()),
                Instr::Add(Reg::avr, (*addr as u16).into()),
                Instr::Lea(Reg::avr.into(), 0u8.into(), 0u8.into()),
            ],

            ExtInstr::ReadAddrTo(reg, addr) => vec![
                Instr::Cpy(Reg::avr, ((addr >> 16) as u16).into()),
                Instr::Shl(Reg::avr, 16_u8.into()),
                Instr::Add(Reg::avr, (*addr as u16).into()),
                Instr::Lea(Reg::avr.into(), 0u8.into(), 0u8.into()),
                Instr::Cpy(*reg, Reg::avr.into()),
            ],

            ExtInstr::WriteAddr(addr, reg_value) => vec![
                Instr::Cpy(Reg::rr0, ((addr >> 16) as u16).into()),
                Instr::Shl(Reg::rr0, 16_u8.into()),
                Instr::Add(Reg::rr0, (*addr as u16).into()),
                Instr::Cpy(Reg::avr, (*reg_value).into()),
                Instr::Wea(Reg::rr0.into(), 0u8.into(), 0u8.into()),
            ],

            ExtInstr::WriteAddrLit(addr, value) => vec![
                Instr::Cpy(Reg::rr0, ((addr >> 16) as u16).into()),
                Instr::Shl(Reg::rr0, 16_u8.into()),
                Instr::Add(Reg::rr0, (*addr as u16).into()),
                Instr::Cpy(Reg::avr, ((value >> 16) as u16).into()),
                Instr::Shl(Reg::avr, 16_u8.into()),
                Instr::Add(Reg::avr, (*value as u16).into()),
                Instr::Wea(Reg::rr0.into(), 0u8.into(), 0u8.into()),
            ],
        }
    }

    /// Convert the extended instruction into machine code (split in words)
    pub fn encode_words(&self) -> Vec<u32> {
        Program::from(self.to_instr()).encode_words()
    }

    /// Convert the extended instruction into machine code
    pub fn encode(&self) -> Vec<u8> {
        Program::from(self.to_instr()).encode()
    }

    /// Convert the extended instruction to a LASM source code
    pub fn to_lasm(&self) -> String {
        Program::from(self.to_instr()).to_lasm(false)
    }
}
