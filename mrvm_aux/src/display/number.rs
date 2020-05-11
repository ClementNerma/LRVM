//! The number display component offers a extremely simple way to display directly unsigned 32-bit numbers.
//! See [`NumberDisplay`] for more details.

use mrvm::board::Bus;
use mrvm_tools::metadata::{DeviceMetadata, DisplayType};
use mrvm_tools::exceptions::AuxHwException;

/// The number display is a very simple 1-byte long and write-only component.  
/// When a word is written to its unique address, it calls the handler provided during the component's creation with the said word.  
/// This allows to simply output a number to debug for instance.
pub struct NumberDisplay {
    hw_id: u64,
    handler: Box<dyn FnMut(u32)>
}

impl NumberDisplay {
    /// Create a number display.  
    /// The handler can is supposed to display the provided number, but this is not required.
    pub fn new(handler: Box<dyn FnMut(u32)>, hw_id: u64) -> Self {
        Self { hw_id, handler }
    }

    /// Create a number display that pretty-prints the received numbers in hexadecimal.
    pub fn new_println(hw_id: u64) -> Self {
        Self {
            hw_id,
            handler: Box::new(|num| println!("[number display] {:#010X}", num))
        }
    }
}

impl Bus for NumberDisplay {
    fn name(&self) -> &'static str {
        "Number Display"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(self.hw_id, 4, DisplayType::Number.wrap(), 0, None).encode()
    }

    fn read(&mut self, _addr: u32, ex: &mut u16) -> u32 {
        *ex = AuxHwException::MemoryNotReadable.encode();
        0
    }

    fn write(&mut self, _addr: u32, word: u32, _ex: &mut u16) {
        (self.handler)(word);
    }

    fn reset(&mut self) { }
}
