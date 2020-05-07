use std::convert::TryInto;
use std::str::{from_utf8, Utf8Error};
use mrvm::board::Bus;

pub struct BufferedDisplay {
    buffer: Vec<u32>,
    words: u32,
    handler: Box<dyn FnMut(Result<&str, Utf8Error>)>
}

impl BufferedDisplay {
    pub fn new(capacity: u32, handler: Box<dyn FnMut(Result<&str, Utf8Error>)>) -> Self {
        let _: usize = capacity.try_into().expect("Display's buffer's capacity must not exceed your CPU architecture (e.g. 32-bit size)");

        assert!(capacity % 4 == 0, "Buffered display's capacity must be aligned");
        assert!(capacity != 0, "Buffered display's capacity cannot be 0");

        let capacity = capacity / 4;

        Self {
            buffer: vec![0; (capacity - 1) as usize],
            words: capacity - 1,
            handler
        }
    }
}

impl Bus for BufferedDisplay {
    fn name(&self) -> &'static str {
        "Buffered Display"
    }

    fn size(&self) -> u32 {
        self.words * 4 + 4
    }

    fn read(&mut self, _addr: u32) -> u32 {
        eprintln!("Warning: tried to read from buffered display");
        0
    }

    fn write(&mut self, addr: u32, word: u32) {
        let addr = addr / 4;

        if addr < self.words {
            self.buffer[addr as usize] = word;
        }

        else if addr == self.words {
            match word {
                0xAAAAAAAA => {
                    // NOTE: `self.buffer.iter().flat_map(|word| word.to_be_bytes().iter())` results in a borrowing error
                    //        so below is the most simple algorithm I came up with.

                    let mut bytes = vec![];
                    for word in &self.buffer {
                        bytes.extend_from_slice(&word.to_be_bytes());
                    }

                    (self.handler)(from_utf8(&bytes))
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