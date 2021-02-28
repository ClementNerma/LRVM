use super::RunConfig;
use crate::exceptions::NativeException;
use mrvm::cpu::Cpu;
use std::fmt;

/// State of the VM when exited
#[derive(Debug, Clone)]
pub struct StoppedState {
    /// Cycles count when the VM stopped
    pub cycles: u128,
    /// The address the VM was stopped at
    pub addr: u32,
    /// If the VM was stopped due to an exception, contains the faulty exception
    pub ex: Option<ExWithMode>,
}

/// Native exception, with mode
#[derive(Debug, Clone)]
pub struct ExWithMode {
    /// The raw exception
    pub raw: u32,

    /// Did the exception occurred in supervisor mode?
    pub sv_mode: bool,
    /// Exception's code
    pub code: u8,
    /// Exception's eventual associated data
    pub associated: u16,
}

/// Run a virtual machine until the CPU halt, eventually encounters an exception or reaches a given number of cycles.
pub fn run_vm(cpu: &mut Cpu, config: RunConfig) -> StoppedState {
    // If the VM is stopped because of an exception, it will be put in here
    let mut stop_ex = None;

    // Address the CPU was at when the VM was stopped
    let mut was_at = cpu.regs.pc;

    // Run the VM until it halts
    while !cpu.halted() {
        // Ensure cycles limit isn't exceeded yet
        if let Some(cycles_limit) = config.cycles_limit {
            if cpu.cycles() > cycles_limit {
                break;
            }
        }

        // Update the current instruction address
        was_at = cpu.regs.pc;

        if config.print_cycles {
            println!(
                "[mrvm] Running cycle {:#010X} at address {:#010X}",
                cpu.cycles(),
                cpu.regs.pc
            );
        }

        // Run the next instruction
        cpu.next();

        // Check if an exception occurred
        if cpu.regs.et != 0 {
            let exception_bytes = cpu.regs.et.to_be_bytes();

            // Complete the exception with the mode it occurred in
            let ex = ExWithMode {
                raw: cpu.regs.et,
                sv_mode: exception_bytes[0] != 0,
                code: exception_bytes[1],
                associated: u16::from_be_bytes([exception_bytes[2], exception_bytes[3]]),
            };

            if config.print_exceptions && !(config.halt_on_exception && config.print_finish) {
                println!(
                    "[mrvm] At address {:#010X} - Exception occurred: {}",
                    was_at,
                    prettify_ex_with_mode(&ex)
                );
            }

            if config.halt_on_exception {
                stop_ex = Some(ex);
                break;
            }
        }
    }

    let state = StoppedState {
        cycles: cpu.cycles(),
        addr: was_at,
        ex: stop_ex,
    };

    if config.print_finish {
        if config.newline_on_finish {
            println!();
        }

        println!("[mrvm] {}", prettify_stop(&state));
    }

    state
}

/// Prettify an exception with mode
pub fn prettify_ex_with_mode(ex: &ExWithMode) -> String {
    match NativeException::decode_parts(ex.code, Some(ex.associated)) {
        Ok(ex) => format!("{}", ex),
        Err(()) => "<invalid exception code or data>".to_string(),
    }
}

/// Prettify a stop state
pub fn prettify_stop(state: &StoppedState) -> String {
    let mut output = format!(
        "Cycle {:#010X}: CPU halted at address {:#010X}",
        state.cycles, state.addr
    );

    if let Some(ex) = &state.ex {
        output.push_str(&format!(
            " because of exception in {} mode: {}",
            if ex.sv_mode { "supervisor" } else { "userland" },
            prettify_ex_with_mode(ex)
        ));
    }

    output
}

impl fmt::Display for ExWithMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", prettify_ex_with_mode(self))
    }
}

impl fmt::Display for StoppedState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", prettify_stop(self))
    }
}
