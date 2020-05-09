use std::fmt;
use super::types::*;

#[derive(Copy, Clone, Debug)]
pub enum DeviceCategory {
    Display(DisplayType),
    Keyboard(KeyboardType),
    Memory(MemoryType),
    Storage(StorageType)
}

impl DeviceCategory {
    pub fn decode(code: u64) -> Result<Self, ()> {
        let cat = (code >> 32) as u32;
        let typ = (code & 0xFFFFFFFF) as u32;

        match cat {
            0x00001000 => Ok(Self::Display(DisplayType::decode(typ)?)),
            0x00002000 => Ok(Self::Keyboard(KeyboardType::decode(typ)?)),
            0x00005000 => Ok(Self::Memory(MemoryType::decode(typ)?)),
            0x0000A000 => Ok(Self::Storage(StorageType::decode(typ)?)),

            _ => Err(())
        }
    }

    pub fn category_code(&self) -> u32 {
        match self {
            Self::Display(_) => 0x00001000,
            Self::Keyboard(_) => 0x00002000,
            Self::Memory(_) => 0x00005000,
            Self::Storage(_) => 0x0000A000
        }
    }

    pub fn type_code(&self) -> u32 {
        match self {
            Self::Display(t) => t.code(),
            Self::Keyboard(t) => t.code(),
            Self::Memory(t) => t.code(),
            Self::Storage(t) => t.code()
        }
    }

    pub fn encode(&self) -> u64 {
        ((self.category_code() as u64) << 32) + self.type_code() as u64
    }
}

impl fmt::Display for DeviceCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Display(d) => format!("Display:{}", d),
            Self::Keyboard(k) => format!("Keyboard:{}", k),
            Self::Memory(m) => format!("Memory:{}", m),
            Self::Storage(s) => format!("Storage:{}", s),
        })
    }
}