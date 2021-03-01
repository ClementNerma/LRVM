use crate::display::CharDisplay;
use crate::storage::BootRom;
use lrvm_tools::asm::{ExtInstr, Instr, Program};
use lrvm_tools::debug::{exec_vm, RunConfig};
use std::sync::{Arc, Mutex};

fn display_prog(character: char, display_addr: u32) -> Program {
    Program::from_instr(ExtInstr::WriteAddrLit(display_addr, character as u32).to_instr())
}

#[test]
fn buffered_display() {
    let mut prog = display_prog('Z', 0x1000);
    prog.append(Instr::Halt().into());

    #[allow(clippy::mutex_atomic)]
    let received_msg = Arc::new(Mutex::new(false));
    let received_msg_closure = Arc::clone(&received_msg);

    let (_, state) = exec_vm(
        vec![
            Box::new(BootRom::with_size(prog.encode_words(), 0x1000, 0x0).unwrap()),
            Box::new(CharDisplay::new(
                Box::new(move |msg| {
                    let mut received_msg = received_msg_closure.lock().unwrap();

                    assert!(
                        !*received_msg,
                        "Received a message twice (second message: {})",
                        msg.map(String::from)
                            .unwrap_or_else(|_| String::from("<Invalid UTF-8 character>"))
                    );

                    let msg = msg.expect("Invalid UTF-8 character received");

                    assert_eq!(msg, 'Z', "Invalid character received: {}", msg);

                    *received_msg = true;
                }),
                0x1,
            )),
        ],
        RunConfig::halt_on_ex(),
    );

    if state.ex.is_some() {
        panic!("Unexpected exception occurred while running the VM!");
    }

    assert!(
        *received_msg.lock().unwrap(),
        "No message received by buffered display"
    );
}
