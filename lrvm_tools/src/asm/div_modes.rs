//! Strongly-typed interfaces for division modes

use super::{cst, RegOrLit1};
use std::fmt::Debug;

/// Division sub mode
pub trait DivSubMode: Sized + Debug + Copy + Clone + PartialEq + Eq {
    /// Sub-mode mask
    const MASK: u8;

    /// Decode the sub mode from a complete mode (e.g. decode the sign mode from the complete 8-bits division mode)
    fn from_mode(mode: u8) -> Result<Self, ()> {
        Self::decode(mode & Self::MASK)
    }

    /// Decode the sub mode
    fn decode(sub_mode: u8) -> Result<Self, ()>;

    /// Encode the sub mode
    fn encode(self) -> u8;

    /// Convert the sub mode to a full division mode.  
    /// All default
    fn to_mode(self) -> DivMode;

    /// Convert the sub mode to its LASM representation (constant name)
    fn to_lasm(self) -> &'static str;
}

/// Division sign
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivSignMode {
    Unsigned,
    Signed,
}

impl DivSignMode {
    pub fn from_is_signed(signed: bool) -> Self {
        if signed {
            Self::Signed
        } else {
            Self::Unsigned
        }
    }

    pub fn is_signed(self) -> bool {
        match self {
            Self::Unsigned => false,
            Self::Signed => true,
        }
    }
}

impl DivSubMode for DivSignMode {
    const MASK: u8 = cst::DIV_SIGN_MODE_MASK;

    fn decode(sub_mode: u8) -> Result<Self, ()> {
        match sub_mode {
            cst::DIV_USG => Ok(Self::Unsigned),
            cst::DIV_SIG => Ok(Self::Signed),
            _ => Err(()),
        }
    }

    fn encode(self) -> u8 {
        match self {
            Self::Unsigned => cst::DIV_USG,
            Self::Signed => cst::DIV_SIG,
        }
    }

    fn to_mode(self) -> DivMode {
        DivMode::from_sub_modes(self, DivByZeroMode::default(), DivOverflowMode::default())
    }

    fn to_lasm(self) -> &'static str {
        match self {
            Self::Unsigned => "DIV_USG",
            Self::Signed => "DIV_SIG",
        }
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
        Self::decode(mask)
    }
}

impl From<DivSignMode> for u8 {
    fn from(mode: DivSignMode) -> u8 {
        mode.encode()
    }
}

/// Rule for division by zero
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivByZeroMode {
    Forbid,
    EqToMin,
    EqToZero,
    EqToMax,
}

impl DivByZeroMode {
    pub fn result(self) -> u32 {
        match self {
            Self::Forbid => 0,
            Self::EqToMin => i32::MIN as u32,
            Self::EqToZero => 0,
            Self::EqToMax => i32::MAX as u32,
        }
    }
}

impl DivSubMode for DivByZeroMode {
    const MASK: u8 = cst::DIV_ZERO_MODE_MASK;

    fn decode(sub_mode: u8) -> Result<Self, ()> {
        match sub_mode {
            cst::DIV_ZRO_FRB => Ok(Self::Forbid),
            cst::DIV_ZRO_MIN => Ok(Self::EqToMin),
            cst::DIV_ZRO_ZRO => Ok(Self::EqToZero),
            cst::DIV_ZRO_MAX => Ok(Self::EqToMin),
            _ => Err(()),
        }
    }

    fn encode(self) -> u8 {
        match self {
            Self::Forbid => cst::DIV_ZRO_FRB,
            Self::EqToMin => cst::DIV_ZRO_MIN,
            Self::EqToZero => cst::DIV_ZRO_ZRO,
            Self::EqToMax => cst::DIV_ZRO_MAX,
        }
    }

    fn to_mode(self) -> DivMode {
        DivMode::from_sub_modes(DivSignMode::default(), self, DivOverflowMode::default())
    }

    fn to_lasm(self) -> &'static str {
        match self {
            Self::Forbid => "DIV_ZRO_FRB",
            Self::EqToMin => "DIV_ZRO_MIN",
            Self::EqToZero => "DIV_ZRO_ZRO",
            Self::EqToMax => "DIV_ZRO_MAX",
        }
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
        Self::decode(mask)
    }
}

impl From<DivByZeroMode> for u8 {
    fn from(mode: DivByZeroMode) -> u8 {
        mode.encode()
    }
}

/// Rule for overflowing division (divising minimum possible value by -1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivOverflowMode {
    Forbid,
    EqToMin,
    EqToZero,
    EqToMax,
}

impl DivOverflowMode {
    pub fn result(self) -> u32 {
        match self {
            Self::Forbid => 0,
            Self::EqToMin => i32::MIN as u32,
            Self::EqToZero => 0,
            Self::EqToMax => i32::MAX as u32,
        }
    }
}

impl DivSubMode for DivOverflowMode {
    const MASK: u8 = cst::DIV_OVFW_MODE_MASK;

    fn decode(sub_mode: u8) -> Result<Self, ()> {
        match sub_mode {
            cst::DIV_OFW_FRB => Ok(Self::Forbid),
            cst::DIV_OFW_MIN => Ok(Self::EqToMin),
            cst::DIV_OFW_ZRO => Ok(Self::EqToZero),
            cst::DIV_OFW_MAX => Ok(Self::EqToMax),
            _ => Err(()),
        }
    }

    fn encode(self) -> u8 {
        match self {
            Self::Forbid => cst::DIV_OFW_FRB,
            Self::EqToMin => cst::DIV_OFW_MIN,
            Self::EqToZero => cst::DIV_OFW_ZRO,
            Self::EqToMax => cst::DIV_OFW_MAX,
        }
    }

    fn to_mode(self) -> DivMode {
        DivMode::from_sub_modes(DivSignMode::default(), DivByZeroMode::default(), self)
    }

    fn to_lasm(self) -> &'static str {
        match self {
            Self::Forbid => "DIV_OFW_FRB",
            Self::EqToMin => "DIV_OFW_MIN",
            Self::EqToZero => "DIV_OFW_ZRO",
            Self::EqToMax => "DIV_OFW_MAX",
        }
    }
}

impl Default for DivOverflowMode {
    fn default() -> Self {
        Self::Forbid
    }
}

impl TryFrom<u8> for DivOverflowMode {
    type Error = ();

    fn try_from(mask: u8) -> Result<Self, Self::Error> {
        Self::decode(mask)
    }
}

impl From<DivOverflowMode> for u8 {
    fn from(mode: DivOverflowMode) -> u8 {
        mode.encode()
    }
}

/// Division/modulus mode, required for the 'div' and 'mod' instructions
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct DivMode(pub DivSignMode, pub DivByZeroMode, pub DivOverflowMode);

impl DivMode {
    /// Get the default division mode:
    /// - Perform unsigned division
    /// - Forbid division by zero (results in exception)
    /// - Forbid overflowing division (results in exception)
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode a division mode
    pub fn decode(mode: u8) -> Result<Self, ()> {
        Ok(Self(
            DivSignMode::from_mode(mode)?,
            DivByZeroMode::from_mode(mode)?,
            DivOverflowMode::from_mode(mode)?,
        ))
    }

    /// Get the division mode from its sub modes
    pub fn from_sub_modes(
        sign_mode: DivSignMode,
        zero_mode: DivByZeroMode,
        mbo_mode: DivOverflowMode,
    ) -> Self {
        Self(sign_mode, zero_mode, mbo_mode)
    }

    /// Get the sign mode
    pub fn sign_mode(self) -> DivSignMode {
        self.0
    }

    /// Get the zero mode
    pub fn zro_mode(self) -> DivByZeroMode {
        self.1
    }

    /// Get the overflow mode
    pub fn mbo_mode(self) -> DivOverflowMode {
        self.2
    }

    /// Set the sign mode
    pub fn with_sign_mode(mut self, mode: DivSignMode) -> Self {
        self.0 = mode;
        self
    }

    /// Set the zero mode
    pub fn with_zro_mode(mut self, mode: DivByZeroMode) -> Self {
        self.1 = mode;
        self
    }

    /// Set the overflow mode
    pub fn with_ofw_mode(mut self, mode: DivOverflowMode) -> Self {
        self.2 = mode;
        self
    }

    /// Encode the mode as a number
    pub fn encode(self) -> u8 {
        self.0.encode() | self.1.encode() | self.2.encode()
    }

    /// Convert the mode to a register-or-literal value
    pub fn to_val(self) -> RegOrLit1 {
        self.encode().into()
    }

    /// Convert the mode to its LASM representation (e.g. 'DIV_MIN_ZRO | DIV_OFW_MAX').  
    /// Sub modes that are set to their default value are not present in the returned string.
    pub fn to_lasm(self) -> String {
        let mut modes = vec![];

        if self.0 != DivSignMode::default() {
            modes.push(self.0.to_lasm());
        }

        if self.1 != DivByZeroMode::default() {
            modes.push(self.1.to_lasm());
        }

        if self.2 != DivOverflowMode::default() {
            modes.push(self.2.to_lasm());
        }

        if modes.is_empty() {
            "0".to_string()
        } else {
            modes.join(" | ")
        }
    }
}

impl TryFrom<u8> for DivMode {
    type Error = ();

    fn try_from(mode: u8) -> Result<Self, Self::Error> {
        Self::decode(mode)
    }
}

impl From<DivMode> for u8 {
    fn from(mode: DivMode) -> u8 {
        mode.encode()
    }
}
