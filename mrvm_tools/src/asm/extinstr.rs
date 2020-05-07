use super::{Instr, Reg};

pub enum ExtInstr {
    SetReg(Reg, u32),
    ReadAddr(u32),
    ReadAddrTo(Reg, u32),
    WriteAddr(u32, Reg),
    WriteAddrLit(u32, u32)
}

impl ExtInstr {
    pub fn to_instr(&self) -> Vec<Instr> {
        match self {
            ExtInstr::SetReg(reg, value) => vec![
                Instr::CPY(*reg, ((value >> 16) as u16).into()),
                Instr::LSH(*reg, 16_u8.into()),
                Instr::ADD(*reg, (*value as u16).into()),
            ],

            ExtInstr::ReadAddr(addr) => vec![
                Instr::CPY(Reg::avr, ((addr >> 16) as u16).into()),
                Instr::LSH(Reg::avr, 16_u8.into()),
                Instr::ADD(Reg::avr, (*addr as u16).into()),
                Instr::LEA(Reg::avr.into(), 0u8.into(), 0u8.into())
            ],

            ExtInstr::ReadAddrTo(reg, addr) => vec![
                Instr::CPY(Reg::avr, ((addr >> 16) as u16).into()),
                Instr::LSH(Reg::avr, 16_u8.into()),
                Instr::ADD(Reg::avr, (*addr as u16).into()),
                Instr::LEA(Reg::avr.into(), 0u8.into(), 0u8.into()),
                Instr::CPY(*reg, Reg::avr.into())
            ],

            ExtInstr::WriteAddr(addr, reg_value) => vec![
                Instr::CPY(Reg::rr0, ((addr >> 16) as u16).into()),
                Instr::LSH(Reg::rr0, 16_u8.into()),
                Instr::ADD(Reg::rr0, (*addr as u16).into()),
                
                Instr::CPY(Reg::avr, (*reg_value).into()),
                Instr::WEA(Reg::rr0.into(), 0u8.into(), 0u8.into())
            ],

            ExtInstr::WriteAddrLit(addr, value) => vec![
                Instr::CPY(Reg::rr0, ((addr >> 16) as u16).into()),
                Instr::LSH(Reg::rr0, 16_u8.into()),
                Instr::ADD(Reg::rr0, (*addr as u16).into()),

                Instr::CPY(Reg::avr, ((value >> 16) as u16).into()),
                Instr::LSH(Reg::avr, 16_u8.into()),
                Instr::ADD(Reg::avr, (*value as u16).into()),

                Instr::WEA(Reg::rr0.into(), 0u8.into(), 0u8.into())
            ]
        }
    }

    pub fn encode_words(&self) -> Vec<u32> {
        self.to_instr().into_iter().map(|instr| instr.encode_word()).collect()
    }

    pub fn encode(&self) -> Vec<u8> {
        self.to_instr().into_iter().map(|instr| instr.encode().to_vec()).flatten().collect()
    }
}
