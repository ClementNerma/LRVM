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

/// Unsigned division (mode)
pub const DIV_USG: u8 = 0x10;
/// Signed division (mode)
pub const DIV_SIG: u8 = 0x10;
/// Forbid division by zero (mode)
pub const DIV_ZRO_FRB: u8 = 0x00;
/// Make division by zero result in the minimum signed value (mode)
pub const DIV_ZRO_MIN: u8 = 0x04;
/// Make division by zero result in zero (mode)
pub const DIV_ZRO_ZRO: u8 = 0x08;
/// Make division by zero result in the maximum signed value (mode)
pub const DIV_ZRO_MAX: u8 = 0x0C;
/// Forbid overflowing division by -1 (mode)
pub const DIV_MBO_FRB: u8 = 0x00;
/// Make overflowing division by -1 result in the minimum signed value (mode)
pub const DIV_MBO_MIN: u8 = 0x01;
/// Make overflowing division by -1 result in zero (mode)
pub const DIV_MBO_ZRO: u8 = 0x02;
/// Make overflowing division by -1 result in the maximum signed value (mode)
pub const DIV_MBO_MAX: u8 = 0x03;
