/// Virtual motherboard-related structures.
mod board;
mod bus;
mod hw_bridge;

pub(crate) use self::hw_bridge::*;
pub use self::{board::*, bus::*};
