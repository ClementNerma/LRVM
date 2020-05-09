
/// Describe a native exception
pub enum NativeException {
    UnknownOpCode(u8),
    UnknownRegister(u8),
    ReadProtectedRegister(u8),
    WriteProtectedRegister(u8),
    UnalignedMemoryAddress { unalignment: u8 },
    MMURefusedRead(u16),
    MMURefusedWrite(u16),
    MMURefusedExec(u16),
    SupervisorReservedInstruction(u8),
    DivisionOrModByZero,
    ForbiddenOverflowDivOrMod,
    UnknownComponentID,
    UnknownHardwareInformationCode,
    ComponentNotMapped,
    HardwareException,
    Interruption(u8)
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
        let data = u16::from_be_bytes([ bytes[2], bytes[3] ]);

        let ex = match code {
            0x01 => Self::UnknownOpCode(data as u8),
            0x02 => Self::UnknownRegister(data as u8),
            0x03 => Self::ReadProtectedRegister(data as u8),
            0x04 => Self::WriteProtectedRegister(data as u8),
            0x05 => Self::UnalignedMemoryAddress { unalignment: data as u8 },
            0x06 => Self::MMURefusedRead(data),
            0x07 => Self::MMURefusedWrite(data),
            0x08 => Self::MMURefusedExec(data),
            0x09 => Self::SupervisorReservedInstruction(data as u8),
            0x0A => Self::DivisionOrModByZero,
            0x0B => Self::ForbiddenOverflowDivOrMod,
            0x0C => Self::UnknownComponentID,
            0x0D => Self::UnknownHardwareInformationCode,
            0x0E => Self::ComponentNotMapped,
            0x10 => Self::HardwareException,
            0xAA => Self::Interruption(data as u8),

            _ => return Err(())
        };

        Ok((ex, bytes[0] != 0))
    }

    /// Get the exception's code
    pub fn code(&self) -> u8 {
        match self {
            Self::UnknownOpCode(_) => 0x01,
            Self::UnknownRegister(_) => 0x02,
            Self::ReadProtectedRegister(_) => 0x03,
            Self::WriteProtectedRegister(_) => 0x04,
            Self::UnalignedMemoryAddress { unalignment: _ } => 0x05,
            Self::MMURefusedRead(_) => 0x06,
            Self::MMURefusedWrite(_) => 0x07,
            Self::MMURefusedExec(_) => 0x08,
            Self::SupervisorReservedInstruction(_) => 0x09,
            Self::DivisionOrModByZero => 0x0A,
            Self::ForbiddenOverflowDivOrMod => 0x0B,
            Self::UnknownComponentID => 0x0C,
            Self::UnknownHardwareInformationCode => 0x0D,
            Self::ComponentNotMapped => 0x0E,
            Self::HardwareException => 0x10,
            Self::Interruption(_) => 0xAA,
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
            Self::MMURefusedRead(addr_lower) => Some(*addr_lower),
            Self::MMURefusedWrite(addr_lower) => Some(*addr_lower),
            Self::MMURefusedExec(addr_lower) => Some(*addr_lower),
            Self::SupervisorReservedInstruction(opcode) => Some((*opcode).into()),
            Self::DivisionOrModByZero => None,
            Self::ForbiddenOverflowDivOrMod => None,
            Self::UnknownComponentID => None,
            Self::UnknownHardwareInformationCode => None,
            Self::ComponentNotMapped => None,
            Self::HardwareException => None,
            Self::Interruption(code) => Some((*code).into()),
        }
    }

    /// Encode the exception on 24-bits
    pub fn encode(&self) -> u32 {
        (self.code() as u32) << 16 + self.associated_data().unwrap_or(0)
    }

    /// Encode the exception with supervisor informations on 32-bits.
    /// `was_sv` indicates if the error occurred in supervisor mode (else it was on userland mode).
    pub fn encode_with_mode(&self, was_sv: bool) -> u32 {
        self.encode() + if was_sv { 1 << 24 } else { 0 }
    }
}
