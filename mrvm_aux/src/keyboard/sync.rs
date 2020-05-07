//! The synchronous keyboard component offers a simple UTF-8 input system.
//! See [`SyncKeyboard`] for more details.

use std::convert::TryInto;
use mrvm::board::Bus;

/// The keyboard works with a buffer and a handler. When it receives a read request, the data is read from the buffer.
/// Writing into the buffer is forbidden but writing to the last word of the component results in it interpreting the provided action code:
///
/// * `0xAA`: trigger a synchronous input and put the result in the buffer
/// * `0xFF`: clear the buffer's content
///
/// The buffer is guaranteed to contain valid UTF-8 data.
pub struct SyncKeyboard {
    buffer: Vec<u32>,
    words: u32,
    handler: Box<dyn FnMut() -> Result<String, ()>>
}

impl SyncKeyboard {
    /// Create a synchronous keyboard component.
    /// The provided capacity must be a multiple of 4, and 4 bytes will be substracted for handling the action code.
    /// This means a capacity of 64 bytes will allow 60 bytes of data or 15 words.
    /// Returns an error message if the capacity is 0, not a multiple or 4 bytes or too large for the running CPU architecture.
    pub fn new(capacity: u32, handler: Box<dyn FnMut() -> Result<String, ()>>) -> Result<Self, &'static str> {
        let _: usize = capacity.try_into()
            .map_err(|_| "Display's buffer's capacity must not exceed your CPU architecture (e.g. 32-bit size)")?;

        if capacity % 4 != 0 {
            return Err("Synchronous keyboard's buffer capacity must be aligned");
        }

        if capacity == 0 {
            return Err("Synchronous keyboard's buffer capacity cannot be 0");
        }


        let capacity = capacity / 4;

        Ok(Self {
            buffer: vec![0; (capacity - 1) as usize],
            words: capacity - 1,
            handler
        })
    }
}

impl Bus for SyncKeyboard {
    fn name(&self) -> &'static str {
        "Synchronous Keyboard"
    }

    fn size(&self) -> u32 {
        self.words * 4 + 4
    }

    fn read(&mut self, addr: u32) -> u32 {
        let addr = addr / 4;

        if addr == self.words {
            0
        } else {
            self.buffer[addr as usize]
        }
    }

    fn write(&mut self, addr: u32, word: u32) {
        if addr / 4 != self.words {
            eprintln!("Warning: tried to write to synchronous keyboard");
        } else {
            match word {
                0xAA => {
                    let mut word = 0;
                    let mut byte_index = 0;
                    let mut pos = 0;

                    for byte in (self.handler)().unwrap().bytes() {
                        word += (byte as u32) << ((3 - byte_index) * 8);
                        
                        if byte_index == 3 {
                            if pos >= self.buffer.len() {
                                eprintln!("Warning: input is too long for synchronous keyboard's buffer (max. {} bytes)", self.words * 4);
                                return ;
                            }

                            self.buffer[pos] = word;
                            pos += 1;
                            byte_index = 0;
                            word = 0;
                        } else {
                            byte_index += 1;
                        }
                    }

                    if byte_index > 0 {
                        if pos >= self.buffer.len() {
                            eprintln!("Warning: input is too long for synchronous keyboard's buffer (max. {} bytes)", self.words * 4);
                            return ;
                        }

                        self.buffer[pos] = word;
                    }
                },

                0xFF => self.reset(),

                code => eprintln!("Warning: unknown action code {:#010X} received by buffered display", code)
            }
        }
    }

    fn reset(&mut self) {
        self.buffer = vec![0; self.buffer.len()];
    }
}