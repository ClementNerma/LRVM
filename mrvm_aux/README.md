# `mrvm_aux` crate

The `mrvm_aux` crate contains a collection of common auxiliary components:

- [`display::BufferedDisplay`](src/display/buffered.rs) is a simple buffer-backed display that is triggered by writing an action code to its last writable address
- [`display::SyncKeyboard`](src/keyboard/sync.rs) is a simple buffer-backed synchronous keyboard that is triggered by writing an action code to its last writable address
- [`memory::VolatileMem`](src/memory/volatile.rs) is a RAM-like memory
- [`storage::BootROM`](src/storage/bootrom.rs) is a read-only persistent storage meant to contain a program's code and data
- [`storage::FlashMem`](src/storage/flash.rs) is a writable persistent memory
- [`storage::PersistentMem`](src/storage/persistent.rs) is a persistent memory that is read from and written to a file in order to keep the data alive after your program stops

You can find more informations on how to use the components in the [tutorial](../docs/Tutorial.md).

All details related to how components work can be found in the [hardware document](../docs/Hardware.md).
