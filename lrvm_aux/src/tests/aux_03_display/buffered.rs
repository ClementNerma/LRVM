use crate::display::BufferedDisplay;
use crate::storage::BootRom;
use lrvm_tools::asm::{ExtInstr, Instr, Program, Reg};
use lrvm_tools::debug::{exec_vm, RunConfig};
use std::sync::{Arc, Mutex};

fn display_prog(text: &str, display_addr: u32, display_final_addr: u32) -> Result<Program, ()> {
    let mut instr = ExtInstr::SetReg(Reg::ac0, display_addr).to_instr();
    instr.push(Instr::Cpy(Reg::avr, 0_u8.into()));

    let mut byte_index = 0;

    let text_bytes = text.bytes();

    if text_bytes.len() as u64 > (display_final_addr - display_addr) as u64 {
        return Err(());
    }

    for byte in text_bytes {
        instr.push(Instr::Add(Reg::avr, byte.into()));
        byte_index += 1;

        if byte_index < 4 {
            instr.push(Instr::Shl(Reg::avr, 8_u8.into()));
        } else {
            instr.push(Instr::Wea(Reg::ac0.into(), 0_u8.into(), 0_u8.into()));
            instr.push(Instr::Add(Reg::ac0, 4_u8.into()));
            instr.push(Instr::Cpy(Reg::avr, 0_u8.into()));
            byte_index = 0;
        }
    }

    if byte_index != 0 {
        instr.push(Instr::Wea(Reg::ac0.into(), 0_u8.into(), 0_u8.into()));
    }

    instr.extend_from_slice(&ExtInstr::WriteAddrLit(display_final_addr, 0xAA).to_instr());

    Ok(Program::from_instr(instr))
}

#[test]
fn buffered_display() {
    let mut prog = display_prog("Hello world!", 0x1000, 0x1100 - 0x04).unwrap();
    prog.append(Instr::Halt().into());

    #[allow(clippy::mutex_atomic)]
    let received_msg = Arc::new(Mutex::new(false));
    let received_msg_closure = Arc::clone(&received_msg);

    let (_, state) = exec_vm(
        vec![
            Box::new(BootRom::with_size(prog.encode_words(), 0x1000, 0x0).unwrap()),
            Box::new(
                BufferedDisplay::new(
                    0x100,
                    Box::new(move |msg| {
                        let mut received_msg = received_msg_closure.lock().unwrap();

                        assert!(
                            !*received_msg,
                            "Received a message twice (second message: {})",
                            msg.unwrap_or("<Invalid UTF-8 string>")
                        );

                        let msg = msg
                            .expect("Invalid UTF-8 message received")
                            .trim_end_matches(char::from(0));
                        assert_eq!(msg, "Hello world!", "Invalid message received: {}", msg);

                        *received_msg = true;
                    }),
                    0x1,
                )
                .unwrap(),
            ),
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
