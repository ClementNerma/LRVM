# `lrvm_aux` crate

The `lrvm_aux` crate contains a collection of common auxiliary components.

You can find more informations on how to use the components in the [tutorial](../docs/Tutorial.md).

All details related to how components work can be found in the [hardware document](../docs/Hardware.md).

## List of components

### Debug interfaces

| Component name                            | Description           |
| ----------------------------------------- | --------------------- |
| [`debug::BasicDebug`](src/debug/basic.rs) | Basic debug interface |

### Volatile memory

| Component name                                 | Description     |
| ---------------------------------------------- | --------------- |
| [`volatile_mem::RAM`](src/volatile_mem/ram.rs) | RAM-like memory |

### Storage

| Component name                                        | Description                                                             |
| ----------------------------------------------------- | ----------------------------------------------------------------------- |
| [`storage::BootROM`](src/storage/bootrom.rs)          | Read-only persistent storage meant to contain a program's code and data |
| [`storage::FlashMem`](src/storage/flash.rs)           | Writable persistent memory                                              |
| [`storage::PersistentMem`](src/storage/persistent.rs) | Persistent memory flushed to a real file                                |

### Display

| Component name                                        | Description                   |
| ----------------------------------------------------- | ----------------------------- |
| [`display::CharDisplay`](src/display/character.rs)    | Display for single characters |
| [`display::BufferedDisplay`](src/display/buffered.rs) | Display for strings           |

### Keyboard

| Component name                                            | Description                                  |
| --------------------------------------------------------- | -------------------------------------------- |
| [`keyboard::SyncCharKeyboard`](src/keyboard/sync_char.rs) | Simple character-backed synchronous keyboard |
| [`keyboard::SyncLineKeyboard`](src/keyboard/sync_line.rs) | Simple buffer-backed synchronous             |
