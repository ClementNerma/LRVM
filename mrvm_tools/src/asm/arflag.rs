use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArFlag {
    Zero,
    Carry,
    Overflow,
    Sign,
    Parity,
    ZeroUpper,
    ZeroLower
}

impl ArFlag {
    pub fn from_code(code: u8) -> Result<Self, ()> {
        match code {
            0x00 => Ok(Self::Zero),
            0x01 => Ok(Self::Carry),
            0x02 => Ok(Self::Overflow),
            0x03 => Ok(Self::Sign),
            0x04 => Ok(Self::Parity),
            0x05 => Ok(Self::ZeroUpper),
            0x06 => Ok(Self::ZeroLower),
            _ => Err(())
        }
    }

    pub fn code(&self) -> u8 {
        match self {
            Self::Zero => 0x00,
            Self::Carry => 0x01,
            Self::Overflow => 0x02,
            Self::Sign => 0x03,
            Self::Parity => 0x04,
            Self::ZeroUpper => 0x05,
            Self::ZeroLower => 0x06
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Zero => "Zero",
            Self::Carry => "Carry",
            Self::Overflow => "Overflow",
            Self::Sign => "Sign",
            Self::Parity => "Parity",
            Self::ZeroUpper => "ZeroUpper",
            Self::ZeroLower => "ZeroLower"
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Zero => "ZF",
            Self::Carry => "CF",
            Self::Overflow => "OF",
            Self::Sign => "SF",
            Self::Parity => "PF",
            Self::ZeroUpper => "ZUF",
            Self::ZeroLower => "ZLF"
        }
    }
}

impl TryFrom<u8> for ArFlag {
    type Error = ();

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl Into<u8> for ArFlag {
    fn into(self) -> u8 {
        self.code()
    }
}
