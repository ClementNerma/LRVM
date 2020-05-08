use mrvm_tools::asm::{Program, Instr, Reg};
use crate::storage::BootROM;
use crate::tests::lib::{prepare_vm, run_until_halt};

#[test]
fn bootrom() {
    let prog = Program::from(vec![
        Instr::CPY(Reg::a0, 0xABCD_u16.into()),
        Instr::HALT()
    ]);

    let mut vm = prepare_vm(vec![
        Box::new(BootROM::with_size(prog.encode_words(), 0x1000).unwrap())
    ]);

    let cpu = &mut vm.cpu();

    run_until_halt(cpu);

    assert_eq!(cpu.cycles(), 2, "CPU was expected to run {} cycles, {} cycles run instead", 2, cpu.cycles());
    assert_eq!(cpu.regs.a[0], 0xABCD, "Registry a0 was expected to contain 0x0000ABCD, contains {:#010X} instead", cpu.regs.a[0]);
}
