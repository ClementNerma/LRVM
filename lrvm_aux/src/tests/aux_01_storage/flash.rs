use lrvm_tools::{
    asm::{ExtInstr, Instr, Program},
    debug::{exec_vm, RunConfig},
};

use crate::storage::{BootRom, PersistentMem};

#[test]
fn flash_mem() {
    let mut program = Program::from_instr(ExtInstr::WriteAddrLit(0x1000, 0x01234567).to_instr());
    program.append_all(ExtInstr::WriteAddrLit(0x1008, 0x89ABCDEF).to_prog_words());
    program.append(Instr::Halt().into());

    let (mut vm, state) = exec_vm(
        vec![
            Box::new(BootRom::with_size(program.encode_words(), 0x1000, 0x0).unwrap()),
            Box::new(PersistentMem::new(0x1000, 0x1).unwrap()),
        ],
        RunConfig::halt_on_ex(),
    );

    if state.ex.is_some() {
        panic!("Unexpected exception occurred while running the VM!");
    }

    let (mut err_a, mut err_b, mut err_c) = (0, 0, 0);

    let (word_a, word_b, word_c) = vm.map(|mem| {
        (
            mem.read(0x1000, &mut err_a),
            mem.read(0x1008, &mut err_b),
            mem.read(0x1010, &mut err_c),
        )
    });

    assert_eq!(
        err_a, 0,
        "Hardware exception occurred while reading word at address 0x00001000: {:#008X}",
        err_a
    );
    assert_eq!(
        err_b, 0,
        "Hardware exception occurred while reading word at address 0x00001008: {:#008X}",
        err_b
    );
    assert_eq!(
        err_c, 0,
        "Hardware exception occurred while reading word at address 0x00001010: {:#008X}",
        err_c
    );

    assert_eq!(word_a, 0x01234567, "Expected word at address 0x00001000 to contain 0x01234567 but it actually contains {:#010X}", word_a);
    assert_eq!(word_b, 0x89ABCDEF, "Expected word at address 0x00001008 to contain 0x89ABCDEF but it actually contains {:#010X}", word_b);
    assert_eq!(word_c, 0x00000000, "Expected word at address 0x00001010 to contain 0x01234567 but it actually contains {:#010X}", word_c);
}
