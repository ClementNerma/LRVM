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

pub use arflag::ArFlag;
pub use cond::If2Cond;
pub use div_modes::{DivByZeroMode, DivMode, DivOverflowMode, DivSignMode};
pub use extinstr::ExtInstr;
pub use hw_infos::HwInfo;
pub use instr::{Instr, InstrDecodingError};
pub use prog::Program;
pub use prog_word::ProgramWord;
pub use reg::Reg;
pub use val::{RegOrLit1, RegOrLit2};
