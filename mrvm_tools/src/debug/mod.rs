use mrvm::board::{MotherBoard, Bus, MappingRange, ContiguousMappingStatus};
use mrvm::cpu::CPU;

/// Prepare a motherboard from a list of components.
/// The mapping status of all components is displayed.
///
/// In case of success, the component's name as well as its start and mapping address are displayed.
/// In case of fail, the reason is displayed with the component's name and the program panics.
pub fn prepare_vm(components: Vec<Box<dyn Bus>>) -> MotherBoard {
    let aux_count = components.len();

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        let ContiguousMappingStatus { mapping, aux_mapping } =
            mem.map_contiguous(0x00000000, (0..aux_count).collect::<Vec<_>>());

        for result in aux_mapping {
            println!(
                "=> Component {:04} '{:32}': {} {} (HW ID: 0x{})",
                result.aux_id,
                result.aux_name,
                if result.aux_mapping.is_ok() { "✓" } else { "✗" },
                match result.aux_mapping {
                    Ok(MappingRange { start_addr, end_addr }) =>
                    format!("{:#010X} -> {:#010X}", start_addr, end_addr),
                    Err(err) => format!("{:?}", err),
                },
                result.aux_hw_id.to_be_bytes().iter().map(|byte| format!("{:002X}", byte)).collect::<Vec<String>>().join(" "),
            );
        }

        if let Err(failed) = mapping {
            panic!("Failed to map {} component{}!", failed.len(), if failed.len() == 1 { "" } else { "s" });
        }
    });

    motherboard.reset();
    motherboard
}

/// Run a virtual machine until the CPU halt, eventually encounters an exception or reaches a given number of cycles.
/// The first member of the returned tuple is the cycle number when the function stopped running the VM, and the second one
/// indicates if the VM was stopped due to an exception (only if `halt_on_ex` is set).
pub fn run_until_halt_ex_limit(cpu: &mut CPU, halt_on_ex: bool, cycles_limit: Option<u32>) -> (u32, bool) {
    let mut had_ex = false;

    while !cpu.halted() && cycles_limit.map(|limit| cpu.cycles() < limit).unwrap_or(true) {
        let was_at = cpu.regs.pc;

        match cpu.next() {
            Ok(true) => {}
            Ok(false) => unreachable!("CPU can't run because it's halted"),
            Err(ex) => {
                println!(
                    "At address {:#010X} - Exception occurred: {:#04X} (data = {:#04X})",
                    was_at,
                    ex.code,
                    ex.associated.unwrap_or(0)
                );

                if halt_on_ex { had_ex = true; break }
            },
        };
    }

    (cpu.cycles(), had_ex)
}

/// Run a virtual machine until the CPU halts.
/// The returned value is the cycle number when the function stopped running the VM.
pub fn run_until_halt(cpu: &mut CPU) -> u32 {
    run_until_halt_ex_limit(cpu, false, None).0
}

/// Run a virtual machine until the CPU halt or encounters an exception.
/// The Ok() variant of the returned value is the cycle number when the function stopped running the VM.
/// The Err() variant indicates the VM was stopped due to an exception and provides the value of the Exception Type register.
pub fn run_until_halt_or_ex(cpu: &mut CPU) -> Result<u32, u32> {
    let res = run_until_halt_ex_limit(cpu, true, None);
    
    if res.1 {
        Err(cpu.regs.et)
    } else {
        Ok(res.0)
    }
}
