use super::{SOURCE, re_assemble};

static EXPECTED_LASM: &str = "
zro a0
add a0, 0x10
shl a0, 0x4
shr a0, 0x4
sub a0, 0x10
div a1, a0, DIV_ZRO_MIN
sub a2, ac2
cmp a1, 0x0
ifeq
call 0x40
ifle
call 0x44
ifleft ZF, EF
call 0x4C
ifxor EF, ZF
call 0x54
halt
ret
add a0, a1
ret
inc a0
ret
dec a0
jp 0x10
ret
";

#[test]
fn re_assembling() {
    let lasm = EXPECTED_LASM.to_string();
    let lasm = lasm.trim().split("\n").collect::<Vec<_>>();

    let assembled = re_assemble(SOURCE);
    let assembled = assembled.trim().split("\n").collect::<Vec<_>>();
    
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