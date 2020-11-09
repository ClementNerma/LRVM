use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal;
use mrvm_aux::display::{BufferedDisplay, CharDisplay, NumberDisplay};
use mrvm_aux::keyboard::{SyncCharKeyboard, SyncLineKeyboard};
use mrvm_aux::storage::BootROM;
use mrvm_aux::volatile_mem::RAM;
use mrvm_tools::debug::{exec_vm, RunConfig};
use mrvm_tools::lasm::assemble_words;
use rand::Rng;

fn main() {
    let program = assemble_words(include_str!("source.lasm"))
        .unwrap_or_else(|err| panic!("Failed to assemble demo program: {}", err));

    let mut rng = rand::thread_rng();

    // BootROM containing the program
    let bootrom = BootROM::with_size(program, 0x1000, rng.gen()).unwrap();

    // The first RAM, for the stack (calls & the brainfuck program's loops)
    let stack = RAM::new(0x1000, rng.gen()).unwrap();

    // The second RAM, for the brainfuck program's memory
    let bf_memory = RAM::new(0x1000, rng.gen()).unwrap();

    // A buffered display, to allow the brainfuck program to display messages
    let display = BufferedDisplay::new_print_lossy(0x1000, rng.gen()).unwrap();

    // A synchronous line keyboard, to allow the brainfuck program to input strings
    let line_keyboard = SyncLineKeyboard::new(
        0x1000,
        Box::new(|| {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input
        }),
        rng.gen(),
    )
    .unwrap();

    // A synchronous character keyboard, to allow the brainfuck program to get single-key inputs
    let char_keyboard = SyncCharKeyboard::new(
        Box::new(|| {
            terminal::enable_raw_mode().unwrap();

            let c = loop {
                if let Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: _,
                }) = event::read().unwrap()
                {
                    break c;
                }
            };

            terminal::disable_raw_mode().unwrap();

            c
        }),
        rng.gen(),
    );

    // A character display, to allow the brainfuck program to display single characters
    let char_display = CharDisplay::new_print_lossy(rng.gen());

    // A number display, to allow the brainfuck program to display numbers without performing the number <=> string conversin
    let num_display = NumberDisplay::new_print(rng.gen());

    exec_vm(
        vec![
            Box::new(bootrom),
            Box::new(stack),
            Box::new(bf_memory),
            Box::new(display),
            Box::new(line_keyboard),
            Box::new(char_keyboard),
            Box::new(char_display),
            Box::new(num_display),
        ],
        RunConfig::halt_on_ex().with_print_finish(false),
    );
}
