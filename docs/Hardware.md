# Hardware management

Virtual hardware (named _auxiliary components_) can be connected to the motherboard, communicating through a _bus_.

## Communications

Components cannot communicate with the motherboard on their own ; only the motherboard can contact them. A synchronous answer must then be provided in order for the virtual machine to continue.

## Bus interface

A component is a structure instance implementing the [`Bus`](../mrvm/src/boards/bus.rs) trait.

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
