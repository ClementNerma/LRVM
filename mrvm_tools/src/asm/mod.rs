//! This module contains a strongly-typed assembler that allows to create programs through sets of instructions
//! and guarantee them to be valid at build time.

pub mod cst;

mod arflag;
mod reg;
mod val;
mod div_modes;
mod instr;
mod extinstr;
mod prog;

pub use arflag::ArFlag;
pub use reg::Reg;
pub use val::{RegOrLit1, RegOrLit2};
pub use div_modes::{DivSignMode, DivByZeroMode, DivMinByLessOneMode, DivMode};
pub use instr::{Instr, InstrDecodingError};
pub use extinstr::ExtInstr;
pub use prog::Program;