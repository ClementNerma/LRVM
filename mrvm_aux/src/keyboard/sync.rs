use std::convert::TryInto;
use mrvm::board::Bus;

pub struct SyncKeyboard {
    buffer: Vec<u32>,
    words: u32,
    handler: Box<dyn FnMut() -> Result<String, ()>>
}

impl SyncKeyboard {
    pub fn new(capacity: u32, handler: Box<dyn FnMut() -> Result<String, ()>>) -> Self {
        let _: usize = capacity.try_into().expect("Display's buffer's capacity must not exceed your CPU architecture (e.g. 32-bit size)");

        assert!(capacity % 4 == 0, "Synchrone keyboard's buffer capacity must be aligned");
        assert!(capacity != 0, "Synchrone keyboard's buffer capacity cannot be 0");

        let capacity = capacity / 4;

        Self {
            buffer: vec![0; (capacity - 1) as usize],
            words: capacity - 1,
            handler
        }
    }
}

impl Bus for SyncKeyboard {
    fn name(&self) -> &'static str {
        "Synchrone Keyboard"
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
                0xAAAAAAAA => {
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

                0xFFFFFFFF => self.reset(),

                code => eprintln!("Warning: unknown action code {:#010X} received by buffered display", code)
            }
        }
    }

    fn reset(&mut self) {
        self.buffer = vec![0; self.buffer.len()];
    }
}