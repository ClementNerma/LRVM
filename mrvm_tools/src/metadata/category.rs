use std::fmt;
use super::types::*;

#[derive(Copy, Clone, Debug)]
pub enum DeviceCategory {
    Clock(ClockType),
    Display(DisplayType),
    Keyboard(KeyboardType),
    Memory(MemoryType),
    Storage(StorageType),
    PlatformSpecific(u32),
    Uncategorized()
}

impl DeviceCategory {
    pub fn decode(code: u64) -> Result<Self, ()> {
        let cat = (code >> 32) as u32;
        let typ = (code & 0xFFFFFFFF) as u32;

        match cat {
            0x00011000 => Ok(Self::Display(DisplayType::decode(typ)?)),
            0x00016000 => Ok(Self::Keyboard(KeyboardType::decode(typ)?)),
            0x00021000 => Ok(Self::Memory(MemoryType::decode(typ)?)),
            0x00022000 => Ok(Self::Storage(StorageType::decode(typ)?)),
            0xEEEEEEEE => Ok(Self::PlatformSpecific(typ)),
            0xFFFFFFFF => Ok(Self::Uncategorized()),

            _ => Err(())
        }
    }

    pub fn category_code(&self) -> u32 {
        match self {
            Self::Clock(_) => 0x00001000,
            Self::Display(_) => 0x00011000,
            Self::Keyboard(_) => 0x00016000,
            Self::Memory(_) => 0x00021000,
            Self::Storage(_) => 0x00022000,
            Self::PlatformSpecific(_) => 0xEEEEEEEE,
            Self::Uncategorized() => 0xFFFFFFFF
        }
    }

    pub fn type_code(&self) -> u32 {
        match self {
            Self::Clock(t) => t.code(),
            Self::Display(t) => t.code(),
            Self::Keyboard(t) => t.code(),
            Self::Memory(t) => t.code(),
            Self::Storage(t) => t.code(),
            Self::PlatformSpecific(typ) => *typ,
            Self::Uncategorized() => 0x00000000
        }
    }

    pub fn encode(&self) -> u64 {
        ((self.category_code() as u64) << 32) + self.type_code() as u64
    }
}

impl fmt::Display for DeviceCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Clock(c) => format!("Clock:{}", c),
            Self::Display(d) => format!("Display:{}", d),
            Self::Keyboard(k) => format!("Keyboard:{}", k),
            Self::Memory(m) => format!("Memory:{}", m),
            Self::Storage(s) => format!("Storage:{}", s),
            Self::PlatformSpecific(code) => format!("PlatformSpecific:(Code={:#010X})", code),
            Self::Uncategorized() => "Uncategorized".to_owned()
        })
    }
}