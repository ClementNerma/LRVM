# Hardware management

- [Communications](#communications)
- [Bus interface](#bus-interface)
- [Metadata](#metadata)
- [Example program](#example-program)
  - [1. Basics](#1-basics)
  - [2. Bus implementation](#2-bus-implementation)
  - [3. Proper thread handling](#3-proper-thread-handling)

Virtual hardware (named _auxiliary components_) can be connected to the motherboard, communicating through a _bus_.

## Communications

Components cannot communicate with the motherboard on their own ; only the motherboard can contact them. A synchronous answer must then be provided in order for the virtual machine to continue.

## Bus interface

A component is a structure instance implementing the [`Bus`](../lrvm/src/board/bus.rs) trait.

The motherboard may send _requests_ to components, which must be answered synchronously (CPU is paused meanwhile). The list of requests are:

| Request name | Parameters             | Answer type    | Description                                                                  |
| ------------ | ---------------------- | -------------- | ---------------------------------------------------------------------------- |
| `NAME`       | N/A                    | `&'static str` | Get the component's generic name (UTF-8 encoded, up to 32 bytes, cut beyond) |
| `METADATA`   | N/A                    | `[u32; 8]`     | Get the component's [metadata](#metadata)                                    |
| `READ`       | `addr: u32`            | `u32`          | Read an address from the component                                           |
| `WRITE`      | `addr: u32, data: u32` | `()`           | Write an address in the component                                            |
| `RESET`      | N/A                    | `()`           | Reset the component                                                          |

The `READ` and `WRITE` requests also receive an `u16` mutable reference that may be used to raise an exception. When the method returns, if the value in the reference is not zero, the CPU will consider an [hardware exception](Architecture.md#exceptions) occurred.

The strongest bits contain the exception code, and the weakest bits the associated data which depends on the type of exception.

Exceptions cannot be raised when receiving `NAME`, `METADATA` or `RESET` requests, as these are expected to never fail.

## Metadata

The motherboard retrieves the component's metadata during mapping to invalidate incorrect mappings. The CPU can also ask the motherboard to send `METADATA` requests in order to retrieve specific informations about the device's type for instance.

The component's metadata is represented as a suite of 8 words, or 32 bytes. It contains:

- Words 0-1 (bytes 00-07): Unique hardware identifier
- Words 2-2 (bytes 08-11): Device's size
- Words 3-3 (bytes 12-15): Device's category
- Words 4-4 (bytes 16-19): Device's type
- Words 5-5 (bytes 20-23): Device's model
- Words 6-7 (bytes 24-31): Optional custom additional data

Note that the size should NEVER change after the component's creation. The size is used to map correctly the components in the memory, and to ensure any `READ` and `WRITE` requests are in range, not exceeding the component's size. If the size changes after creation, it may receive invalid `READ`/`WRITE` requests.

## Example program

### 1. Basics

Below is an advanced example of an asynchronous Rust component, acting as a realtime clock for counting seconds elapsed since the VM started.

This example is taken from the [`AsyncCounter`](../examples/async_hw/src/counter.rs) component from the [`async_hw`](../examples/async_hw/) example.

We first declare our component:

```rust
use std::sync::Arc;
use std::sync::atomic::AtomicU32;

pub struct AsyncCounter {
    hw_id: u64,
    counter: Arc<AtomicU32>,
}
```

The `hw_id` part is the unique hardware identifier. Every component has its own, and it is used to differentiate one component from another, using the [LASM hardware instructions](Architecture.md#hardware-access-instructions). Its only requirement is to be an 8-byte unsigned integer, as well as being unique among all other components (otherwise the VM won't crash but this could cause problems in programs relying on IDs for identification).

Conventionally, the ID generation is left to the user, so we'll make this counter instanciable with an already-set ID:

```rust
impl AsyncCounter {
    pub fn new(hw_id: u64) -> Self {
        Self { hw_id, counter: Arc::default() }
    }
}
```

Now we have to make our asynchronous counter. For that, we're going to create a local counter, stored in our structure, which will then be incremented each second by a thread:

```rust
// ...
use std::thread;
use std::time::Duration;
use std::sync::atomic::Ordering;

// ...
    pub fn new(hw_id: u64) -> Self {
        // Create a shared counter
        let counter = Arc::new(AtomicU32::new(0));

        // Clone its lock to use it from another thread
        let thread_counter = Arc::clone(&counter);

        // Create the thread which will increment the counter each second
        thread::spawn(move || loop {
            // Forever, wait for 1 second...
            thread::sleep(Duration::from_millis(1000));
            // ...and then increment the counter
            thread_counter.fetch_add(1, Ordering::SeqCst);
        });

        // Return the component
        Self { hw_id, counter }
    }
```

### 2. Bus implementation

Well done! Now, we have to make this component pluggable into our VM. For that, we have to implement the `lrvm::board::Bus` trait on it:

```rust
// ...
use lrvm::board::Bus;

// ...

impl Bus for AsyncCounter {
    // The component's name
    fn name(&self) -> &'static str {
        todo!()
    }

    // The component's metadata, giving informations on what the component is
    fn metadata(&self) -> [u32; 8] {
        todo!()
    }

    // Read an address inside the component
    // There is only one possible address here, so we don't have to worry about its value
    fn read(&mut self, addr: u32, ex: &mut u16) -> u32 {
        todo!()
    }

    // Write an address inside the component
    // This is not allowed inside our component, which is read-only
    fn write(&mut self, addr: u32, word: u32, ex: &mut u16) {
        todo!()
    }

    // Reset the component
    fn reset(&mut self) {
        todo!()
    }
}
```

Great! Now, let's fill these methods.

First, `name` indicates the name of our component. It is the same across all instances, so we need to choose it carefully. Also, it can't exceed 32 bytes (so 32 ASCII extended characters, or as few as 8 characters in largest UTF-8 symbols). Beyond this limit, the characters will simply be cut off until the string is not more than 32 bytes long.

```rust
    // ...
    fn name(&self) -> &'static str {
        "Async Counter"
    }
    // ...
```

Next, we have the `metadata` method, which gives informations about our component: its unique hardware identifier, its size in bytes which which indicates how much mapped memory it'll take at most and what the maximum reading and writing address is, the category, the model, and an additional data field.

As we're making a custom component, it doesn't have a specific category or model, but these should be set whenever possible to simplify and improve hardware management in programs.

Now, instead of making the 32-bytes long array by hand, which would be quite complicated, we'll instead use a tool from the `lrvm_tools` crate to help us:

```rust
use lrvm_tools::metadata::{DeviceCategory, DeviceMetadata};

    // ...
    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata {
            hw_id: self.hw_id,
            size: 4,
            category: DeviceCategory::Uncategorized,
            model: None,
            data: None,
        }
        .encode()
    }
    // ...
```

Great. But, our method is a little verbose, so let's simplify it:

```rust
    // ...
    fn metadata(&self) -> [u32; 8] {
        DeviceMetadata::new(self.hw_id, 4, DeviceCategory::Uncategorized, None, None).encode()
    }
    // ...
```

Perfect!

Next, we have the `read` method, which is called when the VM's program tried to read a value from the component's mapped memory. This is how RAM and BootROM components work: values are retrieved from specific addresses of them, to be stored in a register and be manipulated later on.

As our component is a single-word one (4 bytes), we don't have to care about the address, which the memory ensures to be between the bounds, meaning it will always be `0` as addresses must always be aligned (to words, so be multiple of 4 bytes). So let's just return the counter's value:

```rust
    // ...
    fn read(&mut self, _addr: u32, _ex: &mut u16) -> u32 {
        self.counter.load(Ordering::SeqCst)
    }
    // ...
```

Then we have its counterpart, `write`. This function is called when the VM tries to write a value inside the component's mapped memory, which is how components like RAM work: the program writes a value inside a RAM's address, and can retrieve it later on. But, for our component, this doesn't make sense to accept writings, as it only exposes a read-only counter. So, to indicate this is not allowed, as components cannot return values (just like real hardware components), we can instead trigger a _CPU exception_, which is a message sent to the processor to indicate something went wrong.

For that, we have two options: either we encode the values ourselves, which is the 'default' way to do this but clearly not readable nor maintanable. Or, we can use another one of the tools provided by the `lrvm_tools` crates, which exposes sets of values that can be incoded automatically instead. Here is what it looks like with the latter:

```rust
    // ...
    fn write(&mut self, addr: u32, word: u32, ex: &mut u16) {
        *ex = AuxHwException::MemoryNotWritable.encode();
    }
    // ...
```

Which is more readable than the raw equivalent `*ex = 31;` code.

Finally, the last method! `reset` is called either when the motherboard is reset, or when the [`RESET` instruction](Architecture.md#processor-control-instructions) is used on this component.

Its goal is to reset the component's state like it was when it was initially created, which in our case simply consists in setting the counter to `0`:

```rust
    // ...
    fn reset(&mut self) {
        self.counter.store(0, Ordering::SeqCst);
    }
    // ...
```

Ideally, we should even reset the thread itself, as when we'll reset a new second, the thread will increment less than one second later. But that would involve messages passing, which is a bit more complicated so we'll see it in the next section. Also, components should never start when instanciated, only when they receive their first RESET signal (when `reset` is called for the first time).

We did it! Here is our component's complete code (with unused arguments prefixed with `_` for the linter):

```rust
use lrvm::board::Bus;
use lrvm_tools::exceptions::AuxHwException;
use lrvm_tools::metadata::{DeviceCategory, DeviceMetadata};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

/// A 1-word-long component that contains a readable counter.  
/// The counter is incremented each second, asynchronously.
pub struct AsyncCounter {
    hw_id: u64,
    counter: Arc<AtomicU32>,
}

impl AsyncCounter {
    pub fn new(hw_id: u64) -> Self {
        // Create a shared counter
        let counter = Arc::new(AtomicU32::new(0));

        // Clone its lock to use it from another thread
        let thread_counter = Arc::clone(&counter);

        // Create the thread which will increment the counter each second
        thread::spawn(move || loop {
            // Forever, wait for 1 second...
            thread::sleep(Duration::from_millis(1000));
            // ...and then increment the counter
            thread_counter.fetch_add(1, Ordering::SeqCst);
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
        self.counter.store(0, Ordering::SeqCst);
    }
}
```

### 3. Proper thread handling

Our component is now ready to use, but we have several problems:

- The component starts as soon as it is instanciated, which is not conventional ;
- When the component is reset, the counter will be incremented less than one second later as the thread is still running ;
- Destroying the component (drop) doesn't stop the thread which results in a defunct thread

Let's solve these issues by implementing a very simple message-passing system to indicate the thread to stop when required, and also making it start only in `reset`.

First, let's update our structure to fit all those informations:

```rust
// ...
use std::thread::JoinHandle;
// ...

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
```

Now let's rewrite its instanciation:

```rust
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
}
```

Finally, let's dig into the main part: thread handling.

First, let's make a `stop` method in the implementation, whose role will be, if the child thread is running, to notify it to stop and then wait for it to exit properly.

```rust
    // ...

    /// Stop the counting thread (if any is alive)
    fn stop(&mut self) {
        if let Some(handle) = self.counting_thread.take() {
            self.must_stop.store(true, Ordering::SeqCst);
            handle.join().unwrap();
        }
    }

    // ...
```

And now the `reset` method, which will first halt the component (in case it hasn't been halted yet, to avoid having multiple threads running in parallel), and then spawn a new counting thread:

```rust
    // ...
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
    // ...
```

It is important to keep the checking interval as low as possible to not make the program struggle when all components are destroyed at once.

And we can then add a `Drop` implementation to close the thread:

```rust
// Destroy the running thread (if any) when the component is destroyed (dropped)
impl Drop for AsyncCounter {
    fn drop(&mut self) {
        self.stop();
    }
}
```

Now our component is finally complete, and fully functionnal! Here is the final code:

```rust
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
```