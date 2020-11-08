use super::DeviceCategory;
use std::fmt;

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum ClockType {
    Realtime,
}

impl ClockType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x0000_0001 => Ok(Self::Realtime),

            _ => Err(()),
        }
    }

    pub fn code(self) -> u32 {
        match self {
            Self::Realtime => 0x0000_0001,
        }
    }

    pub fn wrap(self) -> DeviceCategory {
        DeviceCategory::Clock(self)
    }

    pub fn encode(self) -> u64 {
        self.wrap().encode()
    }
}

impl fmt::Display for ClockType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Realtime => "Realtime",
            }
        )
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum DisplayType {
    Number,
    Buffered,
}

impl DisplayType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x0000_0001 => Ok(Self::Number),
            0x0000_0100 => Ok(Self::Buffered),

            _ => Err(()),
        }
    }

    pub fn code(self) -> u32 {
        match self {
            Self::Number => 0x0000_0001,
            Self::Buffered => 0x0000_0100,
        }
    }

    pub fn wrap(self) -> DeviceCategory {
        DeviceCategory::Display(self)
    }

    pub fn encode(self) -> u64 {
        self.wrap().encode()
    }
}

impl Into<DeviceCategory> for DisplayType {
    fn into(self) -> DeviceCategory {
        self.wrap()
    }
}

impl fmt::Display for DisplayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number => "Number",
                Self::Buffered => "Buffered",
            }
        )
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum KeyboardType {
    ReadlineSynchronous,
}

impl KeyboardType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x0000_0100 => Ok(Self::ReadlineSynchronous),

            _ => Err(()),
        }
    }

    pub fn code(self) -> u32 {
        match self {
            Self::ReadlineSynchronous => 0x0000_0100,
        }
    }

    pub fn wrap(self) -> DeviceCategory {
        DeviceCategory::Keyboard(self)
    }

    pub fn encode(self) -> u64 {
        self.wrap().encode()
    }
}

impl Into<DeviceCategory> for KeyboardType {
    fn into(self) -> DeviceCategory {
        self.wrap()
    }
}

impl fmt::Display for KeyboardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ReadlineSynchronous => "Readline synchronous",
            }
        )
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum MemoryType {
    RAM,
}

impl MemoryType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x0000_0100 => Ok(Self::RAM),

            _ => Err(()),
        }
    }

    pub fn code(self) -> u32 {
        match self {
            Self::RAM => 0x0000_0100,
        }
    }

    pub fn wrap(self) -> DeviceCategory {
        DeviceCategory::Memory(self)
    }

    pub fn encode(self) -> u64 {
        self.wrap().encode()
    }
}

impl Into<DeviceCategory> for MemoryType {
    fn into(self) -> DeviceCategory {
        self.wrap()
    }
}

impl fmt::Display for MemoryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::RAM => "RAM",
            }
        )
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum StorageType {
    Readonly,
    Flash,
    Persistent,
}

impl StorageType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x0000_0100 => Ok(Self::Readonly),
            0x0000_0011 => Ok(Self::Flash),
            0x0000_0021 => Ok(Self::Persistent),

            _ => Err(()),
        }
    }

    pub fn code(self) -> u32 {
        match self {
            Self::Readonly => 0x0000_0100,
            Self::Flash => 0x0000_0011,
            Self::Persistent => 0x0000_0021,
        }
    }

    pub fn wrap(self) -> DeviceCategory {
        DeviceCategory::Storage(self)
    }

    pub fn encode(self) -> u64 {
        self.wrap().encode()
    }
}

impl Into<DeviceCategory> for StorageType {
    fn into(self) -> DeviceCategory {
        self.wrap()
    }
}

impl fmt::Display for StorageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Readonly => "Readonly",
                Self::Flash => "Flash",
                Self::Persistent => "Persistent",
            }
        )
    }
}
