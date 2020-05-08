use mrvm::board::MotherBoard;
use mrvm_tools::asm::{Program, Instr, Reg};
use crate::storage::BootROM;
use crate::tests::lib::{prepare_vm, run_until_halt, run_until_halt_or_ex};

fn prepare(instr: Instr) -> MotherBoard {
    let prog = Program::from(vec![
        instr,
        Instr::HALT()
    ]);

    prepare_vm(vec![
        Box::new(BootROM::with_size(prog.encode_words(), 0x1000, 0x0).unwrap())
    ])
}

#[test]
fn bootrom_read() {
    let mut vm = prepare(Instr::CPY(Reg::a0, 0xABCD_u16.into()));

    let cpu = &mut vm.cpu();

    let cycles = run_until_halt(cpu).0;

    assert_eq!(cycles, 2, "CPU was expected to run {} cycles, {} cycles run instead", 2, cpu.cycles());
    assert_eq!(cpu.regs.a[0], 0xABCD, "Registry a0 was expected to contain 0x0000ABCD, contains {:#010X} instead", cpu.regs.a[0]);
}

#[test]
fn bootrom_write() {
    let mut vm = prepare(Instr::WEA(0u8.into(), 0u8.into(), 0u8.into()));
    let et_b = run_until_halt_or_ex(&mut vm.cpu()).expect_err("No exception occurred while writing BootROM").to_be_bytes();

    println!("{:#010X}", u32::from_be_bytes(et_b));

    assert_eq!(et_b[1], 0x10, "Unexpected exception occurred while writing BootROM: code {:#004X}, was expecting 0x10", et_b[1]);

    assert_eq!(et_b[2], 0x31, "Unexpected hardware exception occurred while writing BootROM: code {:#004X}, was expecting 0x31", et_b[2]);
}
