/// Virtual motherboard-related structures.
mod board;
mod bus;
mod hwb;

pub(crate) use self::hwb::*;
pub use self::{board::*, bus::*};
