
/// Hardware information number
pub enum HwInfo {
    Count,
    UIDUpper,
    UIDLower,
    NameLength,
    NameW1,
    NameW2,
    NameW3,
    NameW4,
    NameW5,
    NameW6,
    NameW7,
    NameW8,
    DevSize,
    Category,
    Type,
    Model,
    DataUpper,
    DataLower,
    IsMapped,
    MapStart,
    MapEnd,
}

impl HwInfo {
    pub fn from_code(code: u8) -> Result<Self, ()> {
        match code {
            0x00 => Ok(Self::Count),
            0x01 => Ok(Self::UIDUpper),
            0x02 => Ok(Self::UIDLower),
            0x10 => Ok(Self::NameLength),
            0x11 => Ok(Self::NameW1),
            0x12 => Ok(Self::NameW2),
            0x13 => Ok(Self::NameW3),
            0x14 => Ok(Self::NameW4),
            0x15 => Ok(Self::NameW5),
            0x16 => Ok(Self::NameW6),
            0x17 => Ok(Self::NameW7),
            0x18 => Ok(Self::NameW8),
            0x20 => Ok(Self::DevSize),
            0x21 => Ok(Self::Category),
            0x22 => Ok(Self::Type),
            0x23 => Ok(Self::Model),
            0x24 => Ok(Self::DataUpper),
            0x25 => Ok(Self::DataLower),
            0xA0 => Ok(Self::IsMapped),
            0xA1 => Ok(Self::MapStart),
            0xA2 => Ok(Self::MapEnd),
            _ => Err(())
        }
    }

    pub fn code(&self) -> u8 {
        match self {
            Self::Count => 0x00,
            Self::UIDUpper => 0x01,
            Self::UIDLower => 0x02,
            Self::NameLength => 0x10,
            Self::NameW1 => 0x11,
            Self::NameW2 => 0x12,
            Self::NameW3 => 0x13,
            Self::NameW4 => 0x14,
            Self::NameW5 => 0x15,
            Self::NameW6 => 0x16,
            Self::NameW7 => 0x17,
            Self::NameW8 => 0x18,
            Self::DevSize => 0x20,
            Self::Category => 0x21,
            Self::Type => 0x22,
            Self::Model => 0x23,
            Self::DataUpper => 0x24,
            Self::DataLower => 0x25,
            Self::IsMapped => 0xA0,
            Self::MapStart => 0xA1,
            Self::MapEnd => 0xA2,
        }
    }
}

impl Into<u8> for HwInfo {
    fn into(self) -> u8 {
        self.code()
    }
}
