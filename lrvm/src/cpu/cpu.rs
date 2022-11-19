use super::Registers;
use crate::board::HardwareBridge;
use crate::mem::MappedMemory;
use crate::mmu::{MemAction, Mmu};
use std::convert::TryFrom;

/// Central Processing Unit (CPU)
pub struct Cpu {
    /// Registers (available from the outside of the crate)
    pub regs: Registers,
    /// Mapped memory
    pub(crate) mem: MappedMemory,
    /// Memory Management Unit (MMU)
    mmu: Mmu,
    /// Hardware bridge
    hwb: HardwareBridge,
    /// Current cycle count (goes back to 0 after reaching maximum)
    cycles: u128,
    /// Is the CPU halted?
    halted: bool,
    /// (Internal) Did the current cycle change the PC register?
    _cycle_changed_pc: bool,
}

impl Cpu {
    /// Create a new CPU using an existing mapped memory (must be the same one the motherboard this CPU will be connected to uses).
    pub fn new(hwb: HardwareBridge, mem: MappedMemory) -> Self {
        let mut cpu = Self {
            regs: Registers::new(),
            mem,
            mmu: Mmu::new(),
            hwb,
            cycles: 0,
            halted: true,
            _cycle_changed_pc: false,
        };

        // Enable supervisor mode by default
        cpu.regs.smt = 1;

        cpu
    }

    /// Hanldle a RESET signal from the motherboard
    pub fn reset(&mut self) {
        self.regs.reset();
        self.regs.smt = 1;
        self.cycles = 0;
        self.halted = false;
        self._cycle_changed_pc = true;
    }

    /// Run the next instruction. Returns:
    /// * `Ok(true)` if the instruction run correctly
    /// * `Ok(false)` if the CPU is currently halted
    /// * `Err()` if an exception occurred *except for interruptions*
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) {
        // Do not run if the CPU is halted
        if self.halted {
            return;
        }

        // Cycle goes back to 0 when overflowing
        self.cycles = self.cycles.wrapping_add(1);

        // Get the instruction to run
        let instr = match self.mem_exec(self.regs.pc) {
            Err(_) => return,
            Ok(bytes) => bytes.to_be_bytes(),
        };

        // Get its opcode (5 first bits of the first byte)
        let opcode = instr[0] >> 3;
        // Determine which parameters are registers by reading the 3 last bits of the first byte
        let opregs = [
            instr[0] & 0b100 != 0,
            instr[0] & 0b10 != 0,
            instr[0] & 0b1 != 0,
        ];
        // Get the instruction's parameters
        let params = [instr[1], instr[2], instr[3]];

        // Used to determine if the current cycle changed PC (see below)
        self._cycle_changed_pc = false;

        // Run the decoded instruction
        if self.run_instr(opcode, opregs, params).is_err() {
            return;
        }

        // By default, the program counter (located in the PC register) is incremented of 4 bytes to make the CPU retrieve the next instruction
        //  from the memory's next word.
        // BUT if the current instruction purposedly modified PC, we don't want it to the be modified again.
        // So, we only add 4 to PC if it hasn't been changed by the current instruction.
        if !self._cycle_changed_pc {
            self.regs.pc = self.regs.pc.wrapping_add(4);
        }
    }

    /// Check if the CPU is halted
    pub fn halted(&self) -> bool {
        self.halted
    }

    /// Get the number of cycles the CPU run so far
    /// Note that this number goes back to 0 after reaching its maximum (overflow).
    pub fn cycles(&self) -> u128 {
        self.cycles
    }

    /// (Internal) Run a decoded instruction
    /// This method exists for the sole purpose of making the code cleaner in order to make the ".next()" method more understandable
    #[allow(clippy::cognitive_complexity)]
    fn run_instr(&mut self, opcode: u8, opregs: [bool; 3], params: [u8; 3]) -> Result<(), ()> {
        // Decode the opcode's arguments
        // 'REG' = parameter is always a register
        // 'REG_OR_LIT_1' = parameter is either a register or a 1-byte literal
        // 'REG_OR_LIT_2' = parameter is either a register or a 2-bytes literal
        macro_rules! args {
            (REG) => {
                params[0]
            };
            (REG_OR_LIT_1) => {
                __reg_or_lit!(0, 1)
            };
            (REG_OR_LIT_2) => {
                __reg_or_lit!(0, 2)
            };
            (REG, REG) => {
                (params[0], params[1])
            };
            (REG, REG_OR_LIT_1) => {
                (params[0], __reg_or_lit!(1, 1))
            };
            (REG, REG_OR_LIT_2) => {
                (params[0], __reg_or_lit!(1, 2))
            };
            (REG_OR_LIT_1, REG) => {
                (__reg_or_lit!(0, 1), params[1])
            };
            (REG_OR_LIT_1, REG_OR_LIT_1) => {
                (__reg_or_lit!(0, 1), __reg_or_lit!(1, 1))
            };
            (REG_OR_LIT_1, REG_OR_LIT_2) => {
                (__reg_or_lit!(0, 1), __reg_or_lit!(1, 2))
            };
            (REG, REG, REG) => {
                (params[0], params[1], params[2])
            };
            (REG, REG, REG_OR_LIT_1) => {
                (params[0], params[1], __reg_or_lit!(2, 1))
            };
            (REG, REG_OR_LIT_1, REG) => {
                (params[0], __reg_or_lit!(1, 1), params[2])
            };
            (REG, REG_OR_LIT_1, REG_OR_LIT_1) => {
                (params[0], __reg_or_lit!(1, 1), __reg_or_lit!(2, 1))
            };
            (REG_OR_LIT_1, REG, REG) => {
                (__reg_or_lit!(0, 1), params[1], params[2])
            };
            (REG_OR_LIT_1, REG, REG_OR_LIT_1) => {
                (__reg_or_lit!(0, 1), params[1], __reg_or_lit!(2, 1))
            };
            (REG_OR_LIT_1, REG_OR_LIT_1, REG) => {
                (__reg_or_lit!(0, 1), __reg_or_lit!(1, 1), params[2])
            };
            (REG_OR_LIT_1, REG_OR_LIT_1, REG_OR_LIT_1) => {
                (
                    __reg_or_lit!(0, 1),
                    __reg_or_lit!(1, 1),
                    __reg_or_lit!(2, 1),
                )
            };
        }

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

        // Run the instruction based on its opcode
        match opcode {
            // <Unknown instruction>
            0x00 => {
                self.exception(0x01, Some(opcode.into()));
                Err(())
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

            // ADD, SUB, MUL, AND, BOR, XOR, SHL, SHR
            0x03..=0x05 | 0x08..=0x0C => {
                let (reg, mut value) = args!(REG, REG_OR_LIT_2);

                if (opcode == 0x0B || opcode == 0x0C) && !opregs[1] {
                    value >>= 8
                }

                let reg_value = self.read_reg(reg)?;

                let compute = self.compute(reg_value, value, match opcode {
                    0x03 => Op::Add,
                    0x04 => Op::Sub,
                    0x05 => Op::Mul,
                    0x08 => Op::And,
                    0x09 => Op::Bor,
                    0x0A => Op::Xor,
                    0x0B => Op::Shl,
                    0x0C => Op::Shr,
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

            // JPR
            0x0E => {
                let bytes = args!(REG_OR_LIT_2) as i16;

                self.regs.pc = self.regs.pc.wrapping_add(bytes as u32);
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
                    self.exception(0x09, Some(opcode.into()));
                    Err(())
                }
            },

            // ITR
            0x10 => {
                let itr_code = args!(REG_OR_LIT_1);

                self.exception(0xF0, Some(itr_code as u16));
                Ok(())
            },

            // IF, IFN
            0x11 | 0x12 => {
                let flag = args!(REG_OR_LIT_1);

                if flag > 7 {
                    self.exception(0x0C, Some(flag as u8 as u16));
                    return Err(());
                }

                let is_flag_set = (self.regs.af & (1 << (7 - flag))) != 0;

                if is_flag_set != (opcode == 0x11) {
                    self.regs.pc = self.regs.pc.wrapping_add(4);
                }

                Ok(())
            },

            // IF2
            0x13 => {
                let (flag_a, flag_b, cond) = args!(REG_OR_LIT_1, REG_OR_LIT_1, REG_OR_LIT_1);
                let (flag_a, flag_b) = (self.regs.af & (1 << (7 - flag_a)) != 0, self.regs.af & (1 << (7 - flag_b)) != 0);

                let result = match cond {
                    0x01 => flag_a || flag_b,
                    0x02 => flag_a && flag_b,
                    0x03 => flag_a ^ flag_b,
                    0x04 => !flag_a && !flag_b,
                    0x05 => !(flag_a && flag_b),
                    0x06 => flag_a && !flag_b,
                    0x07 => flag_b && !flag_a,
                    _ => {
                        self.exception(0x0D, Some(cond as u8 as u16));
                        return Err(())
                    }
                };

                if !result {
                    self.regs.pc = self.regs.pc.wrapping_add(4);
                }

                Ok(())
            },

            // LSA
            0x14 => {
                let (reg_dest, v_addr, add) = args!(REG, REG_OR_LIT_1, REG_OR_LIT_1);

                let word = self.mem_read(v_addr.wrapping_add(add))?;

                self.write_reg(reg_dest, word)
            },

            // LEA
            0x15 => {
                let (v_addr, add, mul) = args!(REG_OR_LIT_1, REG_OR_LIT_1, REG_OR_LIT_1);

                self.regs.avr = self.mem_read(v_addr.wrapping_add(add.wrapping_mul(mul)))?;
                Ok(())
            },

            // WSA
            0x16 => {
                let (v_addr, add, val) = args!(REG_OR_LIT_1, REG_OR_LIT_1, REG_OR_LIT_1);

                self.mem_write(v_addr.wrapping_add(add), val)
            },

            // WEA
            0x17 => {
                let (v_addr, add, mul) = args!(REG_OR_LIT_1, REG_OR_LIT_1, REG_OR_LIT_1);

                self.mem_write(v_addr.wrapping_add(add.wrapping_mul(mul)), self.regs.avr)
            },

            // SRM
            0x18 => {
                let (v_addr, add, reg_swap) = args!(REG_OR_LIT_1, REG_OR_LIT_1, REG);

                let old_word = self.mem_read(v_addr + add)?;
                let to_write = self.read_reg(reg_swap)?;
                self.mem_write(v_addr + add, to_write)?;
                self.write_reg(reg_swap, old_word)
            },

            // PUSH
            0x19 => {
                let word = args!(REG_OR_LIT_2);

                let stack_v_addr = if self.sv_mode() { self.regs.ssp } else { self.regs.usp }.wrapping_sub(4);

                self.mem_write(stack_v_addr, word)?;

                if self.sv_mode() {
                    self.regs.ssp = stack_v_addr;
                } else {
                    self.regs.usp = stack_v_addr;
                }

                Ok(())
            },

            // POP
            0x1A => {
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
            0x1B => {
                let jmp_v_addr = args!(REG_OR_LIT_2);

                let stack_v_addr = if self.sv_mode() { self.regs.ssp } else { self.regs.usp }.wrapping_sub(4);

                self.mem_write(stack_v_addr, self.regs.pc + 4)?;

                if self.sv_mode() {
                    self.regs.ssp = stack_v_addr;
                } else {
                    self.regs.usp = stack_v_addr;
                }

                self.regs.pc = jmp_v_addr;
                self._cycle_changed_pc = true;

                Ok(())
            },

            // HWD
            0x1C => {
                let (reg_dest, aux_id, hw_info) = args!(REG, REG_OR_LIT_1, REG_OR_LIT_1);

                if aux_id == 0 && hw_info == 0 {
                    return self.write_reg(reg_dest, self.hwb.count() as u32);
                }

                let aux_id = usize::try_from(aux_id)
                    .map_err(|_| self.exception(0x10, Some(aux_id as u16)))?;

                let hw_data = self.get_hw_info(hw_info, aux_id)?;

                self.write_reg(reg_dest, hw_data)
            },

            // CYCLES
            0x1D => {
                let reg_dest = args!(REG);
                self.write_reg(reg_dest, self.cycles as u32)
            },

            // HALT
            0x1E => {
                self.halted = true;
                Ok(())
            },

            // RESET
            0x1F => {
                let mode = args!(REG_OR_LIT_1);

                // Get the two modes (one per byte)
                let (cpu_mode, aux_mode) = ((mode & 0xF0) as u8, (mode & 0x0F) as u8);

                // Determine which components should be reset
                match aux_mode {
                    // Reset all components
                    0x0 => {
                        for id in 0..self.hwb.count() {
                            self.hwb.reset(id).unwrap();
                        }
                    },

                    // Reset a specific component (ID in `avr`)
                    0x1 => {
                        let id = usize::try_from(self.regs.avr)
                            .map_err(|_| self.exception(0x10, Some(self.regs.avr as u16)))?;

                            self.hwb.reset(id)
                                .ok_or_else(|| self.exception(0x10, Some(self.regs.avr as u16)))?;
                    },

                    // Reset a component based on a condition (operand ID in `avr`)
                    0x2..=0x4 => {
                        let ignore_id = usize::try_from(self.regs.avr).ok();

                        // Determine how to test if a component should be reset
                        let test = move |id| match ignore_id {
                            None => true,
                            Some(ignore_id) => match aux_mode {
                                0x2 => id != ignore_id,
                                0x3 => id < ignore_id,
                                0x4 => id > ignore_id,
                                _ => unreachable!()
                            }
                        };

                        for id in 0..self.hwb.count() {
                            if test(id) {
                                self.hwb.reset(id).unwrap();
                            }
                        }
                    },

                    _ => {}

                };

                // Reset the processor
                if cpu_mode == 0 {
                    self.reset();
                }

                Ok(())
            },

            _ => unreachable!("Internal error: processor encountered an instruction with an opcode greater than 0x1F (> 5 bits)")
        }
    }

    /// Try to read a register's value.
    /// Raises an exception if the specified register is only readable in supervisor mode and userland mode is active.
    fn read_reg(&mut self, code: u8) -> Result<u32, ()> {
        if code >= 0x18 && !self.sv_mode() {
            self.exception(0x03, Some(code.into()));
            return Err(());
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
            _ => {
                self.exception(0x02, Some(code.into()));
                Err(())
            }
        }
    }

    /// Try to write a register's value.
    /// Raises an exception if the specified register is only writable in supervisor mode and userland mode is active.
    /// Raises an exception if the specified register is not writable.
    fn write_reg(&mut self, code: u8, word: u32) -> Result<(), ()> {
        let ucode = usize::from(code);

        if code >= 0x17 && !self.sv_mode() {
            self.exception(0x04, Some(code.into()));
            return Err(());
        }

        if code == 0x17 || code == 0x1A || code == 0x1B {
            self.exception(0x04, Some(code.into()));
            return Err(());
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
            _ => {
                self.exception(0x02, Some(code.into()));
                return Err(());
            }
        }

        Ok(())
    }

    /// Perform a numeric computation and set the arithmetic flags.
    /// Raises an exception if a forbidden operation happens (e.g. division by zero when forbidden by the provided division mode).
    fn compute(&mut self, op1: u32, op2: u32, op: Op) -> Result<u32, ()> {
        let iop1 = op1 as i32;
        let iop2 = op2 as i32;

        let (result, has_carry, has_overflow) = match op {
            Op::Add => {
                let (result, has_carry) = op1.overflowing_add(op2);
                (result, has_carry, iop1.overflowing_add(iop2).1)
            }

            Op::Sub => {
                let (result, has_carry) = op1.overflowing_sub(op2);
                (result, has_carry, iop1.overflowing_sub(iop2).1)
            }

            Op::Mul => {
                let (result, has_carry) = iop1.overflowing_mul(iop2);
                (result as u32, has_carry, has_carry)
            }

            // This one is a bit tricky
            Op::Div { mode } | Op::Mod { mode } => {
                // Must we perform a signed division / modulus?
                let signed = mode & 0b0001_0000 != 0;

                match (op == Op::Div { mode }, signed, iop1, iop2) {
                    // Division / modulus by zero
                    (_, _, _, 0) => match (mode & 0b0000_1100) >> 2 {
                        // Forbid
                        0b00 => {
                            self.exception(0x0A, None);
                            return Err(());
                        }
                        // Result in the minimum signed value
                        0b01 => (0x8000_0000, true, true),
                        // Result in zero
                        0b10 => (0x0000_0000, true, true),
                        // Result in the maximum signed value
                        0b11 => (0x7FFF_FFFF, true, true),
                        _ => unreachable!(),
                    },

                    // Minimum signed value divided / moduled by -1 (overflowing multiplication)
                    (_, true, std::i32::MIN, -1) => match (mode & 0b0000_0011) >> 2 {
                        // Forbid
                        0b00 => {
                            self.exception(0x0B, None);
                            return Err(());
                        }
                        // Result in the minimum signed value
                        0b01 => (0x8000_0000, true, true),
                        // Result in zero
                        0b10 => (0x0000_0000, true, true),
                        // Result in the maximum signed value
                        0b11 => (0x7FFF_FFFF, true, true),
                        _ => unreachable!(),
                    },

                    // Safe unsigned division
                    (true, true, _, _) => ((iop1 / iop2) as u32, false, false),

                    // Safe unsigned modulus
                    (false, true, _, _) => ((iop1 % iop2) as u32, false, false),

                    // Safe signed division
                    (true, false, _, _) => (op1 / op2, false, false),

                    // Safe signed modulus
                    (false, false, _, _) => (op1 % op2, false, false),
                }
            }

            Op::And => (op1 & op2, false, false),

            Op::Bor => (op1 | op2, false, false),

            Op::Xor => (op1 ^ op2, false, false),

            Op::Shl => {
                let (result, has_carry) = op1.overflowing_shl(op2);
                (result, has_carry, has_carry)
            }

            Op::Shr => {
                let (result, has_carry) = op1.overflowing_shr(op2);
                (result, has_carry, has_carry)
            }
        };

        // => Compute and assign arithmetic flags to the `af` register

        self.regs.af = 0;

        let flags: [bool; 7] = [
            // Zero Flag
            result == 0,
            // Carry Flag
            has_carry,
            // Overflow Flag
            has_overflow,
            // Sign Flag
            (result >> 31) & 0b1 == 1,
            // Even Flag
            result & 0b1 == 0,
            // Zero-Upper Flag
            result <= 0xFFFF,
            // Zero-Lower Flag
            (result >> 16).trailing_zeros() == 0,
        ];

        for (bit, flag) in flags.iter().enumerate() {
            if *flag {
                self.regs.af += 1 << (7 - bit);
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
    fn exception(&mut self, code: u8, associated: Option<u16>) {
        // Assign the Exception Type `et` register.
        self.regs.et = (if self.sv_mode() { 1 << 24 } else { 0 })
            + (u32::from(code) << 16)
            + u32::from(associated.unwrap_or(0));

        // Jump to the Exception Vector address
        self.regs.pc = self.regs.ev;

        // Enable supervisor mode to deal with the exception
        self.regs.smt = 1;

        // Do not forget to indicate we changed PC
        self._cycle_changed_pc = true;
    }

    /// Ensure an address is aligned, or raise an exception otherwise.
    fn ensure_aligned(&mut self, v_addr: u32) -> Result<u32, ()> {
        if v_addr % 4 != 0 {
            self.exception(0x05, Some((v_addr % 4) as u16));
            Err(())
        } else {
            Ok(v_addr)
        }
    }

    /// Perform an action on the memory.  
    /// The provided address will be first translated by the MMU into a physical address, then the provided handler will be called with:
    ///
    /// * A mutable reference to the mapped memory  
    /// * The translated physical address  
    /// * A mutable reference to the exception variable
    ///
    /// The handler is expected to return a value (of any type), which will be turned into an Err() if an exception occurred.
    fn mem_do<T>(
        &mut self,
        action: MemAction,
        v_addr: u32,
        handler: &mut dyn FnMut(&mut MappedMemory, u32, &mut u16) -> T,
    ) -> Result<T, ()> {
        let v_addr = self.ensure_aligned(v_addr)?;

        match self
            .mmu
            .translate(&mut self.mem, &self.regs, v_addr, action)
        {
            Ok(p_addr) => {
                let mut ex = 0;
                let ret = handler(&mut self.mem, p_addr, &mut ex);

                if ex != 0 {
                    self.exception(0xA0, Some(ex));
                    Err(())
                } else {
                    Ok(ret)
                }
            }

            Err(None) => {
                self.exception(0x06, Some(v_addr as u16));
                Err(())
            }

            Err(Some(ex)) => {
                self.exception(0xA0, Some(ex));
                Err(())
            }
        }
    }

    /// Read an address in the mapped memory.
    /// Raises an exception if address is unaligned or if the MMU doesn't accept reading this address in the current mode.
    fn mem_read(&mut self, v_addr: u32) -> Result<u32, ()> {
        self.mem_do(MemAction::Read, v_addr, &mut |mem, p_addr, ex| {
            mem.read(p_addr, ex)
        })
    }

    /// Write an address in the mapped memory.
    /// Raises an exception if address is unaligned or if the MMU doesn't accept writing this address in the current mode.
    fn mem_write(&mut self, v_addr: u32, word: u32) -> Result<(), ()> {
        self.mem_do(MemAction::Write, v_addr, &mut |mem, p_addr, ex| {
            mem.write(p_addr, word, ex)
        })
    }

    /// Execute (read) an address in the mapped memory.
    /// Raises an exception if address is unaligned or if the MMU doesn't accept executing this address in the current mode.
    fn mem_exec(&mut self, v_addr: u32) -> Result<u32, ()> {
        self.mem_do(MemAction::Exec, v_addr, &mut |mem, p_addr, ex| {
            mem.read(p_addr, ex)
        })
    }

    /// Get informations about an auxiliary comopnent, after retrieving its name and raw metadata
    fn get_hw_info(&mut self, hw_info: u32, aux_id: usize) -> Result<u32, ()> {
        // Get the auxiliary component's name and metadata (if it exists) as well as its optional mapping
        let cache = self
            .hwb
            .cache_of(aux_id)
            .cloned()
            .ok_or_else(|| self.exception(0x10, Some(aux_id as u16)))?;

        let mapping_opt = self.mem.get_mapping(aux_id).cloned();

        let aux_name = cache.name.bytes();

        // Return the value to write depending on the hardware information code
        let data = match hw_info {
            // UID's 32 strongest bits
            0x01 => cache.metadata[0],

            // UID's 32 weakest bits
            0x02 => cache.metadata[1],

            // Name's length, in bytes
            0x10 => aux_name.count() as u32,

            // Name's nth byte
            0x11..=0x18 => {
                let mut name_bytes = aux_name.skip(((hw_info - 0x11) * 4) as usize);
                u32::from_be_bytes([
                    name_bytes.next().unwrap_or(0),
                    name_bytes.next().unwrap_or(0),
                    name_bytes.next().unwrap_or(0),
                    name_bytes.next().unwrap_or(0),
                ])
            }

            // Component's size
            0x20 => cache.metadata[2],
            // Category
            0x21 => cache.metadata[3],
            // Type
            0x22 => cache.metadata[4],
            // Model
            0x23 => cache.metadata[5],
            // Additional data's 32 strongest bits
            0x24 => cache.metadata[6],
            // Additional data's 32 weakest bits
            0x25 => cache.metadata[7],

            // Check if the component is mapped in memory
            0xA0 => {
                if mapping_opt.is_some() {
                    1
                } else {
                    0
                }
            }
            // Mapping's start address
            0xA1 => {
                mapping_opt
                    .ok_or_else(|| self.exception(0x12, Some(aux_id as u16)))?
                    .addr
            }
            // Mapping's end address
            0xA2 => mapping_opt
                .ok_or_else(|| self.exception(0x12, Some(aux_id as u16)))?
                .end_addr(),

            // Invalid information code
            _ => {
                self.exception(0x11, Some(hw_info as u16));
                return Err(());
            }
        };

        Ok(data)
    }
}

/// (Internal) Numeric operation
#[derive(PartialEq, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div { mode: u8 },
    Mod { mode: u8 },
    And,
    Bor,
    Xor,
    Shl,
    Shr,
}
