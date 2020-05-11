# `mrvm_tools` crate

The `mrvm_tools` crate contains a set of tools to deal with MRVM. It is split into several modules:

- [`asm`](src/asm/) is a set of types that allow to build a program in pure Rust and ensure its validity at build time, as well as to decode machine code on the fly
- [`bytes`](src/bytes/) is a set of tools to deal with byte suites, especially converting list of bytes to words and words to bytes
- [`debug`](src/debug/) is a set of tools to set up and run a VM following a provided configuration
- [`lasm`](src/lasm/) is a complete assembler which allows to assemble LASM source code on the fly
- [`metadata`](src/metadata/) is an interface for components to encode easily their metadata

For more informations on how to use this crate, please check the [tutorial](../docs/Tutorial.md).
