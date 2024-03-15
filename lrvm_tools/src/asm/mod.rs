//! This module contains a strongly-typed assembler that allows to create programs through sets of instructions
//! and guarantee them to be valid at build time.

pub mod cst;

mod arflag;
mod cond;
mod div_modes;
mod extinstr;
mod hw_infos;
mod instr;
mod prog;
mod prog_word;
mod reg;
mod val;

pub use self::{
    arflag::ArFlag,
    cond::If2Cond,
    div_modes::{DivByZeroMode, DivMode, DivOverflowMode, DivSignMode},
    extinstr::ExtInstr,
    hw_infos::HwInfo,
    instr::{Instr, InstrDecodingError},
    prog::Program,
    prog_word::ProgramWord,
    reg::Reg,
    val::{RegOrLit1, RegOrLit2},
};
