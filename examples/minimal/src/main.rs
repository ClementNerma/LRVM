use lrvm_aux::storage::BootRom;
use lrvm_tools::debug::{exec_vm, RunConfig};
use lrvm_tools::lasm::assemble_words;

fn main() {
    let program = assemble_words("halt").unwrap();

    exec_vm(
        vec![Box::new(BootRom::new(program, 0x0).unwrap())],
        RunConfig::new(),
    );
}
