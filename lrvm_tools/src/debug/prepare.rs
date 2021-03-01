use lrvm::board::{Bus, MotherBoard};
use lrvm::mem::{ContiguousMappingResult, MappingRange};

/// Prepare a motherboard from a list of components.
/// The mapping status of all components is displayed.
///
/// In case of success, the component's name as well as its start and mapping address are displayed.
/// In case of fail, the reason is displayed with the component's name and the program panics.
pub fn prepare_vm(components: Vec<Box<dyn Bus>>) -> MotherBoard {
    let aux_count = components.len();

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mem| {
        let ContiguousMappingResult {
            mapping,
            aux_mapping,
        } = mem.map_contiguous(0x0000_0000, (0..aux_count).collect::<Vec<_>>());

        for result in aux_mapping {
            println!(
                "=> Component {:04} '{:32}': {} {} (HW ID: 0x{})",
                result.aux_id,
                result.aux_name,
                if result.aux_mapping.is_ok() {
                    "✓"
                } else {
                    "✗"
                },
                match result.aux_mapping {
                    Ok(MappingRange {
                        start_addr,
                        end_addr,
                    }) => format!("{:#010X} -> {:#010X}", start_addr, end_addr),
                    Err(err) => format!("{:?}", err),
                },
                result
                    .aux_hw_id
                    .to_be_bytes()
                    .iter()
                    .map(|byte| format!("{:002X}", byte))
                    .collect::<Vec<String>>()
                    .join(" "),
            );
        }

        if let Err(failed) = mapping {
            panic!(
                "Failed to map {} component{}!",
                failed.len(),
                if failed.len() == 1 { "" } else { "s" }
            );
        }
    });

    motherboard.reset();
    motherboard
}
