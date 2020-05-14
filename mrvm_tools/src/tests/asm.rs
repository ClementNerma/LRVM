use crate::asm::*;

#[test]
fn typed_asm_test() {
    let prog = Program::from(vec![
        Instr::ADD(Reg::a0, 0xFFu8.into()),
        Instr::SUB(Reg::a0, 0xFFu8.into()),
        Instr::DIV(Reg::a0, 0x00u8.into(), cst::DIV_ZRO_MIN.into()),
        Instr::MOD(Reg::a0, 0x00u8.into(), cst::DIV_ZRO_MIN.into()),
        Instr::JPR(RegOrLit2::from(-80i16)),
    ]);

    let encoded = prog.encode();

    assert_eq!(encoded, vec![
        0x1C, 0x00, 0x00, 0xFF,
        0x24, 0x00, 0x00, 0xFF,
        0x34, 0x00, 0x00, 0x04,
        0x3C, 0x00, 0x00, 0x04,
        0x70, 0xFF, 0xB0, 0x00,
    ], "Encoded program is invalid");

    let re_prog = Program::decode(encoded).expect("Failed to decode encoded program");

    assert_eq!(re_prog, prog, "Raw and re-encoded program are different");
}
