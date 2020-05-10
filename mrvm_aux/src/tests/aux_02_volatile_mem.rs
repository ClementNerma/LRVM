use mrvm_tools::asm::{Program, Instr, ExtInstr};
use crate::storage::BootROM;
use crate::memory::VolatileMem;
use mrvm_tools::debug::{prepare_vm, run_vm, RunConfig};

#[test]
fn volatile_mem() {
    let mut program = Program::from(ExtInstr::WriteAddrLit(0x1000, 0x01234567).to_instr());
    program.append_all(ExtInstr::WriteAddrLit(0x1008, 0x89ABCDEF).to_instr());
    program.append(Instr::HALT());

    let mut vm = prepare_vm(vec![
        Box::new(BootROM::with_size(program.encode_words(), 0x1000, 0x0).unwrap()),
        Box::new(VolatileMem::new(0x1000, 0x1).unwrap())
    ]);

    run_vm(&mut vm.cpu(), &RunConfig::new());

    let (mut err_a, mut err_b, mut err_c) = (0, 0, 0);

    let (word_a, word_b, word_c) = vm.map(|mut mem|
        (mem.read(0x1000, &mut err_a), mem.read(0x1008, &mut err_b), mem.read(0x1010, &mut err_c))
    );

    assert_eq!(err_a, 0, "Hardware exception occurred while reading word at address 0x00001000: {:#008X}", err_a);
    assert_eq!(err_b, 0, "Hardware exception occurred while reading word at address 0x00001008: {:#008X}", err_b);
    assert_eq!(err_c, 0, "Hardware exception occurred while reading word at address 0x00001010: {:#008X}", err_c);

    assert_eq!(word_a, 0x01234567, "Expected word at address 0x00001000 to contain 0x01234567 but it actually contains {:#010X}", word_a);
    assert_eq!(word_b, 0x89ABCDEF, "Expected word at address 0x00001008 to contain 0x89ABCDEF but it actually contains {:#010X}", word_b);
    assert_eq!(word_c, 0x00000000, "Expected word at address 0x00001010 to contain 0x01234567 but it actually contains {:#010X}", word_c);
}
