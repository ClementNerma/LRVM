use lrvm::board::Bus;
use lrvm_tools::exceptions::AuxHwException;
use lrvm_tools::metadata::{DeviceCategory, DeviceMetadata};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

/// A 1-word-long component that contains a readable counter.  
/// The counter is incremented each second, asynchronously.
pub struct AsyncCounter {
    hw_id: u64,
    counter: Arc<RwLock<u32>>,
}

impl AsyncCounter {
    pub fn new(hw_id: u64) -> Self {
        // Create a shared counter
        let counter = Arc::new(RwLock::new(0));

        // Clone its lock to use it from another thread
        let thread_counter = Arc::clone(&counter);

        // Create the thread which will increment the counter each second
        thread::spawn(move || loop {
            // Forever, wait for 1 second...
            thread::sleep(Duration::from_millis(1000));
            // ...and then increment the counter
            *(thread_counter.write().unwrap()) += 1;
        });

        // Return the component
        Self { hw_id, counter }
    }
}

impl Bus for AsyncCounter {
    // The component's name
    fn name(&self) -> &'static str {
        "Async Counter"
    }

    // The component's metadata, giving informations on what the component is
    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(self.hw_id, 4, DeviceCategory::Uncategorized(), None, None).encode()
    }

    // Read an address inside the component
    // There is only one possible address here, so we don't have to worry about its value
    fn read(&mut self, _addr: u32, _ex: &mut u16) -> u32 {
        *(self.counter.read().unwrap())
    }

    // Write an address inside the component
    // This is not allowed inside our component, which is read-only
    fn write(&mut self, _addr: u32, _word: u32, ex: &mut u16) {
        *ex = AuxHwException::MemoryNotWritable.encode();
    }

    // Reset the component
    fn reset(&mut self) {
        *(self.counter.write().unwrap()) = 0;
    }
}
