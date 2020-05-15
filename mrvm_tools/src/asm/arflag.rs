use std::convert::TryFrom;

/// Arithmetic flag
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArFlag {
    Zero,
    Carry,
    Overflow,
    Sign,
    Even,
    ZeroUpper,
    ZeroLower
}

impl ArFlag {
    /// Decode an arithmetic flag from its code
    pub fn decode(code: u8) -> Result<Self, ()> {
        match code {
            0x00 => Ok(Self::Zero),
            0x01 => Ok(Self::Carry),
            0x02 => Ok(Self::Overflow),
            0x03 => Ok(Self::Sign),
            0x04 => Ok(Self::Even),
            0x05 => Ok(Self::ZeroUpper),
            0x06 => Ok(Self::ZeroLower),
            _ => Err(())
        }
    }

    /// Get the arithmetic flag's code
    pub fn code(self) -> u8 {
        match self {
            Self::Zero => 0x00,
            Self::Carry => 0x01,
            Self::Overflow => 0x02,
            Self::Sign => 0x03,
            Self::Even => 0x04,
            Self::ZeroUpper => 0x05,
            Self::ZeroLower => 0x06
        }
    }

    /// Get the arithmetic flag's name
    pub fn name(self) -> &'static str {
        match self {
            Self::Zero => "Zero",
            Self::Carry => "Carry",
            Self::Overflow => "Overflow",
            Self::Sign => "Sign",
            Self::Even => "Even",
            Self::ZeroUpper => "ZeroUpper",
            Self::ZeroLower => "ZeroLower"
        }
    }

    /// Get the arithmetic flag's short name
    pub fn short_name(self) -> &'static str {
        match self {
            Self::Zero => "ZF",
            Self::Carry => "CF",
            Self::Overflow => "OF",
            Self::Sign => "SF",
            Self::Even => "EF",
            Self::ZeroUpper => "ZUF",
            Self::ZeroLower => "ZLF"
        }
    }

    /// Convert the flag to its LASM representation
    pub fn to_lasm(self) -> &'static str {
        self.short_name()
    }
}

impl TryFrom<u8> for ArFlag {
    type Error = ();

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        Self::decode(code)
    }
}

impl Into<u8> for ArFlag {
    fn into(self) -> u8 {
        self.code()
    }
}
