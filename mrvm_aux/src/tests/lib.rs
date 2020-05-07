use mrvm::board::{MotherBoard, Bus, MappingRange, ContiguousMappingStatus};
use mrvm::cpu::CPU;

pub fn prepare_vm(components: Vec<Box<dyn Bus>>) -> MotherBoard {
    print!("\n");

    let aux_count = components.len();

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        let ContiguousMappingStatus { mapping, aux_mapping } =
            mem.map_contiguous(0x00000000, (0..aux_count).collect::<Vec<_>>());

        for result in aux_mapping {
            println!(
                "=> Component {:04} '{:32}': {} {}",
                result.aux_id,
                result.aux_name,
                if result.aux_mapping.is_ok() { "✓" } else { "✗" },
                match result.aux_mapping {
                    Ok(MappingRange { start_addr, end_addr }) =>
                        format!("{:#010X} -> {:#010X}", start_addr, end_addr),
                    Err(err) => format!("{:?}", err),
                }
            );
        }

        assert!(mapping.is_ok(), "At least one component could not be mounted correctly!");
    });

    motherboard.reset();
    motherboard
}

pub fn run_until_halt(cpu: &mut CPU, cycles_limit: Option<u32>) -> u32 {
    while !cpu.halted() && cycles_limit.map(|limit| cpu.cycles() < limit).unwrap_or(true) {
        let was_at = cpu.regs.pc;

        match cpu.next() {
            Ok(true) => {}
            Ok(false) => unreachable!("CPU can't run because it's halted"),
            Err(ex) => println!(
                "At address {:#010X} - Exception occurred: {:#04X} (data = {:#04X})",
                was_at,
                ex.code,
                ex.associated.unwrap_or(0)
            ),
        };
    }

    cpu.cycles()
}
