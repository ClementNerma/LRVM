mod components;

use mrvm::board::MotherBoard;
use mrvm_tools::lasm::assemble_words;
use mrvm_tools::bytes::words_to_bytes;
use mrvm_tools::metadata::DeviceCategory;
use mrvm_tools::exceptions::{NativeException, AuxHwException};
use mrvm_tools::debug::{prepare_vm, run_until_halt_or_ex};
use self::components::{BootROM, RAM};

/// Prepare the VM
fn prepare() -> MotherBoard {
    println!("> (1/3) Assembling source program...");

    // Compile the source code
    let program = assemble_words(include_str!("source.lasm"))
        .unwrap_or_else(|err| panic!("Failed to assemble demo program: {}", err));

    println!("> (2/3) Preparing motherboard...");

    let mut motherboard = prepare_vm(vec![
        // BootROM containing the program's machine code
        Box::new(BootROM::new(program, 0x1000)),
        // RAM that will contain informations about each detected components
        Box::new(RAM::new(0x1000)),
        // RAM that will be used for the stack
        Box::new(RAM::new(0x20))
    ]);

    let cpu = motherboard.cpu();

    println!("> (3/3) Starting the virtual machine...");

    if let Err(ex) = run_until_halt_or_ex(cpu) {
        match NativeException::decode_with_mode(ex) {
            Ok((ex, was_sv)) => panic!(
                "Unexpected exception occurred (in {} mode) while running demo program:\n{}",
                if was_sv { "supervisor" } else { "userland" },
                ex
            ),
            Err(_) => panic!("Unexpected UNKNOWN exception occurred while running demo program ({:#006X})", ex)
        }
    }

    motherboard
}

fn main() {
    let mut motherboard = prepare();

    // We read the memory from inside this handler as the mapped memory object cannot be moved out of the motherboard instance.
    motherboard.map(|mut mem| {
        // Address of the data RAM
        let ram_addr = 0x1000;
        
        // This function reads an address in the memory and handles exceptions.
        // If an exception occurrs, it tries to decode it to show a human-readable error instead of an exception code.
        let mut read = move |addr: u32| {
            let mut ex = 0;
            let word = mem.read(addr, &mut ex);

            // Handle exceptions
            if ex != 0 {
                match AuxHwException::decode(ex) {
                    Ok(ex) => panic!("Unexpected exception occurred while reading memory address {:#010X}:\n{}", addr, ex),
                    Err(_) => panic!("Unexpected UNKNOWN exception occurred while reading memory address {:#010X} ({:#006X})", addr, ex)
                }
            }

            word
        };

        // The first word in the data RAM contains the number of components
        // See the source LASM file in this directory to know the exact memory structure of the data RAM
        let components = read(ram_addr);

        // Decode informations on each component
        for aux_id in 0..components {
            let aux_addr = ram_addr + 0x4 + (aux_id * 76);

            println!("\n========== Decoding component n°{}/{} (from memory address {:#010X}) ==========", aux_id + 1, components, aux_addr);
            
            println!("> UID      : {:#018X}", ((read(aux_addr) as u64) << 32) + read(aux_addr + 0x04) as u64);
            
            println!("> Name     : {}", String::from_utf8_lossy(&words_to_bytes(&[
                read(aux_addr + 0x08), read(aux_addr + 0x0C), read(aux_addr + 0x10), read(aux_addr + 0x14),
                read(aux_addr + 0x18), read(aux_addr + 0x1C), read(aux_addr + 0x20), read(aux_addr + 0x24)
            ])));
            
            println!("> Size     : {:#010X} (bytes)", read(aux_addr + 0x28));

            let cat_type = ((read(aux_addr + 0x2C) as u64) << 32) + read(aux_addr + 0x30) as u64;
            println!("> Category : {}", match DeviceCategory::decode(cat_type) {
                Ok(cat) => format!("{}", cat),
                Err(_) => format!("<unknown> ({:#018X})", cat_type)
            });

            println!("> Model    : {:#010X}", read(aux_addr + 0x34));

            println!("> Data     : {:#018X}", (read(aux_addr + 0x38) as u64) << 32 + read(aux_addr + 0x3C));

            println!("> Mapping  : {}", match read(aux_addr + 0x40) {
                0x00000000 => "<device is not mapped>".to_owned(),
                _ => format!("{:#010X} -> {:#010X}", read(aux_addr + 0x44), read(aux_addr + 0x48))
            });
        }
    });
}
