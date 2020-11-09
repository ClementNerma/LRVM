use crate::storage::BootROM;
use mrvm::board::MotherBoard;
use mrvm_tools::asm::{Instr, Program, Reg};
use mrvm_tools::debug::{prepare_vm, run_vm, RunConfig};
use mrvm_tools::exceptions::{AuxHwException, NativeException};

fn prepare(instr: Instr) -> MotherBoard {
    let prog = Program::from_instr(vec![instr, Instr::Halt()]);

    prepare_vm(vec![Box::new(
        BootROM::with_size(prog.encode_words(), 0x1000, 0x0).unwrap(),
    )])
}

#[test]
fn bootrom_read() {
    let mut vm = prepare(Instr::Cpy(Reg::a0, 0xABCD_u16.into()));

    let cpu = &mut vm.cpu();

    let status = run_vm(cpu, RunConfig::new());

    assert_eq!(
        status.cycles, 2,
        "CPU was expected to run {} cycles, {} cycles run instead",
        2, status.cycles
    );
    assert_eq!(
        cpu.regs.a[0], 0xABCD,
        "Registry a0 was expected to contain 0x0000ABCD, contains {:#010X} instead",
        cpu.regs.a[0]
    );
}

#[test]
fn bootrom_write() {
    let mut vm = prepare(Instr::Wea(0u8.into(), 0u8.into(), 0u8.into()));
    let ex = run_vm(&mut vm.cpu(), RunConfig::halt_on_ex())
        .ex
        .expect("No exception occurred while writing BootROM");

    match NativeException::decode_parts(ex.code, ex.associated) {
        Ok(NativeException::HardwareException(AuxHwException::MemoryNotWritable)) => {}
        Ok(NativeException::HardwareException(hw_ex)) => panic!(
            "Wrong hardware exception occurred while writing BootROM: {}",
            hw_ex
        ),
        Ok(ex) => panic!(
            "Expected hardware exception while writing BootROM, got non-hardware exception: {}",
            ex
        ),
        Err(_) => panic!("Unknown exception occurred while writing BootROM: {:?}", ex),
    }
}
