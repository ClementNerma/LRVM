use super::cst;
use super::{ArFlag, DivMode, HwInfo, If2Cond, Reg, RegOrLit1, RegOrLit2};
use std::fmt;

/// Native assembly instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instr {
    Cpy(Reg, RegOrLit2),
    Ex(Reg, Reg),
    Add(Reg, RegOrLit2),
    Sub(Reg, RegOrLit2),
    Mul(Reg, RegOrLit2),
    Div(Reg, RegOrLit1, RegOrLit1),
    Mod(Reg, RegOrLit1, RegOrLit1),
    And(Reg, RegOrLit2),
    Bor(Reg, RegOrLit2),
    Xor(Reg, RegOrLit2),
    Shl(Reg, RegOrLit1),
    Shr(Reg, RegOrLit1),
    Cmp(Reg, RegOrLit2),
    Jpr(RegOrLit2),
    Lsm(RegOrLit2),
    Itr(RegOrLit1),
    If(RegOrLit1),
    IfN(RegOrLit1),
    If2(RegOrLit1, RegOrLit1, RegOrLit1),
    Lsa(Reg, RegOrLit1, RegOrLit1),
    Lea(RegOrLit1, RegOrLit1, RegOrLit1),
    Wsa(RegOrLit1, RegOrLit1, RegOrLit1),
    Wea(RegOrLit1, RegOrLit1, RegOrLit1),
    Srm(RegOrLit1, RegOrLit1, Reg),
    Push(RegOrLit2),
    Pop(Reg),
    Call(RegOrLit2),
    Hwd(Reg, RegOrLit1, RegOrLit1),
    Cycles(Reg),
    Halt(),
    Reset(RegOrLit1),
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
                        Ok(RegOrLit2::reg(_decode_reg(param)?))
                    }
                },
            )
        };

        // Decode the instruction based on its opcode
        match opcode {
            0x01 => Ok(Self::Cpy(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x02 => Ok(Self::Ex(arg_reg(1)?, arg_reg(2)?)),
            0x03 => Ok(Self::Add(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x04 => Ok(Self::Sub(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x05 => Ok(Self::Mul(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x06 => Ok(Self::Div(
                arg_reg(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg_or_lit_1(3)?,
            )),
            0x07 => Ok(Self::Mod(
                arg_reg(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg_or_lit_1(3)?,
            )),
            0x08 => Ok(Self::And(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x09 => Ok(Self::Bor(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x0A => Ok(Self::Xor(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x0B => Ok(Self::Shl(arg_reg(1)?, arg_reg_or_lit_1(2)?)),
            0x0C => Ok(Self::Shr(arg_reg(1)?, arg_reg_or_lit_1(2)?)),
            0x0D => Ok(Self::Cmp(arg_reg(1)?, arg_reg_or_lit_2(2)?)),
            0x0E => Ok(Self::Jpr(arg_reg_or_lit_2(1)?)),
            0x0F => Ok(Self::Lsm(arg_reg_or_lit_2(1)?)),
            0x10 => Ok(Self::Itr(arg_reg_or_lit_1(1)?)),
            0x11 => Ok(Self::If(arg_reg_or_lit_1(1)?)),
            0x12 => Ok(Self::IfN(arg_reg_or_lit_1(1)?)),
            0x13 => Ok(Self::If2(
                arg_reg_or_lit_1(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg_or_lit_1(3)?,
            )),
            0x14 => Ok(Self::Lsa(
                arg_reg(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg_or_lit_1(3)?,
            )),
            0x15 => Ok(Self::Lea(
                arg_reg_or_lit_1(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg_or_lit_1(3)?,
            )),
            0x16 => Ok(Self::Wsa(
                arg_reg_or_lit_1(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg_or_lit_1(3)?,
            )),
            0x17 => Ok(Self::Wea(
                arg_reg_or_lit_1(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg_or_lit_1(3)?,
            )),
            0x18 => Ok(Self::Srm(
                arg_reg_or_lit_1(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg(3)?,
            )),
            0x19 => Ok(Self::Push(arg_reg_or_lit_2(1)?)),
            0x1A => Ok(Self::Pop(arg_reg(1)?)),
            0x1B => Ok(Self::Call(arg_reg_or_lit_2(1)?)),
            0x1C => Ok(Self::Hwd(
                arg_reg(1)?,
                arg_reg_or_lit_1(2)?,
                arg_reg_or_lit_1(3)?,
            )),
            0x1D => Ok(Self::Cycles(arg_reg(1)?)),
            0x1E => Ok(Self::Halt()),
            0x1F => Ok(Self::Reset(arg_reg_or_lit_1(1)?)),
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
            Self::Cpy(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x01
            }

            Self::Ex(a, b) => {
                regs!(true, true);
                push!(regs a, b);
                0x02
            }

            Self::Add(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x03
            }

            Self::Sub(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x04
            }

            Self::Mul(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x05
            }

            Self::Div(a, b, c) => {
                regs!(true, b.is_reg(), c.is_reg());
                push!(regs a);
                push!(regs_or_lit b, c);
                0x06
            }

            Self::Mod(a, b, c) => {
                regs!(true, b.is_reg(), c.is_reg());
                push!(regs a);
                push!(regs_or_lit b, c);
                0x07
            }

            Self::And(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x08
            }

            Self::Bor(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x09
            }

            Self::Xor(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x0A
            }

            Self::Shl(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x0B
            }

            Self::Shr(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x0C
            }

            Self::Cmp(a, b) => {
                regs!(true, b.is_reg());
                push!(regs a);
                push!(regs_or_lit b);
                0x0D
            }

            Self::Jpr(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x0E
            }

            Self::Lsm(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x0F
            }

            Self::Itr(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x10
            }

            Self::If(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x11
            }

            Self::IfN(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x12
            }

            Self::If2(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), c.is_reg());
                push!(regs_or_lit a, b, c);
                0x13
            }

            Self::Lsa(a, b, c) => {
                regs!(true, b.is_reg(), c.is_reg());
                push!(regs a);
                push!(regs_or_lit b, c);
                0x14
            }

            Self::Lea(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), c.is_reg());
                push!(regs_or_lit a, b, c);
                0x15
            }

            Self::Wsa(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), c.is_reg());
                push!(regs_or_lit a, b, c);
                0x16
            }

            Self::Wea(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), c.is_reg());
                push!(regs_or_lit a, b, c);
                0x17
            }

            Self::Srm(a, b, c) => {
                regs!(a.is_reg(), b.is_reg(), true);
                push!(regs_or_lit a, b);
                push!(regs c);
                0x18
            }

            Self::Push(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x19
            }

            Self::Pop(a) => {
                regs!(true);
                push!(regs a);
                0x1A
            }

            Self::Call(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x1B
            }

            Self::Hwd(a, b, c) => {
                regs!(true, b.is_reg(), c.is_reg());
                push!(regs a);
                push!(regs_or_lit b, c);
                0x1C
            }

            Self::Cycles(a) => {
                regs!(true);
                push!(regs a);
                0x1D
            }

            Self::Halt() => 0x1E,

            Self::Reset(a) => {
                regs!(a.is_reg());
                push!(regs_or_lit a);
                0x1F
            }
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
    #[allow(clippy::cognitive_complexity)]
    pub fn to_lasm(self) -> String {
        match self {
            Self::Cpy(a, b) => match (a, b) {
                (Reg::pc, _) => format!("jp {}", b.to_lasm()),
                (_, RegOrLit2::Lit(0)) => format!("zro {}", a.to_lasm()),
                (_, _) => format!("cpy {}, {}", a.to_lasm(), b.to_lasm()),
            },

            Self::Ex(a, b) => format!("ex {}, {}", a.to_lasm(), b.to_lasm()),

            Self::Add(a, b) => match b {
                RegOrLit2::Lit(1) => format!("inc {}", a.to_lasm()),
                _ => format!("add {}, {}", a.to_lasm(), b.to_lasm()),
            },

            Self::Sub(a, b) => match b {
                RegOrLit2::Lit(1) => format!("dec {}", a.to_lasm()),
                _ => format!("sub {}, {}", a.to_lasm(), b.to_lasm()),
            },

            Self::Mul(a, b) => match b {
                RegOrLit2::Lit(0) => format!("zro {}", a.to_lasm()),
                _ => format!("mul {}, {}", a.to_lasm(), b.to_lasm()),
            },

            Self::Div(a, b, RegOrLit1::Reg(c)) => {
                format!("div {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm())
            }

            Self::Mod(a, b, RegOrLit1::Reg(c)) => {
                format!("mod {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm())
            }

            Self::Div(a, b, RegOrLit1::Lit(mode)) | Self::Mod(a, b, RegOrLit1::Lit(mode)) => {
                format!(
                    "{} {}, {}, {}",
                    if matches!(self, Self::Div(_, _, _)) {
                        "div"
                    } else {
                        "mod"
                    },
                    a.to_lasm(),
                    b.to_lasm(),
                    match DivMode::decode(mode) {
                        Ok(mode) => mode.to_lasm(),
                        Err(()) => format!("{:#010b} ; Warning: invalid division mode", mode),
                    }
                )
            }

            Self::And(a, b) => match b {
                RegOrLit2::Lit(0) => format!("zro {}", a.to_lasm()),
                _ => format!("and {}, {}", a.to_lasm(), b.to_lasm()),
            },

            Self::Bor(a, b) => format!("bor {}, {}", a.to_lasm(), b.to_lasm()),

            Self::Xor(a, b) => match b {
                RegOrLit2::Reg(a) => format!("zro {}", a.to_lasm()),
                _ => format!("xor {}, {}", a.to_lasm(), b.to_lasm()),
            },

            Self::Shl(a, b) => format!("shl {}, {}", a.to_lasm(), b.to_lasm()),

            Self::Shr(a, b) => format!("shr {}, {}", a.to_lasm(), b.to_lasm()),

            Self::Cmp(a, b) => format!("cmp {}, {}", a.to_lasm(), b.to_lasm()),

            Self::Jpr(a) => format!("jpr {}", a.to_lasm_signed()), // Be aware of the ".to_lasm_signed()" here as JPR takes a signed argument

            Self::Lsm(a) => format!("lsm {}", a.to_lasm()),

            Self::Itr(a) => format!("itr {}", a.to_lasm()),

            Self::If(a) => match a {
                RegOrLit1::Reg(reg) => format!("if {}", reg.to_lasm()),
                RegOrLit1::Lit(lit) => match ArFlag::decode(lit) {
                    Ok(flag) => match flag {
                        ArFlag::Zero => "ifeq".to_string(),
                        ArFlag::Carry => "ifls".to_string(),
                        _ => format!("if {}", flag.to_lasm()),
                    },
                    Err(()) => format!("if {:#X} ; Warning: unknown flag", lit),
                },
            },

            Self::IfN(a) => match a {
                RegOrLit1::Reg(reg) => format!("ifn {}", reg.to_lasm()),
                RegOrLit1::Lit(lit) => match ArFlag::decode(lit) {
                    Ok(flag) => match flag {
                        ArFlag::Zero => "ifnq".to_string(),
                        ArFlag::Carry => "ifge".to_string(),
                        _ => format!("ifn {}", flag.to_lasm()),
                    },
                    Err(()) => format!("ifn {:#X} ; Warning: unknown flag", lit),
                },
            },

            Self::If2(a, b, c) => {
                let mut warns = vec![];

                enum Pos {
                    Left,
                    Right,
                }

                let mut decode_flag = |flag: RegOrLit1, pos: Pos| -> String {
                    flag.to_lasm_with(|lit| {
                        ArFlag::decode(lit)
                            .map(|lit| lit.to_lasm().to_string())
                            .unwrap_or_else(|()| {
                                warns.push(match pos {
                                    Pos::Left => "invalid left flag",
                                    Pos::Right => "invalid right flag",
                                });
                                format!("{:#004X}", lit)
                            })
                    })
                };

                let no_warn = match (a, b, c) {
                    (_, _, RegOrLit1::Lit(cond)) => match If2Cond::decode(cond) {
                        Ok(If2Cond::Nor)
                            if matches!(a, RegOrLit1::Lit(cst::ZF))
                                && matches!(b, RegOrLit1::Lit(cst::CF)) =>
                        {
                            "ifgt".to_string()
                        }
                        Ok(If2Cond::Or)
                            if matches!(a, RegOrLit1::Lit(cst::ZF))
                                && matches!(b, RegOrLit1::Lit(cst::CF)) =>
                        {
                            "ifle".to_string()
                        }

                        Ok(cond) => format!(
                            "{} {}, {}",
                            match cond {
                                If2Cond::Or => "ifor",
                                If2Cond::And => "ifand",
                                If2Cond::Xor => "ifxor",
                                If2Cond::Nor => "ifnor",
                                If2Cond::Nand => "ifnand",
                                If2Cond::Left => "ifleft",
                                If2Cond::Right => "ifright",
                            },
                            decode_flag(a, Pos::Left),
                            decode_flag(b, Pos::Right)
                        ),

                        Err(()) => {
                            let a = decode_flag(a, Pos::Left);
                            let b = decode_flag(b, Pos::Right);
                            warns.push("invalid condition");
                            format!("if2 {}, {}, {:#004X}", a, b, cond)
                        }
                    },
                    (RegOrLit1::Reg(a), RegOrLit1::Reg(b), RegOrLit1::Reg(c)) => {
                        format!("if2 {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm())
                    }
                    (_, _, RegOrLit1::Reg(cond)) => format!(
                        "if2 {}, {}, {}",
                        decode_flag(a, Pos::Left),
                        decode_flag(b, Pos::Right),
                        cond.to_lasm()
                    ),
                };

                if warns.is_empty() {
                    no_warn
                } else {
                    format!("{} ; {}", no_warn, warns.join(", "))
                }
            }

            Self::Lsa(a, b, c) => format!("lsa {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),

            Self::Lea(a, b, c) => format!("lea {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),

            Self::Wsa(a, b, c) => format!("wsa {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),

            Self::Wea(a, b, c) => format!("wea {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),

            Self::Srm(a, b, c) => format!("srm {}, {}, {}", a.to_lasm(), b.to_lasm(), c.to_lasm()),

            Self::Push(a) => format!("push {}", a.to_lasm()),

            Self::Pop(a) => match a {
                Reg::pc => "ret".to_string(),
                _ => format!("pop {}", a.to_lasm()),
            },

            Self::Call(a) => format!("call {}", a.to_lasm()),

            Self::Hwd(a, b, c) => format!(
                "hwd {}, {}, {}",
                a.to_lasm(),
                b.to_lasm(),
                c.to_lasm_with(|lit| match HwInfo::decode(lit) {
                    Ok(info) => info.to_lasm().to_string(),
                    Err(()) => format!("{:#X} ; Warning: unknown hardware information", lit),
                })
            ),

            Self::Cycles(a) => format!("cycles {}", a.to_lasm()),

            Self::Halt() => "halt".to_string(),

            Self::Reset(a) => format!("reset {}", a.to_lasm()),
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

impl fmt::Display for InstrDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::SourceNotMultipleOf4Bytes => "The provided program's length is not a multiple of 4 bytes (unaligned instructions)".to_string(),
            Self::UnknownOpCode { opcode } => format!("Unknown opcode: {:#004X}", opcode),
            Self::UnknownRegister { param, code } => format!("Parameter {} uses unknown register: {:#004X}", param + 1, code)
        })
    }
}
