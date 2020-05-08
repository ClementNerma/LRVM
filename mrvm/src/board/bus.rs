
/// Bus of an auxiliary component.
/// All components must implement this type in order to be connected to the motherboard.
pub trait Bus {
    /// Get the component's generic name.
    /// **Must not exceed 32 bytes** (not 32 characters).
    fn name(&self) -> &'static str;

    /// Get the component's size. This will be used to determine the range of address to map the component on.
    fn size(&self) -> u32;

    /// Answer a READ request from the bus.
    /// The provided address is guaranteed to be aligned (multiple of 4) and strictly lower than the provided size.
    /// May raise an exception by assigning a non-zero exception code and data to the provided reference.
    fn read(&mut self, addr: u32, ex: &mut u16) -> u32;

    /// Answer a WRITE request from the bus.
    /// The provided address is guaranteed to be aligned (multiple of 4) and strictly lower than the provided size.
    /// May raise ane xception by assigning a non-zero exception code and data to the provided reference.
    fn write(&mut self, addr: u32, word: u32, ex: &mut u16) -> ();

    /// Handle a RESET signal sent by the motherboard.
    /// All volatile data from the component must be reset.
    fn reset(&mut self);
}
