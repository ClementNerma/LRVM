# LRVM Examples

This directory contains a list of examples of usage for LRVM.

All programs include a boilerplate code to create the VM and run it.

| Example name                    | Description                                                     |
| ------------------------------- | --------------------------------------------------------------- |
| [`elapsed_time`](elapsed_time/) | A simple program to count elapsed time                          |
| [`hello_world`](hello_world/)   | A simple program displaying `Hello, world!`                     |
| [`minimal`](minimal/)           | The most simple VM                                              |
| [`async_hw`](async_hw/)         | Create an asynchronous clock component living in another thread |
| [`disassemble`](disassemble/)   | Assemble then disassemble a LASM program                        |
| [`benchmark`](benchmark/)       | Benchmark the speed of LRVM on your computer                    |

There are also a few advanced examples:

| Example name              | Description                                                                             |
| ------------------------- | --------------------------------------------------------------------------------------- |
| [`hw_detect`](hw_detect/) | Detect dynamically all components connected to the motherboard and mapped in the memory |
| [`brainfuck`](brainfuck/) | A complete BrainFuck interpreter with detailed errors output                            |
