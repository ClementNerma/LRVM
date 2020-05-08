
/// Condition for 'IF2' instruction
pub enum If2Cond {
    Or,
    And,
    Xor,
    Nor,
    Nand,
    Left,
    Right
}

impl If2Cond {
    /// Decode an 'IF2' condition
    pub fn decode(code: u8) -> Result<Self, ()> {
        match code {
            0x01 => Ok(Self::Or),
            0x02 => Ok(Self::And),
            0x03 => Ok(Self::Xor),
            0x04 => Ok(Self::Nor),
            0x05 => Ok(Self::Nand),
            0x06 => Ok(Self::Left),
            0x07 => Ok(Self::Right),
            _ => Err(())
        }
    }

    /// Get the code of the 'IF2' condition
    pub fn code(&self) -> u8 {
        match self {
            Self::Or    => 0x01,
            Self::And   => 0x02,
            Self::Xor   => 0x03,
            Self::Nor   => 0x04,
            Self::Nand  => 0x05,
            Self::Left  => 0x06,
            Self::Right => 0x07
        }
    }

    /// Get the name of the 'IF2' condition
    pub fn name(&self) -> &'static str {
        match self {
            Self::Or    => "OR"  ,
            Self::And   => "AND" ,
            Self::Xor   => "XOR" ,
            Self::Nor   => "NOR" ,
            Self::Nand  => "NAND",
            Self::Left  => "LEFT",
            Self::Right => "RIGHT"
        }
    }
}
