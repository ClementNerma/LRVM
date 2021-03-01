//! The basic debug component offers a simple debugging system.
//! See [`BasicDebug`] for more details.
use lrvm::board::Bus;
use lrvm_tools::exceptions::AuxHwException;
use lrvm_tools::metadata::{DebugType, DeviceCategory, DeviceMetadata};

/// The Basic Debug Interface (BDI) is a simple debug tool that provides a set of writable words to debug informations:
///
/// * Word  0: Unsigned byte (as hexadecimal)
/// * Word  1: Unsigned half-word (as hexadecimal)
/// * Word  2: Unsigned word (as hexadecimal)
/// * Word  3: Signed byte (as hexadecimal)
/// * Word  4: Signed half-word (as hexadecimal)
/// * Word  5: Signed word (as hexadecimal)
/// * Word  6: Unsigned byte (as decimal)
/// * Word  7: Unsigned half-word (as decimal)
/// * Word  8: Unsigned word (as decimal)
/// * Word  9: Signed byte (as decimal)
/// * Word 10: Signed half-word (as decimal)
/// * Word 11: Signed word (as decimal)
/// * Word 12: Boolean
/// * Word 13: UTF-8 character
/// * Word 14: Encoding-agnostic character
/// * Word 15: "DEBUG" message
pub struct BasicDebug {
    hw_id: u64,
    debugger: Box<dyn FnMut(DebugInfo)>,
}

/// An information to debug
pub enum DebugInfo {
    UnsignedByteHex(u8),
    UnsignedHalfWordHex(u16),
    UnsignedWordHex(u32),
    SignedByteHex(i8),
    SignedHalfWordHex(i16),
    SignedWordHex(i32),
    UnsignedByteDec(u8),
    UnsignedHalfWordDec(u16),
    UnsignedWordDec(u32),
    SignedByteDec(i8),
    SignedHalfWordDec(i16),
    SignedWordDec(i32),
    Boolean(bool),
    Utf8Char(Result<char, u32>),
    EncodingAgnosticChar(u32),
    DebugMessage,
}

impl BasicDebug {
    /// Create a new Basic Debug Interface (BDI).  
    /// The debugger is a function that takes an information to debug, see [`DebugInfo`] for more details.
    pub fn new(hw_id: u64, debugger: Box<dyn FnMut(DebugInfo)>) -> Self {
        Self { hw_id, debugger }
    }

    /// Create a new Basic Debug Interface (BDI) with a println!-backed debugger
    pub fn new_println(hw_id: u64) -> Self {
        Self::new(
            hw_id,
            Box::new(|info| {
                println!(
                    "[debug:basic] {}",
                    match info {
                        DebugInfo::UnsignedByteHex(n) => format!("{:#004X}", n),
                        DebugInfo::UnsignedHalfWordHex(n) => format!("{:#006X}", n),
                        DebugInfo::UnsignedWordHex(n) => format!("{:#010X}", n),
                        DebugInfo::SignedByteHex(n) => {
                            if n < 0 {
                                format!("-{:#004X}", -n)
                            } else {
                                format!("{:#004X}", n)
                            }
                        }
                        DebugInfo::SignedHalfWordHex(n) => {
                            if n < 0 {
                                format!("-{:#006X}", -n)
                            } else {
                                format!("{:#006X}", n)
                            }
                        }
                        DebugInfo::SignedWordHex(n) => {
                            if n < 0 {
                                format!("-{:#010X}", -n)
                            } else {
                                format!("{:#010X}", n)
                            }
                        }
                        DebugInfo::UnsignedByteDec(n) => format!("{}", n),
                        DebugInfo::UnsignedHalfWordDec(n) => format!("{}", n),
                        DebugInfo::UnsignedWordDec(n) => format!("{}", n),
                        DebugInfo::SignedByteDec(n) => format!("{}", n),
                        DebugInfo::SignedHalfWordDec(n) => format!("{}", n),
                        DebugInfo::SignedWordDec(n) => format!("{}", n),
                        DebugInfo::Boolean(b) => format!("{}", b),
                        DebugInfo::Utf8Char(c) => match c {
                            Ok(c) => format!("{}", c),
                            Err(code) => format!("<Invalid UTF-8 character: {:#010X}>", code),
                        },
                        DebugInfo::EncodingAgnosticChar(c) => {
                            format!("<Encoding-agnostic character: {:#010X}>", c)
                        }
                        DebugInfo::DebugMessage => String::from("debug point"),
                    }
                )
            }),
        )
    }
}

impl Bus for BasicDebug {
    fn name(&self) -> &'static str {
        "Basic Debug Interface"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(
            self.hw_id,
            32,
            DeviceCategory::Debug(DebugType::Basic),
            None,
            None,
        )
        .encode()
    }

    fn read(&mut self, _addr: u32, ex: &mut u16) -> u32 {
        *ex = AuxHwException::MemoryNotReadable.encode();
        0
    }

    fn write(&mut self, addr: u32, word: u32, _ex: &mut u16) {
        let info = match addr / 4 {
            0 => DebugInfo::UnsignedByteHex(word as u8),
            1 => DebugInfo::UnsignedHalfWordHex(word as u16),
            2 => DebugInfo::UnsignedWordHex(word),
            3 => DebugInfo::SignedByteHex(word as i8),
            4 => DebugInfo::SignedHalfWordHex(word as i16),
            5 => DebugInfo::SignedWordHex(word as i32),
            6 => DebugInfo::UnsignedByteDec(word as u8),
            7 => DebugInfo::UnsignedHalfWordDec(word as u16),
            8 => DebugInfo::UnsignedWordDec(word),
            9 => DebugInfo::SignedByteDec(word as i8),
            10 => DebugInfo::SignedHalfWordDec(word as i16),
            11 => DebugInfo::SignedWordDec(word as i32),
            12 => DebugInfo::Boolean(word != 0),
            13 => DebugInfo::Utf8Char(std::char::from_u32(word).ok_or(word)),
            14 => DebugInfo::EncodingAgnosticChar(word),
            15 => DebugInfo::DebugMessage,
            _ => unreachable!("Tried to write to out-of-range address in basic debug interface"),
        };

        (self.debugger)(info);
    }

    fn reset(&mut self) {}
}
