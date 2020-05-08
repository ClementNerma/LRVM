use super::{Reg, RegOrLit1, RegOrLit2};

/// Native assembly instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instr {
    CPY(Reg, RegOrLit2),
    EX(Reg, Reg),
    ADD(Reg, RegOrLit2),
    SUB(Reg, RegOrLit2),
    MUL(Reg, RegOrLit2),
    DIV(Reg, RegOrLit1, RegOrLit1),
    MOD(Reg, RegOrLit1, RegOrLit1),
    AND(Reg, RegOrLit2),
    BOR(Reg, RegOrLit2),
    XOR(Reg, RegOrLit2),
    LSH(Reg, RegOrLit1),
    RSH(Reg, RegOrLit1),
    CMP(Reg, RegOrLit2),
    JMP(RegOrLit2),
    LSM(RegOrLit2),
    ITR(RegOrLit1),
    IF(RegOrLit1),
    IFN(RegOrLit1),
    IF2(RegOrLit1, RegOrLit1, RegOrLit1),
    LSA(Reg, RegOrLit1, RegOrLit1),
    LEA(RegOrLit1, RegOrLit1, RegOrLit1),
    WSA(RegOrLit1, RegOrLit1, RegOrLit1),
    WEA(RegOrLit1, RegOrLit1, RegOrLit1),
    SRM(RegOrLit1, RegOrLit1, Reg),
    PUSH(RegOrLit2),
    POP(Reg),
    CALL(RegOrLit2),
    HWD(Reg, RegOrLit1, RegOrLit1),
    CYCLES(Reg),
    HALT(),
    RESET(),
}

impl Instr {
    /// Try to decode an assembly instruction
    pub fn decode(bytes: [u8; 4]) -> Result<Instr, InstrDecodingError> {
        let opcode = bytes[0] >> 3;

        let (arg_reg, arg_reg_or_lit_1, arg_reg_or_lit_2) = {
            let mut _decode_reg = move |param: usize| {
                Reg::from_code(bytes[param]).map_err(|()| InstrDecodingError::UnknownRegister {
                    param: param - 1,
                    code: bytes[param],
                })
            };

            (
                move |param: usize| _decode_reg(param),
                move |param: usize| {
                    if bytes[0] & (1 << (3 - param)) == 0 {
                        Ok(RegOrLit1::lit(bytes[param]))
                    } else {
                        Ok(RegOrLit1::reg(_decode_reg(param)?))
                    }
                },
                move |param: usize| {
                    if bytes[0] & (1 << (3 - param)) == 0 {
                        Ok(RegOrLit2::lit(u16::from_be_bytes([
                            bytes[param],
                            bytes[param + 1],
                        ])))
                    } else {
                        Ok(RegOrLit2::reg(_decode_reg(param + 1)?))
                    }
                },
            )
        };

        // Decode the instruction based on its opcode
        match opcode {
            0x01 => Ok(Self::CPY(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x02 => Ok(Self::EX(arg_reg(1)?, arg_reg(2)?)),
            0x03 => Ok(Self::ADD(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x04 => Ok(Self::SUB(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x05 => Ok(Self::MUL(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x06 => Ok(Self::DIV(arg_reg(1)?, arg_reg_or_lit_1(2)?, arg_reg_or_lit_1(3)?)),
            0x07 => Ok(Self::MOD(arg_reg(1)?, arg_reg_or_lit_1(2)?, arg_reg_or_lit_1(3)?)),
            0x08 => Ok(Self::AND(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x09 => Ok(Self::BOR(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x0A => Ok(Self::XOR(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x0B => Ok(Self::LSH(arg_reg(1)?, arg_reg_or_lit_1(2)?)),
            0x0C => Ok(Self::RSH(arg_reg(1)?, arg_reg_or_lit_1(2)?)),
            0x0D => Ok(Self::CMP(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x0E => Ok(Self::JMP(arg_reg_or_lit_2(1)?)),
            0x0F => Ok(Self::LSM(arg_reg_or_lit_2(1)?)),
            0x10 => Ok(Self::ITR(arg_reg_or_lit_1(1)?)),
            0x11 => Ok(Self::IF(arg_reg_or_lit_1(1)?)),
            0x12 => Ok(Self::IFN(arg_reg_or_lit_1(1)?)),
            0x13 => Ok(Self::IF2(arg_reg_or_lit_1(1)?, arg_reg_or_lit_1(2)?, arg_reg_or_lit_1(3)?)),
            0x14 => Ok(Self::LSA(arg_reg(1)?, arg_reg_or_lit_1(2)?, arg_reg_or_lit_1(3)?)),
            0x15 => Ok(Self::LEA(arg_reg_or_lit_1(1)?, arg_reg_or_lit_1(2)?, arg_reg_or_lit_1(3)?)),
            0x16 => Ok(Self::WSA(arg_reg_or_lit_1(1)?, arg_reg_or_lit_1(2)?, arg_reg_or_lit_1(3)?)),
            0x17 => Ok(Self::WEA(arg_reg_or_lit_1(1)?, arg_reg_or_lit_1(2)?, arg_reg_or_lit_1(3)?)),
            0x18 => Ok(Self::SRM(arg_reg_or_lit_1(1)?, arg_reg_or_lit_1(2)?, arg_reg(3)?)),
            0x19 => Ok(Self::PUSH(arg_reg_or_lit_2(1)?)),
            0x1A => Ok(Self::POP(arg_reg(1)?)),
            0x1B => Ok(Self::CALL(arg_reg_or_lit_2(1)?)),
            0x1C => Ok(Self::HWD(arg_reg(1)?, arg_reg_or_lit_1(2)?, arg_reg_or_lit_1(3)?)),
            0x1D => Ok(Self::CYCLES(arg_reg(1)?)),
            0x1E => Ok(Self::HALT()),
            0x1F => Ok(Self::RESET()),
            _ => Err(InstrDecodingError::UnknownOpCode { opcode }),
        }
    }

    /// Encode the instruction as a set of 4 bytes
    pub fn encode(&self) -> [u8; 4] {
        let mut r: Vec<bool> = vec![];
        let mut p: Vec<u8> = vec![];

        macro_rules! doo {
            // Declare which parameters are registers
            (r $($is_reg: expr),*) => {{ r = vec![ $( $is_reg ),* ]; }};
            // Push register parameters
            (er $($reg: expr),*) => {{ $( p.push($reg.code()) );* }};
            // Push a parameter's value (register or constant)
            (roc $($val: expr),*) => {{ $( p.extend_from_slice(&$val.value().to_be_bytes()) );* }};
        }

        let opcode = match self {
            Self::CPY(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x01
            }

            Self::EX(a, b) => {
                doo!(r true, true);
                doo!(er a, b);
                0x02
            }

            Self::ADD(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x03
            }

            Self::SUB(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x04
            }

            Self::MUL(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x05
            }

            Self::DIV(a, b, c) => {
                doo!(r true, b.is_reg(), c.is_reg());
                doo!(er a);
                doo!(roc b, c);
                0x06
            }

            Self::MOD(a, b, c) => {
                doo!(r true, b.is_reg(), c.is_reg());
                doo!(er a);
                doo!(roc b, c);
                0x07
            }

            Self::AND(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x08
            }

            Self::BOR(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x09
            }

            Self::XOR(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x0A
            }

            Self::LSH(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x0B
            }

            Self::RSH(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x0C
            }

            Self::CMP(a, b) => {
                doo!(r true, b.is_reg());
                doo!(er a);
                doo!(roc b);
                0x0D
            }

            Self::JMP(a) => {
                doo!(r a.is_reg());
                doo!(roc a);
                0x0E
            }

            Self::LSM(a) => {
                doo!(r a.is_reg());
                doo!(roc a);
                0x0F
            }

            Self::ITR(a) => {
                doo!(r a.is_reg());
                doo!(roc a);
                0x10
            }

            Self::IF(a) => {
                doo!(r a.is_reg());
                doo!(roc a);
                0x11
            }

            Self::IFN(a) => {
                doo!(r a.is_reg());
                doo!(roc a);
                0x12
            }

            Self::IF2(a, b, c) => {
                doo!(r a.is_reg(), b.is_reg(), c.is_reg());
                doo!(roc a, b, c);
                0x13
            }

            Self::LSA(a, b, c) => {
                doo!(r true, b.is_reg(), c.is_reg());
                doo!(er a);
                doo!(roc b, c);
                0x14
            }

            Self::LEA(a, b, c) => {
                doo!(r a.is_reg(), b.is_reg(), c.is_reg());
                doo!(roc a, b, c);
                0x15
            }

            Self::WSA(a, b, c) => {
                doo!(r a.is_reg(), b.is_reg(), c.is_reg());
                doo!(roc a, b, c);
                0x16
            }

            Self::WEA(a, b, c) => {
                doo!(r a.is_reg(), b.is_reg(), c.is_reg());
                doo!(roc a, b, c);
                0x17
            }

            Self::SRM(a, b, c) => {
                doo!(r a.is_reg(), b.is_reg(), true);
                doo!(roc a, b);
                doo!(er c);
                0x18
            }

            Self::PUSH(a) => {
                doo!(r a.is_reg());
                doo!(roc a);
                0x19
            }

            Self::POP(a) => {
                doo!(r true);
                doo!(er a);
                0x1A
            }

            Self::CALL(a) => {
                doo!(r a.is_reg());
                doo!(roc a);
                0x1B
            }

            Self::HWD(a, b, c) => {
                doo!(r true, b.is_reg(), c.is_reg());
                doo!(er a);
                doo!(roc b, c);
                0x1C
            }

            Self::CYCLES(a) => {
                doo!(r true);
                doo!(er a);
                0x1D
            }

            Self::HALT() => 0x1E,
            Self::RESET() => 0x1F,
        };

        assert!(
            r.len() <= 3,
            "Internal error: more than 3 serialized parameters"
        );
        assert!(
            p.len() <= 3,
            "Internal error: serialized parameters length exceed 3 bytes"
        );

        r.resize(3, false);
        p.resize(3, 0);

        [
            (opcode << 3)
                + if r[0] { 1 << 2 } else { 0 }
                + if r[1] { 1 << 1 } else { 0 }
                + if r[2] { 1 } else { 0 },
            p[0],
            p[1],
            p[2],
        ]
    }

    /// Encode the instruction as a single word
    pub fn encode_word(&self) -> u32 {
        u32::from_be_bytes(self.encode())
    }
}

/// Instruction decoding error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstrDecodingError {
    /// The source's length is not a multiple of 4 bytes
    SourceNotMultipleOf4Bytes,
    /// An unknown opcode was found
    UnknownOpCode { opcode: u8 },
    /// An unknown register code was used in a parameter
    UnknownRegister { param: usize, code: u8 },
}
