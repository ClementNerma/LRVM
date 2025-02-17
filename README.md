# Lightweight Rust Virtual Machine

The LRVM project consists in a simple but complete 32-bit virtual machine.

The project's goal is to make a virtual machine which is powerful enough to play with, but simple enough to use it as a learning material on how VM work and making simple proof-of-concept (POC) programs.

The main features are:

- Motherboard emulation, allowing to plug-in virtual components ;
- MMIO emulation for mapping components into memory ;
- MMU emulation for translating virtual addresses into physical ones and managing userland permissions ;
- Supervisor/userland mode ;
- A very simple and friendly API ;
- Debugging tools such as a full-powered assembler and disassembler

It is split into several crates:

- [`lrvm`](lrvm/): LRVM's core, which contains the motherboard, MMIO, CPU and MMU emulation
- [`lrvm_aux`](lrvm_aux/): A set of useful auxiliary components
- [`lrvm_tools`](lrvm_tools/): A set of tools to deal more easily with LRVM, including a strongly-typed assembler and a string-based one

## Examples

Various examples can be found in the [`examples/`](examples/README.md) directory.

## Additional components

An arbitrary number of components can be connected to the virtual motherboard and accessed through memory mapping (MMIO).
Many components are available in the [`lrvm_aux`](lrvm_aux/) crate.

## Performances

LRVM is designed to be easy to learn and use, which means performance is not a primary goal. The CPU works as a single-core interpreter ; there is no multi-threading nor just-in-time compilation (JIT) as this would complexify this project a lot, which would be contrary to its main goal.

You can run a little benchmark to see how many instructions per second your computer can run with LRVM. The benchmark is essentially made of arithmetic instructions, which are the slowest ones as they require to compute the resulting arithmetic flags.

To run the benchmark:

```shell
cd examples/benchmark
cargo run --release
```

For reference, on an **Intel Core i5-1240P**, we get a result of ~ 120 MIPS (Million Instructions Per Second) with [LTO enabled](https://doc.rust-lang.org/cargo/reference/profiles.html#lto). On a **Ryzen 7940HX**, we get ~ 135 MIPS. As you can see, it scales with single-core performance, not multi-core.

## Documentation

The documentation is made of several parts :

- [A step-by-step tutorial](docs/Tutorial.md) explaining how to set up a VM and running it with debugging tools
- [An architecture document](docs/Architecture.md) describing how the VM works, the structure of registers, memory, etc.
- [An hardware specifications document](docs/Hardware.md) describing how auxiliary components work

## Specifications

The virtual machine's specifications (registers, computation, ISA, etc.) are available [here](docs/).

## Assembly language

LRVM uses a lightweight language assembly called LASM (Lightweight Assembly). LASM follows the [assembly language specifications](docs/Architecture.md#assembly-language) and is assembled using [CustomASM](https://github.com/hlorenzi/customasm) (a huge thanks to their author for making this great library).

Here is an example of a simple LASM program:

```lasm
main:
    add a0, 1

    cmp a0, 10
    ifeq
    halt

    jp main
```

This program adds `1` to the `a0` register until it reaches `10`, then it halts the processor.

To see more about the LASM language, see [the tutorial](docs/Tutorial.md).

### Syntax highlighting

You can install the [Visual Studio Code extension](https://marketplace.visualstudio.com/items?itemName=clement-nerma.lrvm-lasm) to get syntax highlighting for LRVM's [Lightweight Assembly (LASM) language](docs/Architecture.md#assembly-language).

Or you can build the sources from the [`vscode-lasm`](vscode-lasm/) directory:

```shell
cd vscode-lang
vsce package
code --install-extension lrvm-lasm-*.vsix
# Enjoy!
```

## Testing

As some examples depends on other, and Cargo does not provide a way to prevent tests from running concurrently, you should not use a simple `cargo test` on LRVM's crates but instead:

```shell
cargo test -- --test-threads=1
```
