use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use mrvm::board::Bus;
use mrvm_tools::metadata::{DeviceMetadata, DeviceCategory};
use mrvm_tools::exceptions::AuxHwException;

/// A 1-word-long component that contains a readable counter.  
/// The counter is incremented each second, asynchronously.
pub struct AsyncCounter {
    hw_id: u64,
    counter: Arc<RwLock<u32>>
}

impl AsyncCounter {
    pub fn new(hw_id: u64) -> Self {
        let counter = Arc::new(RwLock::new(0));
        let thread_counter = Arc::clone(&counter);

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(1000));
                *(thread_counter.write().unwrap()) += 1;
            }
        });

        Self { hw_id, counter: Arc::clone(&counter) }
    }
}

impl Bus for AsyncCounter {
    fn name(&self) -> &'static str {
        "Async Counter"
    }

    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(self.hw_id, 4, DeviceCategory::Uncategorized(), None, None).encode()
    }

    fn read(&mut self, _addr: u32, _ex: &mut u16) -> u32 {
        *(self.counter.read().unwrap())
    }

    fn write(&mut self, _addr: u32, _word: u32, ex: &mut u16) {
        *ex = AuxHwException::MemoryNotWritable.encode();
    }

    fn reset(&mut self) {
        *(self.counter.write().unwrap()) = 0;
    }
}