use mrvm::cpu::CPU;
use super::RunConfig;
use crate::exceptions::NativeException;

/// State of the VM when exited
#[derive(Debug, Clone)]
pub struct StoppedState {
    /// Cycles count when the VM stopped
    pub cycles: u32,
    /// The address the VM was stopped at
    pub addr: u32,
    /// If the VM was stopped due to an exception, contains the faulty exception
    pub ex: Option<ExWithMode>
}

/// Native exception, with mode
#[derive(Debug, Clone)]
pub struct ExWithMode {
    // Did the exception occurred in supervisor mode?
    pub sv_mode: bool,
    /// Exception's code
    pub code: u8,
    /// Exception's eventual associated data
    pub associated: Option<u16>
}

/// Run a virtual machine until the CPU halt, eventually encounters an exception or reaches a given number of cycles.
pub fn run_vm(cpu: &mut CPU, config: &RunConfig) -> StoppedState {
    // If the VM is stopped because of an exception, it will be put in here
    let mut stop_ex = None;

    // Address the CPU was at when the VM was stopped
    let mut was_at = cpu.regs.pc;

    while !cpu.halted() && config.cycles_limit.map(|limit| cpu.cycles() < limit).unwrap_or(true) {
        // Get the mode now, as it will be turned into supervisor mode automatically if an exception occurs
        let was_sv = cpu.regs.smt != 0;

        // Update the current instruction address
        was_at = cpu.regs.pc;

        if config.print_cycles {
            println!("[mrvm] Running cycle {:#010X} at address {:#010X}", cpu.cycles(), cpu.regs.pc);
        }

        // Run the next instruction
        match cpu.next() {
            // When everything is fine
            Ok(true) => {}

            // We have a check here to ensure this situation never happens, because it *should* never happen
            Ok(false) => unreachable!("CPU can't run because it's halted"),

            // Handle exceptions
            Err(ex) => {
                // Complete the exception with the mode it occurred in
                let ex = ExWithMode {
                    sv_mode: was_sv,
                    code: ex.code,
                    associated: ex.associated
                };

                if config.halt_on_exception {
                    stop_ex = Some(ex);
                    break ;
                } else if config.print_exceptions {
                    println!(
                        "[mrvm] At address {:#010X} - Exception occurred: {}",
                        was_at,
                        prettify_ex_with_mode(&ex)
                    );
                }
            },
        };
    }

    let state = StoppedState { cycles: cpu.cycles(), addr: was_at, ex: stop_ex };

    if config.print_finish {
        println!("[mrvm] {}", prettify_stop(&state));
    }

    state
}

/// Prettify an exception with mode
pub fn prettify_ex_with_mode(ex: &ExWithMode) -> String {
    match NativeException::decode_parts(ex.code, ex.associated) {
        Ok(ex) => format!("{}", ex),
        Err(()) => format!("<invalid exception code or data>")
    }
}

/// Prettify a stop state
pub fn prettify_stop(state: &StoppedState) -> String {
    let mut output = format!("Cycle {:#010X}: CPU halted at address {:#010X}", state.cycles, state.addr);

    if let Some(ex) = &state.ex {
        output.push_str(&format!(
            " because of exception in {} mode: {}",
            if ex.sv_mode { "supervisor" } else { "userland" },
            prettify_ex_with_mode(ex)
        ));
    }

    output
}
