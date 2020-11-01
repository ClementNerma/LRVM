/// Virtual motherboard-related structures.
mod board;
mod bus;
mod hwb;

pub use board::*;
pub use bus::*;
pub(crate) use hwb::*;
