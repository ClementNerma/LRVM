use super::types::*;

#[derive(Copy, Clone, Debug)]
pub enum DeviceCategory {
    Display(DisplayType),
    Keyboard(KeyboardType),
    Memory(MemoryType),
    Storage(StorageType)
}

impl DeviceCategory {
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
        (self.category_code() as u64) << 32 + self.type_code() as u64
    }
}
