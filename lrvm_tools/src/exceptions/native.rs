use std::fmt;

use crate::{asm::Reg, exceptions::AuxHwException};

/// Describe a native exception
pub enum NativeException {
    UnknownOpCode(u8),
    UnknownRegister(u8),
    ReadProtectedRegister(u8),
    WriteProtectedRegister(u8),
    UnalignedMemoryAddress { unalignment: u8 },
    MmuRefusedRead(u16),
    MmuRefusedWrite(u16),
    MmuRefusedExec(u16),
    SupervisorReservedInstruction(u8),
    DivisionOrModByZero,
    OverflowingDivOrMod,
    InvalidCondFlag(u8),
    InvalidCondMode(u8),
    UnknownComponentId(u16),
    UnknownHardwareInformationCode(u8),
    ComponentNotMapped(u16),
    HardwareException(AuxHwException),
    Interruption(u8),
}

impl NativeException {
    /// Decode a native exception
    pub fn decode(ex: u32) -> Result<Self, ()> {
        Self::decode_with_mode(ex).map(|(ex, _)| ex)
    }

    /// Decode a native exception along with the supervisor status.
    /// If the error is indicated to have happened in supervisor mode, the second member of the returned tuple is set to `true`.
    /// If it's `false`, the error indicates to have happened in userland mode.
    pub fn decode_with_mode(ex: u32) -> Result<(Self, bool), ()> {
        let bytes = ex.to_be_bytes();

        let code = bytes[1];
        let associated = u16::from_be_bytes([bytes[2], bytes[3]]);

        Ok((Self::decode_parts(code, Some(associated))?, bytes[0] != 0))
    }

    /// Decode a split exception
    pub fn decode_parts(code: u8, associated: Option<u16>) -> Result<Self, ()> {
        let data_or_err = associated.ok_or(());

        match code {
            0x01 => Ok(Self::UnknownOpCode(data_or_err? as u8)),
            0x02 => Ok(Self::UnknownRegister(data_or_err? as u8)),
            0x03 => Ok(Self::ReadProtectedRegister(data_or_err? as u8)),
            0x04 => Ok(Self::WriteProtectedRegister(data_or_err? as u8)),
            0x05 => Ok(Self::UnalignedMemoryAddress {
                unalignment: data_or_err? as u8,
            }),
            0x06 => Ok(Self::MmuRefusedRead(data_or_err?)),
            0x07 => Ok(Self::MmuRefusedWrite(data_or_err?)),
            0x08 => Ok(Self::MmuRefusedExec(data_or_err?)),
            0x09 => Ok(Self::SupervisorReservedInstruction(data_or_err? as u8)),
            0x0A => Ok(Self::DivisionOrModByZero),
            0x0B => Ok(Self::OverflowingDivOrMod),
            0x0C => Ok(Self::InvalidCondFlag(data_or_err? as u8)),
            0x0D => Ok(Self::InvalidCondMode(data_or_err? as u8)),
            0x10 => Ok(Self::UnknownComponentId(data_or_err?)),
            0x11 => Ok(Self::UnknownHardwareInformationCode(data_or_err? as u8)),
            0x12 => Ok(Self::ComponentNotMapped(data_or_err?)),
            0xA0 => Ok(Self::HardwareException(AuxHwException::decode(
                data_or_err?,
            )?)),
            0xF0 => Ok(Self::Interruption(data_or_err? as u8)),

            _ => Err(()),
        }
    }

    /// Get the exception's code
    pub fn code(&self) -> u8 {
        match self {
            Self::UnknownOpCode(_) => 0x01,
            Self::UnknownRegister(_) => 0x02,
            Self::ReadProtectedRegister(_) => 0x03,
            Self::WriteProtectedRegister(_) => 0x04,
            Self::UnalignedMemoryAddress { unalignment: _ } => 0x05,
            Self::MmuRefusedRead(_) => 0x06,
            Self::MmuRefusedWrite(_) => 0x07,
            Self::MmuRefusedExec(_) => 0x08,
            Self::SupervisorReservedInstruction(_) => 0x09,
            Self::DivisionOrModByZero => 0x0A,
            Self::OverflowingDivOrMod => 0x0B,
            Self::InvalidCondFlag(_) => 0x0C,
            Self::InvalidCondMode(_) => 0x0D,
            Self::UnknownComponentId(_) => 0x10,
            Self::UnknownHardwareInformationCode(_) => 0x11,
            Self::ComponentNotMapped(_) => 0x12,
            Self::HardwareException(_) => 0xA0,
            Self::Interruption(_) => 0xF0,
        }
    }

    /// Get the exception's eventual associated data
    pub fn associated_data(&self) -> Option<u16> {
        match self {
            Self::UnknownOpCode(opcode) => Some((*opcode).into()),
            Self::UnknownRegister(reg_id) => Some((*reg_id).into()),
            Self::ReadProtectedRegister(reg_id) => Some((*reg_id).into()),
            Self::WriteProtectedRegister(reg_id) => Some((*reg_id).into()),
            Self::UnalignedMemoryAddress { unalignment } => Some((*unalignment).into()),
            Self::MmuRefusedRead(addr_lower) => Some(*addr_lower),
            Self::MmuRefusedWrite(addr_lower) => Some(*addr_lower),
            Self::MmuRefusedExec(addr_lower) => Some(*addr_lower),
            Self::SupervisorReservedInstruction(opcode) => Some((*opcode).into()),
            Self::DivisionOrModByZero => None,
            Self::OverflowingDivOrMod => None,
            Self::InvalidCondFlag(flag) => Some((*flag).into()),
            Self::InvalidCondMode(flag) => Some((*flag).into()),
            Self::UnknownComponentId(id_lower) => Some(*id_lower),
            Self::UnknownHardwareInformationCode(code) => Some((*code).into()),
            Self::ComponentNotMapped(id_lower) => Some(*id_lower),
            Self::HardwareException(hw_ex) => Some(hw_ex.encode()),
            Self::Interruption(code) => Some((*code).into()),
        }
    }

    /// Encode the exception on 24-bits
    pub fn encode(&self) -> u32 {
        ((self.code() as u32) << 16) + self.associated_data().unwrap_or(0) as u32
    }

    /// Encode the exception with supervisor informations on 32-bits.
    /// `was_sv` indicates if the error occurred in supervisor mode (else it was on userland mode).
    pub fn encode_with_mode(&self, was_sv: bool) -> u32 {
        self.encode() + if was_sv { 1 << 24 } else { 0 }
    }
}

impl fmt::Display for NativeException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::UnknownOpCode(opcode) => format!("Unknown opcode {:#004X}", opcode),
                Self::UnknownRegister(reg_id) => format!("Unknown register code {:#004X}", reg_id),
                Self::ReadProtectedRegister(reg_id) => format!(
                    "Register {} cannot be read in this mode",
                    Reg::from_code(*reg_id).unwrap()
                ),
                Self::WriteProtectedRegister(reg_id) => format!(
                    "Register {} cannot be written in this mode",
                    Reg::from_code(*reg_id).unwrap()
                ),
                Self::UnalignedMemoryAddress { unalignment } =>
                    format!("Unaligned memory address (unalignment is {})", unalignment),
                Self::MmuRefusedRead(addr_lower) => format!(
                    "Address cannot be read in this mode (address' weakest bits are {:#006X})",
                    addr_lower
                ),
                Self::MmuRefusedWrite(addr_lower) => format!(
                    "Address cannot be written in this mode (address' weakest bits are {:#006X})",
                    addr_lower
                ),
                Self::MmuRefusedExec(addr_lower) => format!(
                    "Address cannot be executed in this mode (address' weakest bits are {:#006X})",
                    addr_lower
                ),
                Self::SupervisorReservedInstruction(opcode) => format!(
                    "Instruction with opcode {:#004X} cannot be run in userland mode",
                    opcode
                ),
                Self::DivisionOrModByZero =>
                    "Cannot perform a division or modulus by zero".to_string(),
                Self::OverflowingDivOrMod =>
                    "Cannot perform an overflowing division or modulus".to_string(),
                Self::InvalidCondFlag(flag) =>
                    format!("Invalid IF/IF2 flag provided: {:#004X}", flag),
                Self::InvalidCondMode(mode) =>
                    format!("Invalid IF2 condition mode provided: {:#004X}", mode),
                Self::UnknownComponentId(id_lower) =>
                    format!("Unknown component ID (weakest bits are {:#006X})", id_lower),
                Self::UnknownHardwareInformationCode(code) =>
                    format!("Unknown hardware information code {:#004X}", code),
                Self::ComponentNotMapped(id_lower) =>
                    format!("Component with ID {:#004X} is not mapped", id_lower),
                Self::HardwareException(hw_ex) => format!("Hardware exception: {}", hw_ex),
                Self::Interruption(code) => format!("Interruption (code {:#004X})", code),
            }
        )
    }
}
