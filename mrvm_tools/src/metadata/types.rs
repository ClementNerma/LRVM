use super::DeviceCategory;

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum DisplayType {
    Buffered
}

impl DisplayType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x00000001 => Ok(Self::Buffered),

            _ => Err(())
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            Self::Buffered => 0x00000001
        }
    }

    pub fn wrap(&self) -> DeviceCategory {
        DeviceCategory::Display(*self)
    }

    pub fn encode(&self) -> u64 {
        self.wrap().encode()
    }
}

impl Into<DeviceCategory> for DisplayType {
    fn into(self) -> DeviceCategory {
        self.wrap()
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum KeyboardType {
    ReadlineSynchronous
}

impl KeyboardType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x00000001 => Ok(Self::ReadlineSynchronous),

            _ => Err(())
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            Self::ReadlineSynchronous => 0x00000001
        }
    }

    pub fn wrap(&self) -> DeviceCategory {
        DeviceCategory::Keyboard(*self)
    }

    pub fn encode(&self) -> u64 {
        self.wrap().encode()
    }
}

impl Into<DeviceCategory> for KeyboardType {
    fn into(self) -> DeviceCategory {
        self.wrap()
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum MemoryType {
    Volatile
}

impl MemoryType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x00000001 => Ok(Self::Volatile),

            _ => Err(())
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            Self::Volatile => 0x00000001
        }
    }

    pub fn wrap(&self) -> DeviceCategory {
        DeviceCategory::Memory(*self)
    }

    pub fn encode(&self) -> u64 {
        self.wrap().encode()
    }
}

impl Into<DeviceCategory> for MemoryType {
    fn into(self) -> DeviceCategory {
        self.wrap()
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum StorageType {
    Readonly,
    Flash,
    Persistent
}

impl StorageType {
    pub fn decode(code: u32) -> Result<Self, ()> {
        match code {
            0x00000001 => Ok(Self::Readonly),
            0x00000011 => Ok(Self::Flash),
            0x00000021 => Ok(Self::Persistent),

            _ => Err(())
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            Self::Readonly => 0x00000001,
            Self::Flash => 0x00000011,
            Self::Persistent => 0x00000021
        }
    }

    pub fn wrap(&self) -> DeviceCategory {
        DeviceCategory::Storage(*self)
    }

    pub fn encode(&self) -> u64 {
        self.wrap().encode()
    }
}

impl Into<DeviceCategory> for StorageType {
    fn into(self) -> DeviceCategory {
        self.wrap()
    }
}
