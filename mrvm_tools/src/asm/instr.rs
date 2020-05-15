use super::{Reg, RegOrLit1, RegOrLit2, ArFlag, If2Cond, HwInfo, DivMode};

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
    SHL(Reg, RegOrLit1),
    SHR(Reg, RegOrLit1),
    CMP(Reg, RegOrLit2),
    JPR(RegOrLit2),
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
    RESET(RegOrLit1),
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
            0x0B => Ok(Self::SHL(arg_reg(1)?, arg_reg_or_lit_1(2)?)),
            0x0C => Ok(Self::SHR(arg_reg(1)?, arg_reg_or_lit_1(2)?)),
            0x0D => Ok(Self::CMP(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x0E => Ok(Self::JPR(arg_reg_or_lit_2(1)?)),
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
            0x1F => Ok(Self::RESET(arg_reg_or_lit_1(1)?)),
            _ => Err(InstrDecodingError::UnknownOpCode { opcode }),
        }
    }

    /// Encode the instruction as a set of 4 bytes
    pub fn encode(self) -> [u8; 4] {
        let mut is_reg: Vec<bool> = vec![];
        let mut params: Vec<u8> = vec![];

        // Declare which parameters are registers
        macro_rules! regs { ($($is_reg: expr),*) => {{ is_reg = vec![ $( $is_reg ),* ]; }} }

        // Push parameters
        macro_rules! push {
            // Push register parameters
            (regs $($reg: expr),*) => {{ $( params.push($reg.code()) );* }};
            // Push a parameter's value (register or constant)
            (regs_or_lit $($val: expr),*) => {{ $( params.extend_from_slice(&$val.value().to_be_bytes()) );* }};
        }

        let opcode = match self {
            Self::CPY(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x01
            }

            Self::EX(a, b) => {
                regs!(true, true);
                push!(regs a, b);
                0x02
            }

            Self::ADD(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x03
            }

            Self::SUB(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x04
            }

            Self::MUL(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x05
            }

            Self::DIV(a, b, c) => {
                regs!(true, b.is_reg(), c.is_reg());
                push!(regs a);
                push!(regs_or_lit b, c);
                0x06
            }

            Self::MOD(a, b, c) => {
                regs!(true, b.is_reg(), c.is_reg());
                push!(regs a);
                push!(regs_or_lit b, c);
                0x07
            }

            Self::AND(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x08
            }

            Self::BOR(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x09
            }

            Self::XOR(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x0A
            }

            Self::SHL(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x0B
            }

            Self::SHR(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x0C
            }

            Self::CMP(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x0D
            }

            Self::JPR(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x0E
            }

            Self::LSM(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x0F
            }

            Self::ITR(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x10
            }

            Self::IF(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x11
            }

            Self::IFN(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x12
            }

            Self::IF2(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), c.is_reg());
                push!(regs_or_lit a, b, c);
                0x13
            }

            Self::LSA(a, b, c) => {
                regs!(true, b.is_reg(), c.is_reg());
                push!(regs a);
                push!(regs_or_lit b, c);
                0x14
            }

            Self::LEA(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), c.is_reg());
                push!(regs_or_lit a, b, c);
                0x15
            }

            Self::WSA(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), c.is_reg());
                push!(regs_or_lit a, b, c);
                0x16
            }

            Self::WEA(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), c.is_reg());
                push!(regs_or_lit a, b, c);
                0x17
            }

            Self::SRM(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), true);
                push!(regs_or_lit a, b);
                push!(regs c);
                0x18
            }

            Self::PUSH(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x19
            }

            Self::POP(a) => {
                regs!(true);
                push!(regs a);
                0x1A
            }

            Self::CALL(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x1B
            }

            Self::HWD(a, b, c) => {
                regs!(true, b.is_reg(), c.is_reg());
                push!(regs a);
                push!(regs_or_lit b, c);
                0x1C
            }

            Self::CYCLES(a) => {
                regs!(true);
                push!(regs a);
                0x1D
            }

            Self::HALT() => 0x1E,

            Self::RESET(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x1F
            },
        };

        assert!(
            is_reg.len() <= 3,
            "Internal error: more than 3 serialized parameters"
        );
        assert!(
            params.len() <= 3,
            "Internal error: serialized parameters length exceed 3 bytes"
        );

        is_reg.resize(3, false);
        params.resize(3, 0);

        [
            (opcode << 3)
                + if is_reg[0] { 1 << 2 } else { 0 }
                + if is_reg[1] { 1 << 1 } else { 0 }
                + if is_reg[2] { 1 } else { 0 },
            params[0],
            params[1],
            params[2],
        ]
    }

    /// Encode the instruction as a single word
    pub fn encode_word(self) -> u32 {
        u32::from_be_bytes(self.encode())
    }

    /// Convert the instruction to LASM assembly
    pub fn to_lasm(self) -> String {
        match self {
            Self::CPY(a, b) => format!("cpy {}, {}", a.to_lasm(), b.to_lasm()),
            Self::EX(a, b) => format!("ex {}, {}", a.to_lasm(), b.to_lasm()),
            Self::ADD(a, b) => format!("add {}, {}", a.to_lasm(), b.to_lasm()),
            Self::SUB(a, b) => format!("sub {}, {}", a.to_lasm(), b.to_lasm()),
            Self::MUL(a, b) => format!("mul {}, {}", a.to_lasm(), b.to_lasm()),
            Self::DIV(a, b, RegOrLit1::Reg(c)) => format!("div {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),
            Self::MOD(a, b, RegOrLit1::Reg(c)) => format!("mod {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),
            Self::DIV(a, b, RegOrLit1::Lit(mode)) | Self::MOD(a, b, RegOrLit1::Lit(mode)) => format!(
                "{} {}, {}, {}",
                if matches!(self, Self::DIV(_, _, _)) { "div" } else { "mod" },
                a.to_lasm(),
                b.to_lasm(),
                match DivMode::from_mode(mode) {
                    Ok(mode) => mode.to_lasm(),
                    Err(()) => format!("{:#010b} ; Warning: invalid division mode", mode)
                }
            ),
            Self::AND(a, b) => format!("and {}, {}", a.to_lasm(), b.to_lasm()),
            Self::BOR(a, b) => format!("bor {}, {}", a.to_lasm(), b.to_lasm()),
            Self::XOR(a, b) => format!("xor {}, {}", a.to_lasm(), b.to_lasm()),
            Self::SHL(a, b) => format!("shl {}, {}", a.to_lasm(), b.to_lasm()),
            Self::SHR(a, b) => format!("shr {}, {}", a.to_lasm(), b.to_lasm()),
            Self::CMP(a, b) => format!("cmp {}, {}", a.to_lasm(), b.to_lasm()),
            Self::JPR(a) => format!("jpr {}", a.to_lasm_signed()), // Be aware of the ".to_lasm_signed()" here as JPR takes a signed argument
            Self::LSM(a) => format!("lsm {}", a.to_lasm()),
            Self::ITR(a) => format!("itr {}", a.to_lasm()),
            Self::IF(a) => format!("if {}", a.to_lasm_with(|lit| match ArFlag::decode(lit) {
                Ok(flag) => flag.to_lasm().to_owned(),
                Err(()) => format!("{:#X} ; Warning: unknown flag", lit)
            })),
            Self::IFN(a) => format!("ifn {}", a.to_lasm_with(|lit| match ArFlag::decode(lit) {
                Ok(flag) => flag.to_lasm().to_owned(),
                Err(()) => format!("{:#X} ; Warning: unknown flag", lit)
            })),
            Self::IF2(a, b, c) => {
                let mut warns = vec![];
                let mut decode_flag = |flag: RegOrLit1, warn: &'static str| -> String {
                    flag.to_lasm_with(|lit| match ArFlag::decode(lit) {
                        Ok(flag) => flag.to_lasm().to_owned(),
                        Err(()) => {
                            warns.push(warn);
                            format!("{:#X}", lit)
                        }
                    })
                };

                format!(
                    "if2 {}, {}, {}{}",
                    decode_flag(a, "unknown first condition"),
                    decode_flag(b, "unknown second condition"),
                    c.to_lasm_with(|lit| match If2Cond::decode(lit) {
                        Ok(cond) => cond.to_lasm(),
                        Err(()) => {
                            warns.push("unknown condition");
                            format!("{:#X}", lit)
                        }
                    }),
                    if warns.is_empty() { "".to_owned() } else {
                        format!("; Warning{}: {}", if warns.len() > 1 { "s" } else { "" }, warns.join(" ; "))
                    }
                )
            },
            Self::LSA(a, b, c) => format!("lsa {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),
            Self::LEA(a, b, c) => format!("lea {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),
            Self::WSA(a, b, c) => format!("wsa {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),
            Self::WEA(a, b, c) => format!("wea {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),
            Self::SRM(a, b, c) => format!("srm {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),
            Self::PUSH(a) => format!("push {}", a.to_lasm()),
            Self::POP(a) => format!("pop {}", a.to_lasm()),
            Self::CALL(a) => format!("call {}", a.to_lasm()),
            Self::HWD(a, b, c) => format!("hwd {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm_with(|lit| match HwInfo::decode(lit) {
                Ok(info) => info.to_lasm().to_owned(),
                Err(()) => format!("{:#X} ; Warning: unknown hardware information", lit)
            })),
            Self::CYCLES(a) => format!("cycles {}", a.to_lasm()),
            Self::HALT() => "halt".to_owned(),
            Self::RESET(a) => format!("reset {}", a.to_lasm()),
        }
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
