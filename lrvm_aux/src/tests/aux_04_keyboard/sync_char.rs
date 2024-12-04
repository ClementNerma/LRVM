use std::sync::{Arc, Mutex};

use lrvm_tools::{
    asm::{ExtInstr, Instr, Program, Reg},
    debug::{exec_vm, RunConfig},
};

use crate::{keyboard::SyncCharKeyboard, storage::BootRom};

static PLACEHOLDER_KEYB_INPUT: char = 'Z';

fn keyb_prog(input_end_addr: u32) -> Program {
    let mut prog = Program::from_instr(ExtInstr::SetReg(Reg::ac0, input_end_addr).to_instr());
    prog.append_all(ExtInstr::SetReg(Reg::avr, 0x01).to_prog_words());
    prog.append(Instr::Wea(Reg::ac0.into(), 0_u8.into(), 0_u8.into()).into());

    prog
}

#[test]
fn sync_char() {
    let mut prog = keyb_prog(0x1004);
    prog.append(Instr::Halt().into());

    #[allow(clippy::mutex_atomic)]
    let received_req = Arc::new(Mutex::new(false));
    let received_req_closure = Arc::clone(&received_req);

    let (mut vm, state) = exec_vm(
        vec![
            Box::new(BootRom::with_size(prog.encode_words(), 0x1000, 0x0).unwrap()),
            Box::new(SyncCharKeyboard::new(
                Box::new(move || {
                    let mut received_req = received_req_closure.lock().unwrap();
                    assert!(!*received_req, "Received a keyboard request twice");
                    *received_req = true;

                    PLACEHOLDER_KEYB_INPUT
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
        *received_req.lock().unwrap(),
        "No keyboard request was triggered"
    );

    vm.map(|mem| {
        let mut ex = 0;

        let word = mem.read(0x1000, &mut ex);

        assert_eq!(
            ex, 0,
            "Exception occurred while reading word at address 0x1000: {:#008X}",
            ex
        );

        let character = std::char::from_u32(word)
            .unwrap_or_else(|| panic!("Got invalid character code from keyboard: {:#004X}", word));

        assert_eq!(
            character, PLACEHOLDER_KEYB_INPUT,
            "Invalid character from keyboard: {}",
            character
        );
    });
}
