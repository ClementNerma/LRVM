use lrvm_aux::display::NumberDisplay;
use lrvm_aux::storage::BootRom;
use lrvm_aux::time::RealtimeClock;
use lrvm_tools::debug::{exec_vm, RunConfig};
use lrvm_tools::lasm::assemble_words;

static LASM_SOURCE: &str = "
main:
    cpy ac0, 0x1000
    lsa a1, ac0, 16

wait_second:
    lsa a0, ac0, 16
    cmp a0, a1

    ifle
    jpr 12

    wsa ac0, 24, a0
    cpy a1, a0
    jp wait_second
";

fn main() {
    let program = assemble_words(LASM_SOURCE)
        .unwrap_or_else(|err| panic!("Failed to assemble demo program: {}", err));

    exec_vm(
        vec![
            Box::new(BootRom::with_size(program, 0x1000, 0x1).unwrap()),
            Box::new(RealtimeClock::new(0x2)),
            Box::new(NumberDisplay::new(
                Box::new(|num, _format, _newline| {
                    println!("Elapsed: {} second{}", num, if num > 1 { "s" } else { "" })
                }),
                0x3,
            )),
        ],
        RunConfig::halt_on_ex(),
    );
}
