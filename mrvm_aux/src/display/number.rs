//! The number display component offers a extremely simple way to display directly unsigned 32-bit numbers.
//! See [`NumberDisplay`] for more details.

use std::io::{stdout, Write};

use mrvm::board::Bus;
use mrvm_tools::exceptions::AuxHwException;
use mrvm_tools::metadata::{DeviceMetadata, DisplayType};

/// Formatting for the number display
#[derive(Debug, Clone, Copy)]
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

/// The number display is a very simple 8-word long and write-only component.  
/// When a word is written to one of its 8 unique addresses, it calls the handler provided during the component's creation
///   with the said word (the word index indicates the formatting to use for the number).  
/// This allows to simply output a number to debug for instance.
/// The first 4 addresses will print the number with a newline symbol at the end, while the 4 other won't print a newline.  
pub struct NumberDisplay {
    hw_id: u64,

    /// The parameters are the number to display, the format to use, and if a newline symbol should be printed afterwards.
    handler: Box<dyn FnMut(u32, NumberDisplayFormat, bool)>,
}

impl NumberDisplay {
    /// Create a number display.  
    /// The handler can is supposed to display the provided number, but this is not required.  
    /// The parameters are the number to display, the format to use, and if a newline symbol should be printed afterwards.
    pub fn new(handler: Box<dyn FnMut(u32, NumberDisplayFormat, bool)>, hw_id: u64) -> Self {
        Self { hw_id, handler }
    }

    /// Create a number display which print! the numbers
    pub fn new_print(hw_id: u64) -> Self {
        Self::new(
            Box::new(|num, format, newline| {
                match format {
                    NumberDisplayFormat::Hex => print!("{:#X}", num),
                    NumberDisplayFormat::HexLong => print!("{:#010X}", num),
                    NumberDisplayFormat::Dec => print!("{}", num),
                    NumberDisplayFormat::DecLong => print!("{:#010}", num),
                }

                if newline {
                    println!();
                }

                stdout().flush().expect("Failed to flush STDOUT");
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
            match addr % 16 {
                0 => NumberDisplayFormat::Hex,
                4 => NumberDisplayFormat::HexLong,
                8 => NumberDisplayFormat::Dec,
                12 => NumberDisplayFormat::DecLong,
                _ => unreachable!(),
            },
            addr < 16,
        );
    }

    fn reset(&mut self) {}
}
