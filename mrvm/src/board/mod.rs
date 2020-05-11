/// Virtual motherboard-related structures.

mod board;
mod bus;
mod mem;
mod hwb;

pub use board::*;
pub use bus::*;
pub use mem::*;
pub(crate) use hwb::*;
