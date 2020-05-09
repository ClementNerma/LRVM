mod components;

use mrvm::board::{MotherBoard, Bus, ContiguousMappingStatus, MappingRange};
use mrvm_tools::lasm::assemble_words;
use self::components::{BootROM, RAM};

fn main() {
    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::new(assemble_words(include_str!("source.lasm")).unwrap())),
        Box::new(RAM::new(0x1000))
    ];

    let aux_len = components.len();

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        let ContiguousMappingStatus { mapping, aux_mapping } =
            mem.map_contiguous(0x00000000, (0..aux_len).collect::<Vec<_>>());

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

        if let Err(failed) = mapping {
            panic!("Failed to map {} component{}!", failed.len(), if failed.len() == 1 { "" } else { "s" });
        }
    });

    motherboard.reset();

    let cpu = motherboard.cpu();

    while !cpu.halted() {
        let was_at = cpu.regs.pc;

        match cpu.next() {
            Ok(true) => {}
            Ok(false) => unreachable!("CPU can't run because it's halted"),
            Err(ex) => {
                panic!(
                    "At address {:#010X} - Exception occurred: {:#04X} (data = {:#04X})",
                    was_at,
                    ex.code,
                    ex.associated.unwrap_or(0)
                );
            },
        };
    }
}
