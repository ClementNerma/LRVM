//! The buffered display component offers a simple UTF-8 display system.
//! See [`BufferedDisplay`] for more details.

use std::convert::TryInto;
use std::str::{from_utf8, Utf8Error};
use mrvm::board::Bus;
use mrvm_tools::bytes::words_to_bytes;
use mrvm_tools::metadata::{DeviceMetadata, DisplayType};
use mrvm_tools::exceptions::AuxHwException;

/// The buffered display works with a buffer and a handler. When it receives a write request, it writes it into the buffer unless the
/// write address is on its last word ; in this case, in interprets the word as:
///
/// * `0xAA`: display the buffer's content and clear it afterwards
/// * `0xBB`: display the buffer's content lossiliy (handles invalid UTF-8 characters) and clear it afterwards
/// * `0xFF`: clear the buffer's content
///
/// The buffer may contain invalid UTF-8 data. When a display request is received, the handler is called with the decoded UTF-8 string,
/// which is a result object handling either the valid UTF-8 string or a decoding error object with the faulty raw buffer's content.
pub struct BufferedDisplay {
    buffer: Vec<u32>,
    words: u32,
    handler: Box<dyn FnMut(Result<&str, (Utf8Error, Vec<u8>)>)>,
    hw_id: u64
}

impl BufferedDisplay {
    /// Create a buffered display component.
    /// The provided capacity must be a multiple of 4, and 4 bytes will be substracted for handling the action code.
    /// This means a capacity of 64 bytes will allow 60 bytes of data or 15 words.
    /// Returns an error message if the capacity is 0, not a multiple or 4 bytes or too large for the running CPU architecture.
    pub fn new(capacity: u32, handler: Box<dyn FnMut(Result<&str, (Utf8Error, Vec<u8>)>)>, hw_id: u64) -> Result<Self, &'static str> {
        let _: usize = capacity.try_into()
            .map_err(|_| "Display's buffer's capacity must not exceed your CPU architecture (e.g. 32-bit size)")?;

        if capacity == 0 {
            return Err("Buffered display's capacity cannot be 0");
        }

        if capacity % 4 != 0 {
            return Err("Buffered display's capacity must be aligned");
        }

        let capacity = capacity / 4;

        Ok(Self {
            buffer: vec![0; (capacity - 1) as usize],
            words: capacity - 1,
            handler,
            hw_id
        })
    }
}

impl Bus for BufferedDisplay {
    fn name(&self) -> &'static str {
        "Buffered Display"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(
            self.hw_id,
            self.words * 4 + 4,
            DisplayType::Buffered.into(),
            0x00000000,
            None
        ).encode()
    }

    fn read(&mut self, _addr: u32, ex: &mut u16) -> u32 {
        *ex = AuxHwException::MemoryNotReadable.into();
        0
    }

    fn write(&mut self, addr: u32, word: u32, ex: &mut u16) {
        let addr = addr / 4;

        if addr < self.words {
            self.buffer[addr as usize] = word;
        }

        else if addr == self.words {
            match word {
                0xAA => {
                    let bytes = words_to_bytes(&self.buffer);
                    (self.handler)(from_utf8(&bytes).map_err(|err| (err, bytes.clone())))
                },

                0xBB => {
                    let bytes = words_to_bytes(&self.buffer);
                    (self.handler)(Ok(&String::from_utf8_lossy(&bytes)))
                },

                0xFF => self.reset(),

                code => *ex = AuxHwException::UnknownOperation(code as u8).into()
            }
        }
    }

    fn reset(&mut self) {
        self.buffer = vec![0; self.buffer.len()];
    }
}