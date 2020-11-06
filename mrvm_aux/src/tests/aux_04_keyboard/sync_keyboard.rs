use crate::keyboard::SyncKeyboard;
use crate::storage::BootROM;
use mrvm_tools::asm::{ExtInstr, Instr, Program, Reg};
use mrvm_tools::debug::{exec_vm, RunConfig};
use std::sync::{Arc, Mutex};

static PLACEHOLDER_KEYB_INPUT: &str = "Placeholder keyboard input";

fn keyb_prog(input_end_addr: u32) -> Program {
    let mut prog = Program::from(ExtInstr::SetReg(Reg::ac0, input_end_addr).to_instr());
    prog.append_all(ExtInstr::SetReg(Reg::avr, 0xAA).to_instr());
    prog.append(Instr::Wea(Reg::ac0.into(), 0_u8.into(), 0_u8.into()));

    prog
}

#[test]
fn sync_keyboard() {
    let mut prog = keyb_prog(0x1100 - 0x04);
    prog.append(Instr::Halt());

    #[allow(clippy::mutex_atomic)]
    let received_req = Arc::new(Mutex::new(false));
    let received_req_closure = Arc::clone(&received_req);

    let (mut vm, state) = exec_vm(
        vec![
            Box::new(BootROM::with_size(prog.encode_words(), 0x1000, 0x0).unwrap()),
            Box::new(
                SyncKeyboard::new(
                    0x100,
                    Box::new(move || {
                        let mut received_req = received_req_closure.lock().unwrap();
                        assert!(!*received_req, "Received a keyboard request twice");
                        *received_req = true;

                        Ok(String::from(PLACEHOLDER_KEYB_INPUT))
                    }),
                    0x1,
                )
                .unwrap(),
            ),
        ],
        &RunConfig::halt_on_ex(),
    );

    if state.ex.is_some() {
        panic!("Unexpected exception occurred while running the VM!");
    }

    assert!(
        *received_req.lock().unwrap(),
        "No keyboard request was triggered"
    );

    vm.map(|mem| {
        let mut bytes = vec![];

        let mut ex = 0;

        for addr_r in 0x1000 / 4..=(0x1100 - 4) / 4 {
            bytes.extend(&mem.read(addr_r * 4, &mut ex).to_be_bytes());
            assert_eq!(
                ex,
                0,
                "Exception occurred while reading word at address {:#010X}: {:#008X}",
                addr_r * 4,
                ex
            );
        }

        let string = String::from_utf8(bytes).expect("Received invalid UTF-8 string from keyboard");
        let string = string.trim_end_matches(char::from(0));

        assert_eq!(
            string, PLACEHOLDER_KEYB_INPUT,
            "Invalid string from keyboard: {}",
            string
        );
    });
}
