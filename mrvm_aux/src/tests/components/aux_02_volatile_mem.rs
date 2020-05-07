use mrvm_tools::asm::{Program, Instr, ExtInstr};
use crate::storage::BootROM;
use crate::memory::VolatileMem;
use crate::tests::lib::{prepare_vm, run_until_halt};

#[test]
fn volatile_mem() {
    let mut program = Program::from(ExtInstr::WriteAddrLit(0x1000, 0x01234567).to_instr());
    program.append_all(ExtInstr::WriteAddrLit(0x1008, 0x89ABCDEF).to_instr());
    program.append(Instr::HALT());

    let mut vm = prepare_vm(vec![
        Box::new(BootROM::with_size(program.encode_words(), 0x1000).unwrap()),
        Box::new(VolatileMem::new(0x1000).unwrap())
    ]);

    run_until_halt(&mut vm.cpu(), None);

    let (word_a, word_b, word_c) = vm.map(|mut mem|
        (mem.read(0x1000), mem.read(0x1008), mem.read(0x1010))
    );

    assert_eq!(word_a, 0x01234567, "Expected word at address 0x00001000 to contain 0x01234567 but it actually contains {:#010X}", word_a);
    assert_eq!(word_b, 0x89ABCDEF, "Expected word at address 0x00001008 to contain 0x89ABCDEF but it actually contains {:#010X}", word_b);
    assert_eq!(word_c, 0x00000000, "Expected word at address 0x00001010 to contain 0x01234567 but it actually contains {:#010X}", word_c);
}
