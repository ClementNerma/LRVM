use lrvm::mem::MappedMemory;
use lrvm_aux::storage::BootRom;
use lrvm_aux::volatile_mem::Ram;
use lrvm_tools::bytes::words_to_bytes;
use lrvm_tools::debug::{exec_vm, RunConfig};
use lrvm_tools::exceptions::AuxHwException;
use lrvm_tools::lasm::assemble_words;
use lrvm_tools::metadata::DeviceCategory;
use rand::Rng;

struct Component {
    uid: u64,
    name: String,
    size: u32,
    cat_type: Result<DeviceCategory, u64>,
    model: u32,
    data: u64,
    mapping: Option<(u32, u32)>,
}

/// Decode a component from the detection program  
/// Returns an error message in case of error
fn decode_component(mem: &mut MappedMemory, aux_addr: u32) -> Result<Component, String> {
    // This function reads an address in the memory and handles exceptions.
    // If an exception occurrs, it tries to decode it to show a human-readable error instead of an exception code.
    let mut read = move |addr: u32| {
        let mut ex = 0;
        let word = mem.read(addr, &mut ex);

        // Handle exceptions
        if ex != 0 {
            return Err(format!(
                "Exception occurred while manually reading memory address {:#010X}: {}",
                addr,
                match AuxHwException::decode(ex) {
                    Ok(ex) => format!("{}", ex),
                    Err(()) => "<unknown exception>".to_string(),
                },
            ));
        }

        Ok(word)
    };

    // Get the component's unique identifier
    let uid = ((read(aux_addr)? as u64) << 32) + read(aux_addr + 0x04)? as u64;

    // Get the component's name
    let name = String::from_utf8_lossy(&words_to_bytes([
        read(aux_addr + 0x08)?,
        read(aux_addr + 0x0C)?,
        read(aux_addr + 0x10)?,
        read(aux_addr + 0x14)?,
        read(aux_addr + 0x18)?,
        read(aux_addr + 0x1C)?,
        read(aux_addr + 0x20)?,
        read(aux_addr + 0x24)?,
    ]))
    .to_string();

    // Get the component's size
    let size = read(aux_addr + 0x28)?;

    // Get and decode the component's category
    let cat_type = {
        let code = ((read(aux_addr + 0x2C)? as u64) << 32) + read(aux_addr + 0x30)? as u64;
        DeviceCategory::decode(code).map_err(|()| code)
    };

    // Get the component's model
    let model = read(aux_addr + 0x34)?;

    // Get the component's associated data
    let data = ((read(aux_addr + 0x38)? as u64) << 32) + read(aux_addr + 0x3C)? as u64;

    // Get the component's mapping if it's mapped in the memory
    let mapping = {
        let mapping = read(aux_addr + 0x40)?;

        if mapping == 0x0000_0000 {
            None
        } else {
            Some((read(aux_addr + 0x44)?, read(aux_addr + 0x48)?))
        }
    };

    // Success!
    Ok(Component {
        uid,
        name,
        size,
        cat_type,
        model,
        data,
        mapping,
    })
}

fn main() {
    println!("> Assembling LASM code...");

    // Compile the source code
    let program = assemble_words(include_str!("source.lasm"))
        .unwrap_or_else(|err| panic!("Failed to assemble demo program: {}", err));

    println!("> Setting up and booting the VM...");

    let mut rng = rand::rng();

    let mut motherboard = exec_vm(
        vec![
            // BootROM containing the program's machine code
            Box::new(BootRom::with_size(program, 0x1000, rng.random()).unwrap()),
            // RAM that will contain informations about each detected components
            Box::new(Ram::new(0x1000, rng.random()).unwrap()),
            // RAM that will be used for the stack
            Box::new(Ram::new(0x20, rng.random()).unwrap()),
        ],
        RunConfig::halt_on_ex(),
    )
    .0;

    // We read the memory from inside this handler as the mapped memory object cannot be moved out of the motherboard instance.
    motherboard.map(|mem| {
        // Address of the data RAM
        let ram_addr = 0x1000;

        // The first word in the data RAM contains the number of components
        // See the source LASM file in this directory to know the exact memory structure of the data RAM
        let mut ex = 0;
        let components = mem.read(ram_addr, &mut ex);

        if ex != 0 {
            panic!(
                "Exception occurred while manually reading getting components count at memory address {:#010X}: {}",
                ram_addr,
                match AuxHwException::decode(ex) {
                    Ok(ex) => format!("{}", ex),
                    Err(()) => "<unknown exception>".to_string(),
                },
            )
        }

        // Decode informations on each component
        for aux_id in 0..components {
            let aux_addr = ram_addr + 0x4 + (aux_id * 76);

            println!(
                "\n========== Decoding component n°{}/{} (from memory address {:#010X}) ==========",
                aux_id + 1,
                components,
                aux_addr
            );

            let component = decode_component(mem, aux_addr).unwrap_or_else(|err| panic!("{}", err));

            println!("> UID      : {:#018X}", component.uid);
            println!("> Name     : {}", component.name);
            println!("> Size     : {:#010X} (bytes)", component.size);
            println!(
                "> Category : {}",
                match component.cat_type {
                    Ok(cat_type) => format!("{}", cat_type),
                    Err(code) => format!("<unknown> ({:#018X})", code),
                }
            );
            println!("> Model    : {:#010X}", component.model);
            println!("> Data     : {:#018X}", component.data);
            println!(
                "> Mapping  : {}",
                match component.mapping {
                    None => "<device is not mapped>".to_string(),
                    Some((start_addr, end_addr)) =>
                        format!("{:#010X} -> {:#010X}", start_addr, end_addr),
                }
            );
        }
    });
}
