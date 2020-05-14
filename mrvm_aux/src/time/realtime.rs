//! The real time clock component provides a way to get informations about the current time.  
//! See [`RealtimeClock`] for more details.

use std::time::{SystemTime, Instant, UNIX_EPOCH};
use mrvm::board::Bus;
use mrvm_tools::metadata::{DeviceMetadata, ClockType};
use mrvm_tools::exceptions::AuxHwException;

/// The realtime clock component is a 6-word-long readonly component.  
///
/// The first and second word contains the number of seconds that passed since UNIX_EPOCH, encoded on 64 bits.  
/// This means the second word can handle all durations up to 2120 (roughly).  
/// The third word contains the subsequent number of milliseconds, microseconds and nanoseconds.  
///
/// The fourth to sixth words use the same format, but with the time elapsed since the component was last reset.
pub struct RealtimeClock {
    hw_id: u64,
    reset_at: Instant
}

impl RealtimeClock {
    pub fn new(hw_id: u64) -> Self {
        Self { hw_id, reset_at: Instant::now() }
    }
}

impl Bus for RealtimeClock {
    fn name(&self) -> &'static str {
        "Realtime Clock"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(
            self.hw_id,
            24,
            ClockType::Realtime.wrap(),
            None,
            None
        ).encode()
    }

    fn read(&mut self, addr: u32, ex: &mut u16) -> u32 {
        let time = if addr < 14 {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration ) => duration,
                Err(_) => {
                    *ex = AuxHwException::TimeSynchronizationError.encode();
                    return 0;
                }
            }
        } else {
            self.reset_at.elapsed()
        };

        match addr % 3 {
            0x00 => (time.as_secs() >> 32) as u32,
            0x01 => (time.as_secs() & 0xFFFFFFFF) as u32,
            0x02 => (time.subsec_millis() * 1_000_000) + (time.subsec_micros() * 1000) + time.subsec_nanos(),
            _ => unreachable!()
        }
    }

    fn write(&mut self, _addr: u32, _word: u32, ex: &mut u16) {
        *ex = AuxHwException::MemoryNotWritable.encode();
    }

    fn reset(&mut self) {
        self.reset_at = Instant::now();
    }
}