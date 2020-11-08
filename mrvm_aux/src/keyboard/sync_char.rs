//! The synchronous character keyboard component offers a simple one-character reading system.
//! See [`SyncLineKeyboard`] for more details.

use mrvm::board::Bus;
use mrvm_tools::exceptions::AuxHwException;
use mrvm_tools::metadata::{DeviceMetadata, KeyboardType};

/// The keyboard works with a 1-word buffer and a handler. When it receives a read request, the data is read from the buffer.
/// Writing into the buffer is forbidden but writing to the second word of the component results in it interpreting the provided action code:
///
/// * `0xAA`: trigger a synchronous input and put the result in the buffer
/// * `0xFF`: clear the buffer's content
///
/// The buffer is guaranteed to contain a valid UTF-8 character.
pub struct SyncCharKeyboard {
    buffer: char,
    handler: Box<dyn FnMut() -> char>,
    hw_id: u64,
}

impl SyncCharKeyboard {
    /// Create a synchronous character keyboard component.
    pub fn new(handler: Box<dyn FnMut() -> char>, hw_id: u64) -> Self {
        Self {
            buffer: 0 as char,
            handler,
            hw_id,
        }
    }
}

impl Bus for SyncCharKeyboard {
    fn name(&self) -> &'static str {
        "Synchronous Character Keyboard"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(
            self.hw_id,
            8,
            KeyboardType::ReadCharSynchronous.into(),
            None,
            None,
        )
        .encode()
    }

    fn read(&mut self, addr: u32, ex: &mut u16) -> u32 {
        let addr = addr / 4;

        if addr == 0 {
            self.buffer as u32
        } else if addr == 1 {
            *ex = AuxHwException::MemoryNotReadable.into();
            0
        } else {
            unreachable!() // Safety guarantee
        }
    }

    fn write(&mut self, addr: u32, word: u32, ex: &mut u16) {
        if addr == 0 {
            *ex = 0x31 << 8;
        } else if addr == 1 {
            match word {
                0xAA => self.buffer = (self.handler)(),
                0xFF => self.reset(),
                code => *ex = AuxHwException::UnknownOperation(code as u8).into(),
            }
        } else {
            unreachable!() // Safety guarantee
        }
    }

    fn reset(&mut self) {
        self.buffer = 0 as char;
    }
}
