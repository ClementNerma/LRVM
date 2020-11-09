use super::Instr;

/// A single word in a strongly-typed program
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramWord {
    Instr(Instr),
    Raw([u8; 4]),
}

impl ProgramWord {
    pub fn decode(bytes: [u8; 4]) -> Self {
        match Instr::decode(bytes) {
            Ok(instr) => Self::Instr(instr),
            Err(_) => Self::Raw(bytes),
        }
    }

    pub fn decode_word(&self, word: u32) -> Self {
        Self::decode(word.to_be_bytes())
    }

    pub fn is_instr(&self) -> bool {
        return matches!(self, Self::Instr(_));
    }

    pub fn is_raw(&self) -> bool {
        return matches!(self, Self::Raw(_));
    }

    pub fn encode(&self) -> [u8; 4] {
        match self {
            Self::Instr(instr) => instr.encode(),
            Self::Raw(bytes) => *bytes,
        }
    }

    pub fn encode_word(&self) -> u32 {
        match self {
            Self::Instr(instr) => instr.encode_word(),
            Self::Raw(bytes) => u32::from_be_bytes(*bytes),
        }
    }

    pub fn to_lasm(&self) -> String {
        match self {
            Self::Instr(instr) => instr.to_lasm(),
            Self::Raw(bytes) => format!(
                "#d32 0x{:002X}_{:002X}_{:002X}_{:002X}",
                bytes[0], bytes[1], bytes[2], bytes[3],
            ),
        }
    }
}
