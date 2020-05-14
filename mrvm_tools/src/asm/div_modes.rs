//! Strongly-typed interfaces for division modes

use std::convert::TryFrom;
use super::{cst, RegOrLit1};

/// Division sign
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DivSignMode {
    Unsigned,
    Signed
}

impl DivSignMode {
    pub fn from_is_signed(signed: bool) -> Self {
        if signed {
            Self::Signed
        } else {
            Self::Unsigned
        }
    }

    pub fn from_mask(mask: u8) -> Result<Self, ()> {
        match mask {
            mask if mask == cst::DIV_USG => Ok(Self::Unsigned),
            mask if mask == cst::DIV_SIG => Ok(Self::Signed),
            _ => Err(())
        }
    }

    pub fn is_signed(&self) -> bool {
        match self {
            Self::Unsigned => false,
            Self::Signed => true
        }
    }

    pub fn mask(&self) -> u8 {
        match self {
            Self::Unsigned => cst::DIV_USG,
            Self::Signed => cst::DIV_SIG
        }
    }

    pub fn to_mode(&self) -> DivMode {
        DivMode::from(*self, DivByZeroMode::default(), DivMinByLessOneMode::default())
    }
}

impl Default for DivSignMode {
    fn default() -> Self {
        Self::Unsigned
    }
}

impl TryFrom<u8> for DivSignMode {
    type Error = ();

    fn try_from(mask: u8) -> Result<Self, Self::Error> {
        Self::from_mask(mask)
    }
}

impl Into<u8> for DivSignMode {
    fn into(self) -> u8 {
        self.mask()
    }
}

/// Mode for division by zero
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DivByZeroMode {
    Forbid,
    EqToMin,
    EqToZero,
    EqToMax
}

impl DivByZeroMode {
    pub fn from_mask(mask: u8) -> Result<Self, ()> {
        match mask {
            mask if mask == cst::DIV_ZRO_FRB => Ok(Self::Forbid),
            mask if mask == cst::DIV_ZRO_MIN => Ok(Self::EqToMin),
            mask if mask == cst::DIV_ZRO_ZRO => Ok(Self::EqToZero),
            mask if mask == cst::DIV_ZRO_MAX => Ok(Self::EqToMin),
            _ => Err(())
        }
    }

    pub fn mask(&self) -> u8 {
        match self {
            Self::Forbid   => cst::DIV_ZRO_FRB,
            Self::EqToMin  => cst::DIV_ZRO_MIN,
            Self::EqToZero => cst::DIV_ZRO_ZRO,
            Self::EqToMax  => cst::DIV_ZRO_MAX
        }
    }

    pub fn result(&self) -> u32 {
        match self {
            Self::Forbid   => 0,
            Self::EqToMin  => std::i32::MIN as u32,
            Self::EqToZero => 0,
            Self::EqToMax  => std::i32::MAX as u32
        }
    }

    pub fn to_mode(&self) -> DivMode {
        DivMode::from(DivSignMode::default(), *self, DivMinByLessOneMode::default())
    }
}

impl Default for DivByZeroMode {
    fn default() -> Self {
        Self::Forbid
    }
}

impl TryFrom<u8> for DivByZeroMode {
    type Error = ();

    fn try_from(mask: u8) -> Result<Self, Self::Error> {
        Self::from_mask(mask)
    }
}

impl Into<u8> for DivByZeroMode {
    fn into(self) -> u8 {
        self.mask()
    }
}

/// Mode for overflowing division by -1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DivMinByLessOneMode {
    Forbid,
    EqToMin,
    EqToZero,
    EqToMax
}

impl DivMinByLessOneMode {
    pub fn from_mask(mask: u8) -> Result<Self, ()> {
        match mask {
            mask if mask == cst::DIV_MBO_FRB => Ok(Self::Forbid),
            mask if mask == cst::DIV_MBO_MIN => Ok(Self::EqToMin),
            mask if mask == cst::DIV_MBO_ZRO => Ok(Self::EqToZero),
            mask if mask == cst::DIV_MBO_MAX => Ok(Self::EqToMin),
            _ => Err(())
        }
    }

    pub fn mask(&self) -> u8 {
        match self {
            Self::Forbid   => cst::DIV_MBO_FRB,
            Self::EqToMin  => cst::DIV_MBO_MIN,
            Self::EqToZero => cst::DIV_MBO_ZRO,
            Self::EqToMax  => cst::DIV_MBO_MAX
        }
    }

    pub fn result(&self) -> u32 {
        match self {
            Self::Forbid   => 0,
            Self::EqToMin  => std::i32::MIN as u32,
            Self::EqToZero => 0,
            Self::EqToMax  => std::i32::MAX as u32
        }
    }

    pub fn to_mode(&self) -> DivMode {
        DivMode::from(DivSignMode::default(), DivByZeroMode::default(), *self)
    }
}

impl Default for DivMinByLessOneMode {
    fn default() -> Self {
        Self::Forbid
    }
}

impl TryFrom<u8> for DivMinByLessOneMode {
    type Error = ();

    fn try_from(mask: u8) -> Result<Self, Self::Error> {
        Self::from_mask(mask)
    }
}

impl Into<u8> for DivMinByLessOneMode {
    fn into(self) -> u8 {
        self.mask()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DivMode(pub DivSignMode, pub DivByZeroMode, pub DivMinByLessOneMode);

impl DivMode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(sign_mode: DivSignMode, zero_mode: DivByZeroMode, mbo_mode: DivMinByLessOneMode) -> Self {
        Self(sign_mode, zero_mode, mbo_mode)
    }

    pub fn from_mode(mode: u8) -> Result<Self, ()> {
        Ok(Self(
            DivSignMode::from_mask(mode & 0b0001_0000)?,
            DivByZeroMode::from_mask(mode & 0b0000_1100)?,
            DivMinByLessOneMode::from_mask(mode & 0b0000_0011)?
        ))
    }

    pub fn sign_mode(&self) -> DivSignMode {
        self.0
    }

    pub fn zro_mode(&self) -> DivByZeroMode {
        self.1
    }

    pub fn mbo_mode(&self) -> DivMinByLessOneMode {
        self.2
    }

    pub fn set_sign_mode(&mut self, mode: DivSignMode) {
        self.0 = mode;
    }

    pub fn set_zro_mode(&mut self, mode: DivByZeroMode) {
        self.1 = mode;
    }

    pub fn set_mbo_mode(&mut self, mode: DivMinByLessOneMode) {
        self.2 = mode;
    }

    pub fn mode(&self) -> u8 {
        self.0.mask() | self.1.mask() | self.2.mask()
    }

    pub fn to_roc(&self) -> RegOrLit1 {
        self.mode().into()
    }
}

impl TryFrom<u8> for DivMode {
    type Error = ();

    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        Self::from_mode(mode)
    }
}

impl Into<u8> for DivMode {
    fn into(self) -> u8 {
        self.mode()
    }
}
