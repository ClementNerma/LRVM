use std::time::Instant;

use lrvm_aux::storage::BootRom;
use lrvm_tools::debug::{exec_vm, RunConfig};
use lrvm_tools::lasm::assemble_words;

fn main() {
    if cfg!(debug_assertions) {
        println!("WARNING: It seems like the benchmark is running in debug mode.");
        println!("This will hugely decrease performances.");
    }

    let program = assemble_words(include_str!("source.lasm"))
        .unwrap_or_else(|err| panic!("Failed to assemble demo program: {}", err));

    let time = Instant::now();

    let (mut motherboard, state) = exec_vm(
        vec![Box::new(BootRom::new(program, 0x0).unwrap())],
        RunConfig::quiet().with_halt_on_exception(true),
    );

    if state.ex.is_some() {
        eprintln!("ERROR: Benchmark program failed (see above).");
        return;
    }

    let ended = time.elapsed().as_micros();
    let cycles = motherboard.cpu().cycles();

    println!("Benchmark completed in   : {} ms", ended / 1000);
    println!("Number of cycles         : {} cycles", cycles);

    let cycles_per_second = cycles as f64 * (1_000_000.0 / ended as f64);

    let freq = cycles_per_second / 1_000_000.0;

    println!();
    println!(
        "Running speed is (~) {:.2} MIPS (Million Instructions Per Second)",
        freq
    );
}
