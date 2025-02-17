mod counter;

use counter::AsyncCounter;
use lrvm_aux::display::NumberDisplay;
use lrvm_aux::storage::BootRom;
use lrvm_aux::volatile_mem::Ram;
use lrvm_tools::debug::{exec_vm, RunConfig};
use lrvm_tools::lasm::assemble_words;
use rand::Rng;

fn main() {
    let program = assemble_words(include_str!("source.lasm"))
        .unwrap_or_else(|err| panic!("Failed to assemble demo program: {}", err));

    let mut rng = rand::rng();

    exec_vm(
        vec![
            Box::new(BootRom::with_size(program, 0x1000, rng.random()).unwrap()),
            Box::new(Ram::new(0x1000, rng.random()).unwrap()),
            Box::new(AsyncCounter::new(rng.random())),
            Box::new(NumberDisplay::new_print(rng.random())),
        ],
        RunConfig::halt_on_ex(),
    );
}
