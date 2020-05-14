use mrvm_aux::storage::BootROM;
use mrvm_tools::lasm::assemble_words;
use mrvm_tools::debug::{exec_vm, RunConfig};

fn main() {
    let program = assemble_words("halt").unwrap();

    exec_vm(vec![ Box::new( BootROM::new(program, 0x0123_4567_89AB_CDEF).unwrap() ) ], &RunConfig::new());
}