use mrvm::board::MotherBoard;
use mrvm_tools::asm::{Program, Instr, Reg};
use crate::storage::BootROM;
use crate::tests::lib::{prepare_vm, run_until_halt};

fn prepare(instr: Instr, panic_on_invalid: bool) -> MotherBoard {
    let prog = Program::from(vec![
        instr,
        Instr::HALT()
    ]);

    prepare_vm(vec![
        Box::new(BootROM::with_size(prog.encode_words(), 0x1000).unwrap().set_panic_on_invalid(panic_on_invalid))
    ])
}

#[test]
fn bootrom_read() {
    let mut vm = prepare(Instr::CPY(Reg::a0, 0xABCD_u16.into()), false);

    let cpu = &mut vm.cpu();

    run_until_halt(cpu);

    assert_eq!(cpu.cycles(), 2, "CPU was expected to run {} cycles, {} cycles run instead", 2, cpu.cycles());
    assert_eq!(cpu.regs.a[0], 0xABCD, "Registry a0 was expected to contain 0x0000ABCD, contains {:#010X} instead", cpu.regs.a[0]);
}

#[test]
#[should_panic(expected = "Error: attempted to write the BootROM")]
fn bootrom_write() {
    let mut vm = prepare(Instr::WEA(0u8.into(), 0u8.into(), 0u8.into()), true);

    run_until_halt(&mut vm.cpu());
}
