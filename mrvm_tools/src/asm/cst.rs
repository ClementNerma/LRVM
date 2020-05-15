//! MRVM constants

/// Zero Flag
pub const ZF: u8 = 0x00;
/// Carry Flag
pub const CF: u8 = 0x01;
/// Overflow Flag
pub const OF: u8 = 0x02;
/// Sign Flag
pub const SF: u8 = 0x03;
/// Parity Flag
pub const PF: u8 = 0x04;
/// Zero-Upper Flag
pub const ZUF: u8 = 0x05;
/// Zero-Lower Flag
pub const ZLF: u8 = 0x06;

/// Division sign mode mask
pub const DIV_SIGN_MODE_MASK: u8 = 0b0001_0000;
/// Division by zero mode mask
pub const DIV_ZERO_MODE_MASK: u8 = 0b0000_1100;
/// Overflowing division mode mask
pub const DIV_OVFW_MODE_MASK: u8 = 0b0000_0011;

/// Unsigned division (mode)
pub const DIV_USG: u8 = 0;
/// Signed division (mode)
pub const DIV_SIG: u8 = 0b0001_0000;

/// Forbid division by zero (mode)
pub const DIV_ZRO_FRB: u8 = 0;
/// Make division by zero result in the minimum signed value (mode)
pub const DIV_ZRO_MIN: u8 = 0b0000_0100;
/// Make division by zero result in zero (mode)
pub const DIV_ZRO_ZRO: u8 = 0b0000_1000;
/// Make division by zero result in the maximum signed value (mode)
pub const DIV_ZRO_MAX: u8 = 0b0000_1100;

/// Forbid overflowing division by -1 (mode)
pub const DIV_OFW_FRB: u8 = 0;
/// Make overflowing division by -1 result in the minimum signed value (mode)
pub const DIV_OFW_MIN: u8 = 0b0000_0001;
/// Make overflowing division by -1 result in zero (mode)
pub const DIV_OFW_ZRO: u8 = 0b0000_0010;
/// Make overflowing division by -1 result in the maximum signed value (mode)
pub const DIV_OFW_MAX: u8 = 0b0000_0011;
