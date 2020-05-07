
pub trait Bus {
    fn name(&self) -> &'static str;
    fn size(&self) -> u32;
    fn read(&mut self, addr: u32) -> u32; // Guaranteed to be aligned (multiple of 4)
    fn write(&mut self, addr: u32, word: u32); // Guaranteed to be aligned (multiple of 4)
    fn reset(&mut self);
}
