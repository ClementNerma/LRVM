
pub enum HwException {
    UnknownOperation(u8),
    UnsupportedOperation,

    GenericPhysicalReadError,
    MemoryNotReadable,

    GenericPhysicalWriteError,
    MemoryNotWritable,
}

impl HwException {
    pub fn code(&self) -> u8 {
        match self {
            Self::UnknownOperation(_) => 0x10,
            Self::UnsupportedOperation => 0x11,

            Self::GenericPhysicalReadError => 0x20,
            Self::MemoryNotReadable => 0x21,

            Self::GenericPhysicalWriteError => 0x30,
            Self::MemoryNotWritable => 0x31,
        }
    }

    pub fn encode(&self) -> u16 {
        let associated = match self {
            Self::UnknownOperation(op) => Some(*op),
            Self::UnsupportedOperation => None,

            Self::GenericPhysicalReadError => None,
            Self::MemoryNotReadable => None,

            Self::GenericPhysicalWriteError => None,
            Self::MemoryNotWritable => None,
        };

        (self.code() as u16) << 8 + associated.unwrap_or(0) as u16
    }
}

impl Into<u16> for HwException {
    fn into(self) -> u16 {
        self.encode()
    }
}