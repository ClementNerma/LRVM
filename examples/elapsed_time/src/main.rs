use mrvm_aux::storage::BootROM;
use mrvm_aux::time::RealtimeClock;
use mrvm_aux::display::NumberDisplay;
use mrvm_tools::lasm::assemble_words;
use mrvm_tools::debug::{exec_vm, RunConfig};

static LASM_SOURCE: &'static str = "
main:
    cpy ac0, 0x1000
    lsa a1, ac0, 16

wait_second:
    lsa a0, ac0, 16
    cmp a0, a1

    ifle
    jmp 12

    wsa ac0, 24, a0
    cpy a1, a0
    jmpa wait_second
";

fn main() {
    let program = assemble_words(LASM_SOURCE)
        .unwrap_or_else(|err| panic!("Failed to assemble demo program: {}", err));

    exec_vm(vec![
        Box::new(BootROM::with_size(program, 0x1000, 0x1).unwrap()),
        Box::new(RealtimeClock::new(0x2)),
        Box::new(NumberDisplay::new(Box::new(|num| println!("Elapsed: {} second{}", num, if num > 1 { "s" } else { "" })), 0x3)),
    ], &RunConfig::halt_on_ex());
}
