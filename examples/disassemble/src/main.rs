#[cfg(test)]
mod tests;

use mrvm_tools::asm::Program;
use mrvm_tools::lasm::assemble;

static SOURCE: &str = include_str!("source.lasm");

pub fn re_assemble(source: &str) -> String {
    println!("> Assembling source program...");

    let assembled =
        assemble(source).unwrap_or_else(|err| panic!("Failed to assemble source program: {}", err));

    println!("> Decoding assembled program...");

    let decoded = Program::decode(assembled, true).unwrap_or_else(|(instr, err)| {
        panic!(
            "Failed to decode assembled instruction {}: {}",
            instr + 1,
            err
        )
    });

    println!("> Converting decoded program to LASM code...");

    decoded.to_lasm(false)
}

fn main() {
    println!("\n{}", re_assemble(SOURCE));
}
