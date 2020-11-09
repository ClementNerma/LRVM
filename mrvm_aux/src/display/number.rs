//! The number display component offers a extremely simple way to display directly unsigned 32-bit numbers.
//! See [`NumberDisplay`] for more details.

use mrvm::board::Bus;
use mrvm_tools::exceptions::AuxHwException;
use mrvm_tools::metadata::{DeviceMetadata, DisplayType};

/// Formatting for the number display
pub enum NumberDisplayFormat {
    /// Display as hexadecimal
    Hex,
    /// Display as hexadecimal with '0' left padding
    HexLong,
    /// Display as decimal
    Dec,
    /// Display as decimal with '0' left padding
    DecLong,
}

/// The number display is a very simple 4-word long and write-only component.  
/// When a word is written to one of its unique address, it calls the handler provided during the component's creation
///   with the said word (the word index indicates the formatting to use for the number).  
/// This allows to simply output a number to debug for instance.
pub struct NumberDisplay {
    hw_id: u64,
    handler: Box<dyn FnMut(u32, NumberDisplayFormat)>,
}

impl NumberDisplay {
    /// Create a number display.  
    /// The handler can is supposed to display the provided number, but this is not required.
    pub fn new(handler: Box<dyn FnMut(u32, NumberDisplayFormat)>, hw_id: u64) -> Self {
        Self { hw_id, handler }
    }

    /// Create a number display which print! the numbers
    pub fn new_print(hw_id: u64) -> Self {
        Self::new(
            Box::new(|num, format| match format {
                NumberDisplayFormat::Hex => print!("{:#X}", num),
                NumberDisplayFormat::HexLong => print!("{:#010X}", num),
                NumberDisplayFormat::Dec => print!("{}", num),
                NumberDisplayFormat::DecLong => print!("{:#010}", num),
            }),
            hw_id,
        )
    }
}

impl Bus for NumberDisplay {
    fn name(&self) -> &'static str {
        "Number Display"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(self.hw_id, 16, DisplayType::Number.wrap(), None, None).encode()
    }

    fn read(&mut self, _addr: u32, ex: &mut u16) -> u32 {
        *ex = AuxHwException::MemoryNotReadable.encode();
        0
    }

    fn write(&mut self, addr: u32, word: u32, _ex: &mut u16) {
        (self.handler)(
            word,
            match addr {
                0 => NumberDisplayFormat::Hex,
                4 => NumberDisplayFormat::HexLong,
                8 => NumberDisplayFormat::Dec,
                12 => NumberDisplayFormat::DecLong,
                _ => unreachable!(),
            },
        );
    }

    fn reset(&mut self) {}
}
