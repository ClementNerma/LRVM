use mrvm_aux::storage::BootROM;
use mrvm_tools::debug::{exec_vm, RunConfig};
use mrvm_tools::lasm::assemble_words;

fn main() {
    let program = assemble_words("halt").unwrap();

    exec_vm(
        vec![Box::new(BootROM::new(program, 0x0).unwrap())],
        RunConfig::new(),
    );
}
