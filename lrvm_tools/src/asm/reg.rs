use super::{RegOrLit1, RegOrLit2};
use std::fmt;

/// CPU register
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Reg {
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    c0,
    c1,
    ac0,
    ac1,
    ac2,
    rr0,
    rr1,
    rr2,
    rr3,
    rr4,
    rr5,
    rr6,
    rr7,
    avr,
    af,
    pc,
    ssp,
    usp,
    et,
    era,
    ev,
    mtt,
    pda,
    smt,
}

impl Reg {
    /// Decode a register code
    pub fn from_code(code: u8) -> Result<Self, ()> {
        match code {
            0x00 => Ok(Self::a0),
            0x01 => Ok(Self::a1),
            0x02 => Ok(Self::a2),
            0x03 => Ok(Self::a3),
            0x04 => Ok(Self::a4),
            0x05 => Ok(Self::a5),
            0x06 => Ok(Self::a6),
            0x07 => Ok(Self::a7),
            0x08 => Ok(Self::c0),
            0x09 => Ok(Self::c1),
            0x0A => Ok(Self::ac0),
            0x0B => Ok(Self::ac1),
            0x0C => Ok(Self::ac2),
            0x0D => Ok(Self::rr0),
            0x0E => Ok(Self::rr1),
            0x0F => Ok(Self::rr2),
            0x10 => Ok(Self::rr3),
            0x11 => Ok(Self::rr4),
            0x12 => Ok(Self::rr5),
            0x13 => Ok(Self::rr6),
            0x14 => Ok(Self::rr7),
            0x15 => Ok(Self::avr),
            0x16 => Ok(Self::pc),
            0x17 => Ok(Self::af),
            0x18 => Ok(Self::ssp),
            0x19 => Ok(Self::usp),
            0x1A => Ok(Self::et),
            0x1B => Ok(Self::era),
            0x1C => Ok(Self::ev),
            0x1D => Ok(Self::mtt),
            0x1E => Ok(Self::pda),
            0x1F => Ok(Self::smt),
            _ => Err(()),
        }
    }

    /// Decode a register from its name
    pub fn from_name(name: &str) -> Result<Self, ()> {
        match name {
            "a0" => Ok(Self::a0),
            "a1" => Ok(Self::a1),
            "a2" => Ok(Self::a2),
            "a3" => Ok(Self::a3),
            "a4" => Ok(Self::a4),
            "a5" => Ok(Self::a5),
            "a6" => Ok(Self::a6),
            "a7" => Ok(Self::a7),
            "c0" => Ok(Self::c0),
            "c1" => Ok(Self::c1),
            "ac0" => Ok(Self::ac0),
            "ac1" => Ok(Self::ac1),
            "ac2" => Ok(Self::ac2),
            "rr0" => Ok(Self::rr0),
            "rr1" => Ok(Self::rr1),
            "rr2" => Ok(Self::rr2),
            "rr3" => Ok(Self::rr3),
            "rr4" => Ok(Self::rr4),
            "rr5" => Ok(Self::rr5),
            "rr6" => Ok(Self::rr6),
            "rr7" => Ok(Self::rr7),
            "avr" => Ok(Self::avr),
            "af" => Ok(Self::af),
            "pc" => Ok(Self::pc),
            "ssp" => Ok(Self::ssp),
            "usp" => Ok(Self::usp),
            "et" => Ok(Self::et),
            "era" => Ok(Self::era),
            "ev" => Ok(Self::ev),
            "mtt" => Ok(Self::mtt),
            "pda" => Ok(Self::pda),
            "smt" => Ok(Self::smt),
            _ => Err(()),
        }
    }

    /// Get the register's code
    pub fn code(self) -> u8 {
        match self {
            Self::a0 => 0x00,
            Self::a1 => 0x01,
            Self::a2 => 0x02,
            Self::a3 => 0x03,
            Self::a4 => 0x04,
            Self::a5 => 0x05,
            Self::a6 => 0x06,
            Self::a7 => 0x07,
            Self::c0 => 0x08,
            Self::c1 => 0x09,
            Self::ac0 => 0x0A,
            Self::ac1 => 0x0B,
            Self::ac2 => 0x0C,
            Self::rr0 => 0x0D,
            Self::rr1 => 0x0E,
            Self::rr2 => 0x0F,
            Self::rr3 => 0x10,
            Self::rr4 => 0x11,
            Self::rr5 => 0x12,
            Self::rr6 => 0x13,
            Self::rr7 => 0x14,
            Self::avr => 0x15,
            Self::pc => 0x16,
            Self::af => 0x17,
            Self::ssp => 0x18,
            Self::usp => 0x19,
            Self::et => 0x1A,
            Self::era => 0x1B,
            Self::ev => 0x1C,
            Self::mtt => 0x1D,
            Self::pda => 0x1E,
            Self::smt => 0x1F,
        }
    }

    /// Get the register's name
    pub fn name(self) -> &'static str {
        match self {
            Self::a0 => "a0",
            Self::a1 => "a1",
            Self::a2 => "a2",
            Self::a3 => "a3",
            Self::a4 => "a4",
            Self::a5 => "a5",
            Self::a6 => "a6",
            Self::a7 => "a7",
            Self::c0 => "c0",
            Self::c1 => "c1",
            Self::ac0 => "ac0",
            Self::ac1 => "ac1",
            Self::ac2 => "ac2",
            Self::rr0 => "rr0",
            Self::rr1 => "rr1",
            Self::rr2 => "rr2",
            Self::rr3 => "rr3",
            Self::rr4 => "rr4",
            Self::rr5 => "rr5",
            Self::rr6 => "rr6",
            Self::rr7 => "rr7",
            Self::avr => "avr",
            Self::pc => "pc",
            Self::af => "af",
            Self::ssp => "ssp",
            Self::usp => "usp",
            Self::et => "et",
            Self::era => "era",
            Self::ev => "ev",
            Self::mtt => "mtt",
            Self::pda => "pda",
            Self::smt => "smt",
        }
    }

    /// Convert the register to a register-or-1-byte-literal parameter
    pub fn to_roc_1(self) -> RegOrLit1 {
        RegOrLit1::reg(self)
    }

    /// Convert the register to a register-or-2-bytes-literal parameter
    pub fn to_roc_2(self) -> RegOrLit2 {
        RegOrLit2::reg(self)
    }

    /// Convert the register to LASM representation
    pub fn to_lasm(self) -> &'static str {
        self.name()
    }
}

impl TryFrom<u8> for Reg {
    type Error = ();

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

impl From<Reg> for u8 {
    fn from(reg: Reg) -> u8 {
        reg.code()
    }
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
