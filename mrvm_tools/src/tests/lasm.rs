use crate::lasm;

static DEMO_ASM: &'static str = include_str!("demo.lasm");

#[test]
fn lasm_test() {
    let asm_bytes = lasm::assemble(DEMO_ASM)
        .unwrap_or_else(|r| panic!("Failed to assemble demo program: {}", r));

    assert!(asm_bytes.len() % 4 == 0, "Unaligned assembly output");

    assert_eq!(
        asm_bytes,
        vec![
            0x1C, 0x00, 0x00, 0xFF, 0x24, 0x00, 0x00, 0xFF, 0x34, 0x00, 0x00, 0x04, 0x3C, 0x00,
            0x00, 0x04, 0x70, 0xFF, 0xF0, 0x00,
        ],
        "Bad assembly output"
    );
}
