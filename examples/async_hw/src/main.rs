mod counter;

use rand::Rng;
use mrvm_aux::storage::BootROM;
use mrvm_aux::volatile_mem::RAM;
use mrvm_aux::display::NumberDisplay;
use mrvm_tools::lasm::assemble_words;
use mrvm_tools::debug::{exec_vm, RunConfig};
use counter::AsyncCounter;

fn main() {
    let program = assemble_words(include_str!("source.lasm")).unwrap_or_else(|err| panic!("Failed to assemble demo program: {}", err));

    let mut rng = rand::thread_rng();

    exec_vm(vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(RAM::new(0x1000, rng.gen()).unwrap()),
        Box::new(AsyncCounter::new(rng.gen())),
        Box::new(NumberDisplay::new(Box::new(|num| println!("Counter: {}", num)), rng.gen()))
    ], &RunConfig::halt_on_ex());
}
