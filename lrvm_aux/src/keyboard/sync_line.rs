//! The synchronous line keyboard component offers a simple UTF-8 line reading system.
//! See [`SyncLineKeyboard`] for more details.

use lrvm::board::Bus;
use lrvm_tools::{
    exceptions::AuxHwException,
    metadata::{DeviceMetadata, KeyboardType},
};

/// The keyboard works with a buffer and a handler.
///
/// When it receives a read request, the data is read from the buffer.
///
/// Writing into the buffer is forbidden but writing to the last word of the component results in it interpreting the provided action code:
///
/// * `0xAA`: trigger a synchronous input and put the result in the buffer
/// * `0xFF`: clear the buffer's content
///
/// The buffer is guaranteed to contain valid UTF-8 data.
pub struct SyncLineKeyboard {
    buffer: Vec<u32>,
    capacity: u32,
    handler: Box<dyn FnMut() -> String>,
    hw_id: u64,
}

impl SyncLineKeyboard {
    /// Create a synchronous keyboard component.
    /// The provided capacity must be a multiple of 4, and 4 bytes will be substracted for handling the action code.
    /// This means a capacity of 64 bytes will allow 60 bytes of data or 15 words.
    /// Returns an error message if the capacity is 0, not a multiple or 4 bytes or too large for the running CPU architecture.
    pub fn new(
        capacity: u32,
        handler: Box<dyn FnMut() -> String>,
        hw_id: u64,
    ) -> Result<Self, &'static str> {
        let _: usize = capacity.try_into().map_err(|_| {
            "Display's buffer's capacity must not exceed your CPU architecture (e.g. 32-bit size)"
        })?;

        if capacity % 4 != 0 {
            return Err("Synchronous keyboard's buffer capacity must be aligned");
        }

        if capacity == 0 {
            return Err("Synchronous keyboard's buffer capacity cannot be 0");
        }

        let capacity = capacity / 4;

        Ok(Self {
            buffer: vec![0; (capacity - 1) as usize],
            capacity: capacity - 1,
            handler,
            hw_id,
        })
    }
}

impl Bus for SyncLineKeyboard {
    fn name(&self) -> &'static str {
        "Synchronous Line Keyboard"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(
            self.hw_id,
            self.capacity * 4 + 4,
            KeyboardType::ReadLineSynchronous.into(),
            None,
            None,
        )
        .encode()
    }

    fn read(&mut self, addr: u32, _ex: &mut u16) -> u32 {
        let addr = addr / 4;

        if addr == self.capacity {
            0
        } else {
            self.buffer[addr as usize]
        }
    }

    fn write(&mut self, addr: u32, word: u32, ex: &mut u16) {
        if addr / 4 != self.capacity {
            *ex = 0x31 << 8;
        } else {
            match word {
                0xAA => {
                    let mut word = 0;
                    let mut byte_index = 0;
                    let mut pos = 0;

                    for byte in (self.handler)().bytes() {
                        word += (byte as u32) << ((3 - byte_index) * 8);

                        if byte_index == 3 {
                            if pos >= self.buffer.len() {
                                eprintln!("Warning: input is too long for synchronous keyboard's buffer (max. {} bytes)", self.capacity * 4);
                                return;
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
                            eprintln!("Warning: input is too long for synchronous keyboard's buffer (max. {} bytes)", self.capacity * 4);
                            return;
                        }

                        self.buffer[pos] = word;
                    }
                }

                0xFF => self.reset(),

                code => *ex = AuxHwException::UnknownOperation(code as u8).into(),
            }
        }
    }

    fn reset(&mut self) {
        self.buffer = vec![0; self.buffer.len()];
    }
}
