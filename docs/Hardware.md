# Hardware management

- [Communications](#communications)
- [Bus interface](#bus-interface)
- [Metadata](#metadata)
- [Example program](#example-program)

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

Below is an advanced example of an asynchronous Rust component, acting as a realtime clock for counting seconds elapsed since the VM started.

This example is taken from the [`AsyncCounter`](../examples/async_hw/src/counter.rs) component from the [`async_hw`](../examples/async_hw/) example.

We first declare our component:

```rust
use std::sync::{Arc, RwLock};

pub struct AsyncCounter {
    hw_id: u64,
    counter: Arc<RwLock<u32>>,
}
```

The `hw_id` part is the unique hardware identifier. Every component has its own, and it is used to differentiate one component from another, using the [LASM hardware instructions](Architecture.md#hardware-access-instructions). Its only requirement is to be an 8-byte unsigned integer, as well as being unique among all other components (otherwise the VM won't crash but this could cause problems in programs relying on IDs for identification).

Conventionally, the ID generation is left to the user, so we'll make this counter instanciable with an already-set ID:

```rust
impl AsyncCounter {
    pub fn new(hw_id: u64) -> Self {
        Self { hw_id, counter: todo!() }
    }
}
```

Now we have to make our asynchronous counter. For that, we're going to create a local counter, stored in our structure, which will then be incremented each second by a thread:

```rust
// ...
use std::thread;
use std::time::Duration;

// ...
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
```

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
            category: DeviceCategory::Uncategorized(),
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
        DeviceMetadata::new(self.hw_id, 4, DeviceCategory::Uncategorized(), None, None).encode()
    }
    // ...
```

Perfect!

Next, we have the `read` method, which is called when the VM's program tried to read a value from the component's mapped memory. This is how RAM and BootROM components work: values are retrieved from specific addresses of them, to be stored in a register and be manipulated later on.

As our component is a single-word one (4 bytes), we don't have to care about the address, which the memory ensures to be between the bounds, meaning it will always be `0` as addresses must always be aligned (to words, so be multiple of 4 bytes). So let's just return the counter's value:

```rust
    // ...
    fn read(&mut self, _addr: u32, _ex: &mut u16) -> u32 {
        *(self.counter.read().unwrap())
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
        *(self.counter.write().unwrap()) = 0;
    }
    // ...
```

Ideally, we should even reset the thread itself, as when we'll reset a new second, the thread will increment less than one second later. But that would involve messages passing, which is beyong this tutorial, so we'll simply stick with:

```rust
    // ...
    fn reset(&mut self) {
        *(self.counter.write().unwrap()) = 0;
    }
    // ...
```

We did it! Here is our component's complete code (with unused arguments prefixed with `_` for the linter):

```rust
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
```
