use lrvm::board::Bus;
use lrvm_tools::exceptions::AuxHwException;
use lrvm_tools::metadata::{DeviceCategory, DeviceMetadata};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use thread::JoinHandle;

/// A 1-word-long component that contains a readable counter.  
/// The counter is incremented each second, asynchronously.
pub struct AsyncCounter {
    /// The program's unique hardware identifier
    hw_id: u64,

    /// The counter's value
    counter: Arc<AtomicU32>,

    /// Used to indicate to the counting thread to exit
    must_stop: Arc<AtomicBool>,

    /// Child thread incrementing the counter every second
    counting_thread: Option<JoinHandle<()>>,
}

impl AsyncCounter {
    pub fn new(hw_id: u64) -> Self {
        // Instanciate the component
        Self {
            hw_id,
            counter: Arc::default(),
            must_stop: Arc::default(),
            counting_thread: None,
        }
    }

    /// Stop the counting thread (if any is alive)
    pub fn stop(&mut self) {
        if let Some(handle) = self.counting_thread.take() {
            self.must_stop.store(true, Ordering::SeqCst);
            handle.join().unwrap();
        }
    }
}

impl Bus for AsyncCounter {
    // The component's name
    fn name(&self) -> &'static str {
        "Async Counter"
    }

    // The component's metadata, giving informations on what the component is
    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(self.hw_id, 4, DeviceCategory::Uncategorized, None, None).encode()
    }

    // Read an address inside the component
    // There is only one possible address here, so we don't have to worry about its value
    fn read(&mut self, _addr: u32, _ex: &mut u16) -> u32 {
        self.counter.load(Ordering::SeqCst)
    }

    // Write an address inside the component
    // This is not allowed inside our component, which is read-only
    fn write(&mut self, _addr: u32, _word: u32, ex: &mut u16) {
        *ex = AuxHwException::MemoryNotWritable.encode();
    }

    // Reset the component
    fn reset(&mut self) {
        // Stop the existing thread
        self.stop();

        // Create a shared counter
        self.counter = Arc::new(AtomicU32::new(0));

        // Create a "must stop" HALT signal
        self.must_stop = Arc::new(AtomicBool::new(false));

        // Clone its lock to use it from another thread
        let thread_counter = Arc::clone(&self.counter);

        // Clone it to use it from another thread
        let thread_must_stop = Arc::clone(&self.must_stop);

        // Create the thread which will increment the counter each second
        self.counting_thread = Some(thread::spawn(move || loop {
            // Forever, wait for 1 second...
            for _ in 1..100 {
                thread::sleep(Duration::from_millis(10));

                // ...while periodically listening to HALT signals...
                if thread_must_stop.load(Ordering::SeqCst) {
                    return;
                }
            }

            // ...then increment the counter
            thread_counter.fetch_add(1, Ordering::SeqCst);
        }));
    }
}

// Destroy the running thread (if any) when the component is destroyed (dropped)
impl Drop for AsyncCounter {
    fn drop(&mut self) {
        self.stop();
    }
}
