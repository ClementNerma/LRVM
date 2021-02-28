# `hw_detect` example

This crate is an advanced example of hardware detection. The main program located in [`src/main.rs`](src/main.rs) attaches a set of hardware components to be used by the program located in [`src/source.lasm`](src/source.lasm), which then populates the memory with encoded hardware informations decoded by the main program when the VM stops.

All available data are retrieved by the hardware detector, so this is a good starting point if you want to learn how to detect hardware in [LASM](../../docs/Architecture.md#assembly-language).