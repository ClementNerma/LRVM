# `lrvm` crate

The `lrvm` crate contains the core of LRVM: the motherboard and components emulation, as well as the CPU and the MMU.

It is split across several main types:

- [`board::MotherBoard`](src/board/board.rs) is the virtual motherboard type
- [`mem::Mem`](src/mem/mem.rs) is the virtual motherboard's mapped memory
- [`board::Bus`](src/board/bus.rs) is a trait components must implement in order to be connected to the motherboard
- [`cpu::CPU`](src/cpu/cpu.rs) is the virtual CPU
- [`cpu::Regs`](src/cpu/regs.rs) represent the registers of the virtual CPU
- [`mmu::MMU`](src/mmu/mmu.rs) is the virtual MMU (Memory Management Unit)

If you want to make use of this crate, you can take up the [tutorial](../docs/Tutorial.md) to see how to build a virtual machine, assemble a program and run it with debugging tools.

You can find a collection of useful auxiliary components in the [`lrvm_aux`](../lrvm_aux/) crate.

There is also a set of tools to deal with the VM (including a complete assembler and a set of debugging tools) in the [`lrvm_tools`](../lrvm_tools/) crate.

You can find more details about LRVM's architecture in the [architecture document](../docs/Architecture.md).
