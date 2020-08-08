use crate::asm::*;

fn prog() -> Program {
    Program::from(vec![
        Instr::Add(Reg::a0, 0xFFu8.into()),
        Instr::Sub(Reg::a0, 0xFFu8.into()),
        Instr::Div(Reg::a0, 0x00u8.into(), cst::DIV_ZRO_MIN.into()),
        Instr::Mod(Reg::a0, 0x00u8.into(), (cst::DIV_ZRO_MIN | cst::DIV_OFW_MAX).into()),
        Instr::Jpr(RegOrLit2::from(-80i16)),
    ])
}

fn encoded() -> Vec<u8> {
    vec![
        0x1C, 0x00, 0x00, 0xFF,
        0x24, 0x00, 0x00, 0xFF,
        0x34, 0x00, 0x00, 0x04,
        0x3C, 0x00, 0x00, 0x07,
        0x70, 0xFF, 0xB0, 0x00,
    ]
}

fn assembled() -> Vec<&'static str> {
    vec![
        "add a0, 0xFF",
        "sub a0, 0xFF",
        "div a0, 0x0, DIV_ZRO_MIN",
        "mod a0, 0x0, DIV_ZRO_MIN | DIV_OFW_MAX",
        "jpr -0x50"
    ]
}

#[test]
fn encoding() {
    assert_eq!(prog().encode(), encoded(), "Encoded program is not valid");
}

#[test]
fn decoding() {
    let re_prog = Program::decode(encoded()).expect("Failed to decode encoded program");
    assert_eq!(re_prog, prog(), "Original and re-encoded program are different");
}

#[test]
fn asm_conversion() {
    let lasm = prog().to_lasm_lines();
    let assembled = assembled();
    
    if lasm.len() != assembled.len() {
        panic!(
            "Assembled code is {} than expected.\nExpected:\n\n{}\n\nGot:\n\n{}",
            if lasm.len() > assembled.len() { "greater" } else { "smaller" },
            assembled.join("\n"),
            lasm.join("\n")
        );
    }

    for i in 0..lasm.iter().count() {
        if lasm[i] != assembled[i] {
            panic!("Assembled program differs from expected one.\nExpected: {}\nGot     : {}\nAt line {}.", assembled[i], lasm[i], i + 1);
        }
    }
}
