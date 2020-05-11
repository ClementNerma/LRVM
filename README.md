# Minimal Rust Virtual Machine

The MRVM project consists in a simple but complete 32-bit virtual machine.

The main features are:

- Motherboard emulation, allowing to plug-in virtual components ;
- MMIO emulation for mapping components into memory ;
- MMU emulation for translation virtual addresses into physical ones and managing userland permissions ;
- Supervisor/userland mode ;
- A very simple and friendly API

It is split into several crates:

- [`mrvm`](mrvm/): MRVM's core, which contains the motherboard, MMIO, CPU and MMU emulation
- [`mrvm_aux`](mrvm_aux/): A set of useful auxiliary components
- [`mrvm_tools`](mrvm_tools/): A set of tools to deal more easily with MRVM, including a strongly-typed assembler and a string-based one

## Additional components

An arbitrary number of components can be connected to the virtual motherboard and accessed through memory mapping (MMIO).
Many components are available in the [`mrvm_aux`](mrvm_aux/) crate.

## Documentation

The documentation is made of several parts :

- [A step-by-step tutorial](docs/Tutorial.md) explaining how to set up a VM and running it with debugging tools
- [An architecture document](docs/Architecture.md) describing how the VM works, the structure of registers, memory, etc.
- [An hardware specifications document](docs/Hardware.md) describing how auxiliary components work

## Specifications

The virtual machine's specifications (registers, computation, ISA, etc.) are available [here](docs/).

## Assembly language

MRVM uses a lightweight language assembly called LASM (Lightweight Assembly). LASM follows the [assembly language specifications](docs/Architecture.md#assembly-language) and is assembled using [CustomASM](https://github.com/hlorenzi/customasm) (a huge thanks to their author for making this great library).

Here is an example of a simple LASM program:

```asm
main:
    add a0, 1
    cmp a0, 10
    if zf
    halt
    jp -16
```

This program adds `1` to the `a0` register until it reaches `10`, then it halts the processor.

## Examples

Examples can be found in the [`examples/`](examples/) directory.

## Testing

As some examples depends on other, and Cargo does not provide a way to prevent tests from running concurrently, you should not use a simple `cargo test` on MRVM's crates but instead:

```shell
cargo test -- --test-threads=1
```

And if you want the output of each test:

```shell
cargo test -- --test-threads=1 --nocapture
```
