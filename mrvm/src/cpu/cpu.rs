use std::sync::{Arc, Mutex};
use crate::board::MappedMemory;
use crate::mmu::MMU;
use super::Registers;

/// Central Processing Unit (CPU)
pub struct CPU {
    /// Registers (available from the outside of the crate)
    pub regs: Registers,
    /// MMU (available from the outside of the crate)
    pub mmu: MMU,
    /// Current cycle count (goes back to 0 after reaching maximum)
    cycles: u32,
    /// Is the CPU halted?
    halted: bool,
    /// (Internal) Did the current cycle change the PC register?
    _cycle_changed_pc: bool
}

impl CPU {
    /// Create a new CPU using an existing mapped memory (must be the same one the motherboard this CPU will be connected to uses).
    pub fn new(mem: Arc<Mutex<MappedMemory>>) -> Self {
        Self {
            regs: Registers::new(),
            mmu: MMU::new(mem),
            cycles: 0,
            halted: true,
            _cycle_changed_pc: false
        }
    }

    /// Hanldle a RESET signal from the motherboard
    pub fn reset(&mut self) {
        self.regs.reset();
        self.cycles = 0;
        self.halted = false;
        self._cycle_changed_pc = true;
    }

    /// Run the next instruction.
    /// Returns `Ok(true)` if the instruction run correctly, `Ok(false)` if the CPU is currently halted and an `Err()` if an exception occurred.
    pub fn next(&mut self) -> Result<bool, Ex> {
        // Do not run if the CPU is halted
        if self.halted {
            return Ok(false);
        }

        // Cycle goes back to 0 when overflowing
        self.cycles = self.cycles.wrapping_add(1);

        // Get the instruction to run
        let instr = self.exec_mem(self.regs.pc)?.to_be_bytes();

        // Get its opcode (5 first bits of the first byte)
        let opcode = instr[0] >> 3;
        // Determine which parameters are registers by reading the 3 last bits of the first byte
        let opregs = [instr[0] & 0b100 != 0, instr[0] & 0b10 != 0, instr[0] & 0b1 != 0];
        // Get the instruction's parameters
        let params = [instr[1], instr[2], instr[3]];

        // Used to determine if the current cycle changed PC (see below)
        self._cycle_changed_pc = false;

        // Decode a register-or-literal parameter
        macro_rules! __reg_or_lit {
            // '$param' is the parameter first byte's index (starting from 0)
            // '$value' is the decoded parameter's value (combined bytes of params[$param..=$param+<param length>])
            // If the specified parameter is marked as being a register, the provided value is considered to be a register ID and the we try
            // to read its value. Else, the provided value is a plain number.
            (with_val $param: expr, $value: expr) => {
                if opregs[$param] { self.read_reg(params[$param])? } else { $value }
            };
            // 1-byte long parameters
            ($param: expr, 1) => {
                __reg_or_lit!(with_val $param, params[$param].into())
            };
            // 2-bytes long parameters
            ($param: expr, 2) => {
                __reg_or_lit!(with_val $param, u16::from_be_bytes([ params[$param], params[$param + 1] ]).into())
            };
        }

        // Decode the opcode's arguments
        // 'REG' = parameter is always a register
        // 'REG_OR_LIT_1' = parameter is either a register or a 1-byte literal
        // 'REG_OR_LIT_2' = parameter is either a register or a 2-bytes literal
        macro_rules! args {
            (REG) => { params[0] };
            (REG_OR_LIT_1) => { __reg_or_lit!(0, 1) };
            (REG_OR_LIT_2) => { __reg_or_lit!(0, 2) };
            (REG, REG) => { (params[0], params[1]) };
            (REG, REG_OR_LIT_1) => { (params[0], __reg_or_lit!(1, 1)) };
            (REG, REG_OR_LIT_2) => { (params[0], __reg_or_lit!(1, 2)) };
            (REG_OR_LIT_1, REG) => { (__reg_or_lit!(0, 1), params[1]) };
            (REG_OR_LIT_1, REG_OR_LIT_1) => { (__reg_or_lit!(0, 1), __reg_or_lit!(1, 1)) };
            (REG_OR_LIT_1, REG_OR_LIT_2) => { (__reg_or_lit!(0, 1), __reg_or_lit!(1, 2)) };
            (REG, REG, REG) => { (params[0], params[1], params[2]) };
            (REG, REG, REG_OR_LIT_1) => { (params[0], params[1], __reg_or_lit!(2, 1)) };
            (REG, REG_OR_LIT_1, REG) => { (params[0], __reg_or_lit!(1, 1), params[2]) };
            (REG, REG_OR_LIT_1, REG_OR_LIT_1) => { (params[0], __reg_or_lit!(1, 1), __reg_or_lit!(2, 1)) };
            (REG_OR_LIT_1, REG, REG) => { (__reg_or_lit!(0, 1), params[1], params[2]) };
            (REG_OR_LIT_1, REG, REG_OR_LIT_1) => { (__reg_or_lit!(0, 1), params[1], __reg_or_lit!(2, 1)) };
            (REG_OR_LIT_1, REG_OR_LIT_1, REG) => { (__reg_or_lit!(0, 1), __reg_or_lit!(1, 1), params[2]) };
            (REG_OR_LIT_1, REG_OR_LIT_1, REG_OR_LIT_1) => { (__reg_or_lit!(0, 1), __reg_or_lit!(1, 1), __reg_or_lit!(2, 1)) };
        }

        // Wrap the opcode's matching block in a closure in order to be able to interrupt the flow with the `?` operator
        // This makes this part a **lot** more readable
        let ok_or_exception = (|| -> Result<(), Ex> {
            // Run the instruction based on its opcode
            match opcode {
                // <Unknown instruction>
                0x00 => {
                    Err(self.exception(0x01, Some(opcode.into())))
                },

                // CPY
                0x01 => {
                    let (reg_dest, value) = args!(REG, REG_OR_LIT_2);
                    self.write_reg(reg_dest, value)
                },

                // EX
                0x02 => {
                    let (reg_a, reg_b) = args!(REG, REG);
                    let pivot_a = self.read_reg(reg_a)?;

                    self.read_reg(reg_b).and_then(|reg_b_value| {
                        self.write_reg(reg_a, reg_b_value)?;
                        self.write_reg(reg_b, pivot_a)
                    })
                },

                // ADD, SUB, MUL, AND, BOR, XOR, LSH, RSH
                0x03..=0x05 | 0x08..=0x0C => {
                    let (reg, value) = if opcode != 0x0B && opcode != 0x0C { args!(REG, REG_OR_LIT_2) } else { args!(REG, REG_OR_LIT_1) };
                    let reg_value = self.read_reg(reg)?;

                    let compute = self.compute(reg_value, value.into(), match opcode {
                        0x03 => Op::Add,
                        0x04 => Op::Sub,
                        0x05 => Op::Mul,
                        0x08 => Op::And,
                        0x09 => Op::Bor,
                        0x0A => Op::Xor,
                        0x0B => Op::Lsh,
                        0x0C => Op::Rsh,
                        _ => unreachable!()
                    })?;

                    self.write_reg(reg, compute)
                },

                // DIV
                0x06 => {
                    let (reg, value, mode) = args!(REG, REG_OR_LIT_1, REG_OR_LIT_1);
                    let reg_value = self.read_reg(reg)?;

                    let compute = self.compute(reg_value, value, Op::Div { mode: (mode & 0xFF) as u8 })?;
                    self.write_reg(reg, compute)
                },

                // MOD
                0x07 => {
                    let (reg, value, mode) = args!(REG, REG_OR_LIT_1, REG_OR_LIT_1);
                    let reg_value = self.read_reg(reg)?;

                    let compute = self.compute(reg_value, value, Op::Mod { mode: (mode & 0xFF) as u8 })?;
                    self.write_reg(reg, compute)
                },

                // CMP
                0x0D => {
                    let (reg, value) = args!(REG, REG_OR_LIT_2);
                    let reg_value = self.read_reg(reg)?;

                    self.compute(reg_value, value, Op::Sub)?;
                    Ok(())
                },

                // JMP
                0x0E => {
                    let bytes = args!(REG_OR_LIT_2) as i16;

                    self.regs.pc = (self.regs.pc as i32).wrapping_add(bytes.into()) as u32;
                    self._cycle_changed_pc = true;
                    Ok(())
                },

                // LSM
                0x0F => {
                    if self.sv_mode() {
                        self.regs.pc = args!(REG_OR_LIT_2);
                        self.regs.smt = 0;
                        Ok(())
                    } else {
                        Err(self.exception(0x08, Some(opcode.into())))
                    }
                },

                // ITR
                0x10 => {
                    let itr_code = args!(REG_OR_LIT_1);

                    Err(self.exception(0xAA, Some(itr_code as u16)))
                },

                // IF
                0x11 | 0x12 => {
                    let flag: u32 = args!(REG_OR_LIT_1).into();

                    if (self.regs.af & (1 << flag) == 1) ^ (opcode == 0x11) {
                        self.regs.pc = self.regs.pc.wrapping_add(8);
                        self._cycle_changed_pc = true;
                    }

                    Ok(())
                },

                // IFAND, IFOR, IFNOR, IFLFT
                0x13..=0x16 => {
                    let (flag_a, flag_b) = args!(REG_OR_LIT_1, REG_OR_LIT_1);
                    let (flag_a, flag_b) = (self.regs.af & (1 << flag_a) == 0, self.regs.af & (1 << flag_b) == 0);

                    let cond = match opcode {
                        0x13 => flag_a && flag_b,
                        0x14 => flag_a || flag_b,
                        0x15 => !flag_a && !flag_b,
                        0x16 => flag_a && !flag_b,
                        _ => unreachable!()
                    };

                    if !cond {
                        self.regs.pc = self.regs.pc.wrapping_add(8);
                    }

                    self._cycle_changed_pc = true;
                    Ok(())
                },

                // LDA
                0x17 => {
                    let (reg_dest, v_addr) = args!(REG, REG_OR_LIT_2);

                    let word = self.mem_read(v_addr)?;
                    self.write_reg(reg_dest, word)
                },

                // LEA
                0x18 => {
                    let (v_addr, add, mul) = args!(REG_OR_LIT_1, REG_OR_LIT_1, REG_OR_LIT_1);

                    self.regs.avr = self.mem_read(v_addr + add * mul)?;
                    Ok(())
                },

                // WEA
                0x19 => {
                    let (v_addr, add, mul) = args!(REG_OR_LIT_1, REG_OR_LIT_1, REG_OR_LIT_1);

                    self.mem_write(v_addr + add * mul, self.regs.avr)
                },

                // PUSH
                0x1A => {
                    let word = args!(REG_OR_LIT_2);

                    let v_addr = if self.sv_mode() {
                        self.regs.ssp = self.regs.ssp.wrapping_sub(4);
                        self.mem_read(self.regs.ssp)?
                    } else {
                        self.regs.usp = self.regs.usp.wrapping_sub(4);
                        self.mem_read(self.regs.usp)?
                    };

                    self.mem_write(v_addr, word)
                },

                // POP
                0x1B => {
                    let reg_dest = args!(REG);

                    let word = if self.sv_mode() {
                        let word = self.mem_read(self.regs.ssp)?;
                        self.regs.ssp = self.regs.ssp.wrapping_add(4);
                        word
                    } else {
                        let word = self.mem_read(self.regs.usp)?;
                        self.regs.usp = self.regs.usp.wrapping_add(4);
                        word
                    };

                    self.write_reg(reg_dest, word)
                },

                // CALL
                0x1C => {
                    let v_addr = args!(REG_OR_LIT_2);

                    let sp_v_addr = if self.sv_mode() {
                        self.regs.ssp = self.regs.ssp.wrapping_sub(4);
                        self.mem_read(self.regs.ssp)?
                    } else {
                        self.regs.usp = self.regs.usp.wrapping_sub(4);
                        self.mem_read(self.regs.usp)?
                    };

                    self.regs.pc = v_addr;
                    self.mem_write(sp_v_addr, self.regs.pc.wrapping_add(4))
                },

                // CYCLES
                0x1D => {
                    let reg_dest = args!(REG);
                    self.write_reg(reg_dest, self.cycles)
                },

                // HALT
                0x1E => {
                    self.halted = true;
                    Ok(())
                },

                // RESET
                0x1F => {
                    self.reset();
                    Ok(())
                },

                _ => unreachable!("Internal error: processor encountered an instruction with an opcode greater than 0x1F (> 5 bits)")
            }
        })();

        // By default, the program counter (located in the PC register) is incremented of 4 bytes to make the CPU retrieve the next instruction
        //  from the memory's next word.
        // BUT if the current instruction purposedly modified PC, we don't want it to the be modified again.
        // So, we only add 4 to PC if it hasn't been changed by the current instruction.
        if !self._cycle_changed_pc {
            self.regs.pc = self.regs.pc.wrapping_add(4);
        }

        ok_or_exception.map(|()| true)
    }

    /// Check if the CPU is halted
    pub fn halted(&self) -> bool {
        self.halted
    }

    /// Get the number of cycles the CPU run so far
    /// Note that this number goes back to 0 after reaching its maximum (overflow).
    pub fn cycles(&self) -> u32 {
        self.cycles
    }

    /// Try to read a register's value.
    /// Raises an exception if the specified register is only readable in supervisor mode and userland mode is active.
    fn read_reg(&mut self, code: u8) -> Result<u32, Ex> {
        if code >= 0x18 && !self.sv_mode() {
            return Err(self.exception(0x03, Some(code.into())));
        }

        let ucode = usize::from(code);

        match code {
            0x00..=0x07 => Ok(self.regs.a[ucode]),
            0x08..=0x09 => Ok(self.regs.c[ucode - 0x08]),
            0x0A..=0x0C => Ok(self.regs.ac[ucode - 0x0A]),
            0x0D..=0x14 => Ok(self.regs.rr[ucode - 0x0D]),
            0x15 => Ok(self.regs.avr),
            0x16 => Ok(self.regs.pc),
            0x17 => Ok(self.regs.af),
            0x18 => Ok(self.regs.ssp),
            0x19 => Ok(self.regs.usp),
            0x1A => Ok(self.regs.et),
            0x1B => Ok(self.regs.era),
            0x1C => Ok(self.regs.ev),
            0x1D => Ok(self.regs.mtt),
            0x1E => Ok(self.regs.pda),
            0x1F => Ok(self.regs.smt),
            _ => Err(self.exception(0x02, Some(code.into())))
        }
    }

    /// Try to write a register's value.
    /// Raises an exception if the specified register is only writable in supervisor mode and userland mode is active.
    /// Raises an exception if the specified register is not writable.
    fn write_reg(&mut self, code: u8, word: u32) -> Result<(), Ex> {
        let ucode = usize::from(code);

        if code >= 0x17 && !self.sv_mode() {
            return Err(self.exception(0x04, Some(code.into())));
        }

        if code == 0x17 || code == 0x1A || code == 0x1B {
            return Err(self.exception(0x04, Some(code.into())));
        }

        // If we change PC, indicate it has been changed to the CPU won't jump 4 bytes ahead.
        if code == 0x16 {
            self._cycle_changed_pc = true;
        }

        match code {
            0x00..=0x07 => self.regs.a[ucode] = word,
            0x08..=0x09 => self.regs.c[ucode - 0x08] = word,
            0x0A..=0x0C => self.regs.ac[ucode - 0x0A] = word,
            0x0D..=0x14 => self.regs.rr[ucode - 0x0D] = word,
            0x15 => self.regs.avr = word,
            0x16 => self.regs.pc = word,
            0x17 => self.regs.af = word,
            0x18 => self.regs.ssp = word,
            0x19 => self.regs.usp = word,
            0x1A => self.regs.et = word,
            0x1B => self.regs.era = word,
            0x1C => self.regs.ev = word,
            0x1D => self.regs.mtt = word,
            0x1E => self.regs.pda = word,
            0x1F => self.regs.smt = word,
            _ => return Err(self.exception(0x02, Some(code.into())))
        }
        
        Ok(())
    }

    /// Perform a numeric computation and set the arithmetic flags.
    /// Raises an exception if a forbidden operation happens (e.g. division by zero when forbidden by the provided division mode).
    fn compute(&mut self, op1: u32, op2: u32, op: Op) -> Result<u32, Ex> {
        let iop1 = op1 as i32;
        let iop2 = op2 as i32;

        let (result, has_carry, has_overflow) = match op {
            Op::Add => {
                let (result, has_carry) = op1.overflowing_add(op2);
                (result, has_carry, iop1.overflowing_add(iop2).1)
            },

            Op::Sub => {
                let (result, has_carry) = op1.overflowing_sub(op2);
                (result, has_carry, iop1.overflowing_add(iop2).1)
            },

            Op::Mul => {
                let (result, has_carry) = iop1.overflowing_mul(iop2);
                (result as u32, has_carry, has_carry)
            },

            // This one is a bit tricky
            Op::Div { mode } | Op::Mod { mode } => {
                // Must we perform a signed division / modulus?
                let signed = mode & 0b00010000 != 0;

                match (op == Op::Div { mode }, signed, iop1, iop2) {
                    // Division / modulus by zero
                    (_, _, _, 0) => match (mode & 0b00001100) >> 2 {
                        // Forbid
                        0b00 => return Err(self.exception(0x09, None)),
                        // Result in the minimum signed value
                        0b01 => (0x80000000, true, true),
                        // Result in zero
                        0b10 => (0x00000000, true, true),
                        // Result in the maximum signed value
                        0b11 => (0x7FFFFFFF, true, true),
                        _ => unreachable!()
                    },

                    // Maximum signed value divided / moduled by -1 (overflowing multiplication)
                    (_, true, std::i32::MIN, -1) => match (mode & 0b00000011) >> 2 {
                        // Forbid
                        0b00 => return Err(self.exception(0x0A, None)),
                        // Result in the minimum signed value
                        0b01 => (0x80000000, true, true),
                        // Result in zero
                        0b10 => (0x00000000, true, true),
                        // Result in the maximum signed value
                        0b11 => (0x7FFFFFFF, true, true),
                        _ => unreachable!()
                    },

                    // Safe unsigned division
                    (true, true, _, _) => ((iop1 / iop2) as u32, false, false),

                    // Safe unsigned modulus
                    (false, true, _, _) => ((iop1 % iop2) as u32, false, false),

                    // Safe signed division
                    (true, false, _, _) => (op1 / op2, false, false),

                    // Safe signed modulus
                    (false, false, _, _) => (op1 % op2, false, false)
                }
            },

            Op::And => (op1 & op2, false, false),

            Op::Bor => (op1 | op2, false, false),

            Op::Xor => (op1 ^ op2, false, false),

            Op::Lsh => {
                let (result, has_carry) = op1.overflowing_shl(op2);
                (result, has_carry, has_carry)
            },

            Op::Rsh => {
                let (result, has_carry) = op1.overflowing_shr(op2);
                (result, has_carry, has_carry)
            }
        };
        
        // => Compute and assign arithmetic flags to the `af` register

        self.regs.af = 0;

        let flags: [bool; 7] = [
            // Zero
            result == 0,
            // Carry Flag
            has_carry,
            // Overflow Flag
            has_overflow,
            // Sign Flag
            (result >> 31) & 0b1 == 1,
            // Parity Flag
            result & 0b1 == 0,
            // Zero-Upper Flag
            result <= 0xFFFF,
            // Zero-Lower Flag
            (result >> 16) & 0xFFFF == 0
        ];

        for (bit, flag) in flags.iter().enumerate() {
            if *flag {
                self.regs.af += 1 << bit;
            }
        }

        Ok(result)
    }

    /// Check if the CPU is currently in supervisor mode
    fn sv_mode(&self) -> bool {
        self.regs.smt != 0
    }

    /// Raise an exception with the provided `code` and `associated` data.
    /// Returns the related exception object.
    fn exception(&mut self, code: u8, associated: Option<u16>) -> Ex {
        // Assign the Exception Type `et` register.
        self.regs.et =
            (if self.sv_mode() { 1 << 24 } else { 0 }) +
            (u32::from(code) << 16) +
            u32::from(associated.unwrap_or(0));

        // Jump to the Exception Vector address
        self.regs.pc = self.regs.ev;

        // Enable supervisor mode to deal with the exception
        self.regs.smt = 1;

        // Do not forget to indicate we changed PC
        self._cycle_changed_pc = true;

        Ex { code, associated }
    }

    /// Ensure an address is aligned, or raise an exception otherwise.
    fn ensure_aligned(&mut self, v_addr: u32) -> Result<u32, Ex> {
        if v_addr % 4 != 0 {
            Err(self.exception(0x05, Some((v_addr % 4) as u16)))
        } else {
            Ok(v_addr)
        }
    }

    /// Read an address in the mapped memory.
    /// Raises an exception if address is unaligned or if the MMU doesn't accept reading this address in the current mode.
    fn mem_read(&mut self, v_addr: u32) -> Result<u32, Ex> {
        let v_addr = self.ensure_aligned(v_addr)?;
        let mut ex = 0;

        self.mmu.read(&self.regs, v_addr, &mut ex)
            .map_err(|()| self.exception(0x06, Some(v_addr as u16)))
            .and_then(|word| { if ex != 0 { Err(self.exception(0x10, Some(ex))) } else { Ok(word) } })
    }

    /// Write an address in the mapped memory.
    /// Raises an exception if address is unaligned or if the MMU doesn't accept writing this address in the current mode.
    fn mem_write(&mut self, v_addr: u32, word: u32) -> Result<(), Ex> {
        let v_addr = self.ensure_aligned(v_addr)?;
        let mut ex = 0;

        self.mmu.write(&self.regs, v_addr, word, &mut ex)
            .map_err(|()| self.exception(0x07, Some(v_addr as u16)))
            .and_then(|()| { if ex != 0 { Err(self.exception(0x10, Some(ex))) } else { Ok(()) } })
    }

    /// Execute (read) an address in the mapped memory.
    /// Raises an exception if address is unaligned or if the MMU doesn't accept executing this address in the current mode.
    fn exec_mem(&mut self, v_addr: u32) -> Result<u32, Ex> {
        let v_addr = self.ensure_aligned(v_addr)?;
        let mut ex = 0;

        self.mmu.exec(&self.regs, v_addr, &mut ex)
            .map_err(|()| self.exception(0x07, Some(v_addr as u16)))
            .and_then(|word| { if ex != 0 { Err(self.exception(0x10, Some(ex))) } else { Ok(word) } })
    }
}

/// (Internal) Numeric operation
#[derive(PartialEq, Debug)]
enum Op { Add, Sub, Mul, Div { mode: u8 }, Mod { mode: u8 }, And, Bor, Xor, Lsh, Rsh }

/// Occurred exception
pub struct Ex {
    /// Exception's code
    pub code: u8,
    /// Exception's associated data (not all exceptions have some)
    pub associated: Option<u16>
}
