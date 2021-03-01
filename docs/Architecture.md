# Rust VM

- [Central Processing Unit](#central-processing-unit)
  - [Processor](#processor)
  - [Registers](#registers)
  - [Arithmetic flags](#arithmetic-flags)
  - [Exceptions](#exceptions)
- [Startup](#startup)
- [Memory Management Unit](#memory-management-unit)
  - [Physical address pages](#physical-address-pages)
- [Memory-Mapped Input/Output](#memory-mapped-inputoutput)
- [The stack](#the-stack)
- [Processor instructions](#processor-instructions)
  - [Instructions format](#instructions-format)
  - [Assembly language](#assembly-language)
    - [Constants](#constants)
      - [Flags](#flags)
    - [Division modes](#division-modes)
    - [`IF2` conditions](#if2-conditions)
    - [`HWD` hardware information codes](#hwd-hardware-information-codes)
    - [Notation](#notation)
    - [Reading hardware informations](#reading-hardware-informations)
    - [Assignment instructions](#assignment-instructions)
    - [Arithmetic instructions](#arithmetic-instructions)
    - [Bitwise instructions](#bitwise-instructions)
    - [Logical instructions](#logical-instructions)
    - [Control flow instructions](#control-flow-instructions)
    - [Conditional instructions](#conditional-instructions)
    - [Memory read/write instructions](#memory-readwrite-instructions)
    - [Hardware access instructions](#hardware-access-instructions)
    - [Processor control instructions](#processor-control-instructions)
    - [Alias instructions](#alias-instructions)

RustVM is a single-threaded, lightweight virtual machine created for challenge purposes as well as to study how hardware behaves.

## Central Processing Unit

The CPU uses a 32-bit architecture, with a custom set of fixed-length instructions (32 bit each).
It contains a single monothread processor, a MMU, and a memory relying on MMIO.

### Processor

The processor supports exceptions/interruptions, stack management, and supervisor/userland mode.
It can perform integer-based as well as logical computations, but not floating-point ones.
It has direct access to the memory but relies on the MMU to translate virtual addresses into physical ones.

The processor uses Big Endian (BE) representation for numbers.

### Registers

The processor contains the 32 following registers, all with a capacity of 32 bits:

| Name                       | Symbol | Code   | Supervisor   | Userland     | Purpose                                                       |
| -------------------------- | ------ | ------ | ------------ | ------------ | ------------------------------------------------------------- |
| Arithmetic 0               | `a0`   | `0x00` | Read + Write | Read + Write | (Conventionally) Arithmetic computation                       |
| Arithmetic 1               | `a1`   | `0x01` | Read + Write | Read + Write | (Conventionally) Arithmetic computation                       |
| Arithmetic 2               | `a2`   | `0x02` | Read + Write | Read + Write | (Conventionally) Arithmetic computation                       |
| Arithmetic 3               | `a3`   | `0x03` | Read + Write | Read + Write | (Conventionally) Arithmetic computation                       |
| Arithmetic 4               | `a4`   | `0x04` | Read + Write | Read + Write | (Conventionally) Arithmetic computation                       |
| Arithmetic 5               | `a5`   | `0x05` | Read + Write | Read + Write | (Conventionally) Arithmetic computation                       |
| Arithmetic 6               | `a6`   | `0x06` | Read + Write | Read + Write | (Conventionally) Arithmetic computation                       |
| Arithmetic 7               | `a7`   | `0x07` | Read + Write | Read + Write | (Conventionally) Arithmetic computation                       |
| Comparison 0               | `c0`   | `0x08` | Read + Write | Read + Write | (Conventionally) Comparisons                                  |
| Comparison 1               | `c1`   | `0x09` | Read + Write | Read + Write | (Conventionally) Comparisons                                  |
| Address Computation 0      | `ac0`  | `0x0A` | Read + Write | Read + Write | (Conventionally) Address computation                          |
| Address Computation 1      | `ac1`  | `0x0B` | Read + Write | Read + Write | (Conventionally) Address computation                          |
| Address Computation 2      | `ac2`  | `0x0C` | Read + Write | Read + Write | (Conventionally) Address computation                          |
| Routine Register 0         | `rr0`  | `0x0D` | Read + Write | Read + Write | (Conventionally) Routine-scoped computation                   |
| Routine Register 1         | `rr1`  | `0x0E` | Read + Write | Read + Write | (Conventionally) Routine-scoped computation                   |
| Routine Register 2         | `rr2`  | `0x0F` | Read + Write | Read + Write | (Conventionally) Routine-scoped computation                   |
| Routine Register 3         | `rr3`  | `0x10` | Read + Write | Read + Write | (Conventionally) Routine-scoped computation                   |
| Routine Register 4         | `rr4`  | `0x11` | Read + Write | Read + Write | (Conventionally) Routine-scoped computation                   |
| Routine Register 5         | `rr5`  | `0x12` | Read + Write | Read + Write | (Conventionally) Routine-scoped computation                   |
| Routine Register 6         | `rr6`  | `0x13` | Read + Write | Read + Write | (Conventionally) Routine-scoped computation                   |
| Routine Register 7         | `rr7`  | `0x14` | Read + Write | Read + Write | (Conventionally) Routine-scoped computation                   |
| Atomic Value Register      | `avr`  | `0x15` | Read + Write | Read + Write | (Conventionally) Computation that can be overwritten anywhen  |
| Program Counter            | `pc`   | `0x16` | Read + Write | Read + Write | Know the memory address of the next instruction               |
| Arithmetic Flags           | `af`   | `0x17` | Read         | Read         | Know infos on the result of the previous arithmetic operation |
| Supervisor Stack Pointer   | `ssp`  | `0x18` | Read + Write |              | Know the address of the supervisor's stack's last item        |
| Userland Stack Pointer     | `usp`  | `0x19` | Read + Write |              | Know the address of the userland's stack's last item          |
| Exception Type             | `et`   | `0x1A` | Read         |              | Know the last exception's type and in which mode it occurred  |
| Exception Return Address   | `era`  | `0x1B` | Read         |              | Know the address that raised an exception                     |
| Exception Vector           | `ev`   | `0x1C` | Read + Write |              | Know the address of the instruction to jump at on exception   |
| Memory Translation Toggler | `mtt`  | `0x1D` | Read + Write |              | Know if the MMU is enabled (`0` if not, any other value else) |
| Page Directory Address     | `pda`  | `0x1E` | Read + Write |              | Know the address of the Page Directory for the MMU            |
| Supervisor Mode Toggler    | `smt`  | `0x1F` | Read + Write |              | Know if the supervisor mode is enabled (`0` if not)           |

Conventionally, the `avr` register is used for very short-living operations, meaning it can be overwritten anywhen and may not be restored when recovering from an exception.

Also, the `rr0` to `rr7` registers are conventionally reserved to be used by routines, which means they may be rewritten when calling a routine.

### Arithmetic flags

The `af` register is set after each arithmetic instruction. Each of its bits are _flags_.
They give the following indications on the operation that happened (starting from the strongest bit):

| No. | Flag name       | Symbol | Description                                                                                    |
| --- | --------------- | ------ | ---------------------------------------------------------------------------------------------- |
| 0   | Zero Flag       | ZF     | Result is equal to `0`                                                                         |
| 1   | Carry Flag      | CF     | Result would be too large to fit in 32 bits                                                    |
| 2   | Overflow Flag   | OF     | Result would be too large to fit in 32 bits using two's complement representation              |
| 3   | Sign Flag       | SF     | Result's first bit is `1` (so it would be negative in two's complement representation)         |
| 4   | Even Flag       | EF     | Result's last bit is `0` (so it is even in both unsigned and two's complement representations) |
| 5   | Zero-Upper Flag | ZUF    | Result is smaller than `2^16` (so its upper bits are zeros)                                    |
| 6   | Zero-Lower Flag | ZLF    | Result's lower bits are zeros                                                                  |

A flag is called _set_ if its value is `1`.
The other bits of this register are unused and so are always equal to `0`.

### Exceptions

An _exception_ occurs either when an fatal error occurs, or when an interruption is called. Here is the list errors:

| Code   | Description                                                          | Associated data                    |
| ------ | -------------------------------------------------------------------- | ---------------------------------- |
| `0x01` | Provided opcode does not match any known operation's opcode          | Faulty opcode                      |
| `0x02` | Provided registry identifier does not match any known register's one | Faulty registry identifier         |
| `0x03` | This registry cannot be read from this mode                          | Registry identifier                |
| `0x04` | This registry cannot be written from this mode                       | Registry identifier                |
| `0x05` | Provided memory address is unaligned (not a multiple of 4)           | Unalignment (faulty_address % 4)   |
| `0x06` | Cannot read from this memory address in this mode                    | Address' weakest 16 bits           |
| `0x07` | Cannot write to this memory address in this mode                     | Address' weakest 16 bits           |
| `0x08` | Cannot execute an instruction from this memory address in this mode  | Address' weakest 16 bits           |
| `0x09` | This instruction can only be ran in supervisor mode                  | Faulty opcode                      |
| `0x0A` | Cannot perform a division or modulus by 0                            |                                    |
| `0x0B` | Forbidden operation overflow (division or modulus by -1 overflowed)  |                                    |
| `0x0C` | Invalid flag provided in `IF` or `IF2` instruction                   | Faulty flag                        |
| `0x0D` | Invalid condition mode provided in `IF2` instruction                 | Faulty code                        |
| `0x10` | Unknown component ID in `HWD` instruction                            | Faulty ID (weakest 16 bits)        |
| `0x11` | Invalid hardware information code in `HWD` instruction               | Faulty code                        |
| `0x12` | Component is not mapped                                              | Faulty ID (weakest 16 bits)        |
| `0xA0` | Hardware exception                                                   | Exception's code & associated data |
| `0xF0` | An interruption occurred                                             | Interruption code                  |

The content of the exception type `et` register is as follows, starting from the strongest byte:

- Bits 00-07: `0x00` = exception occurred in userland, `0x01` = exception occurred in supervisor mode
- Bits 08-15: code of the exception (see table above)
- Bits 16-31: associated data (see table above)

When an exception occurs, three things happen:

- The value of the exception vector `ev` is copied into the program counter `pc`
- The value of `et` is set according to the above description
- If the exception occurred in supervisor mode, the "byte for mode" is `1`, otherwise it's `0`
- Supervisor mode is toggled on using `smt`

## Startup

When the CPU starts, its sets all registers to `0`, except `smt` with is set to `1` to enable supervisor mode.
This behaviour results reading the first instruction from address `0` of the memory.

## Memory Management Unit

The Memory Management Unit, abbreviated MMU, allows to translate virtual adresses to physical adresses using adress tables.
It is the only component to have direct access to the memory.

The MMU only allows to read and write whole words (32-bits values) on unaligned addresses (multiples of 4 bytes).

### Physical address pages

When the `mtt` (Memory Translation Toggler) register is set to a non-zero value, the MMU will translate all memory addresses using physical address pages. Their purpose is to translate a virtual address into a physical one that will be provided to the memory module.

Now, let's see how the MMU translates addresses, which is important to understand how to structure the pages:

The `pda` register (Page Directory Address) points to the virtual address pages index (abbreviated VPI). The strongest 10 bits of the virtual address is used to determine the entry to read (each entry being a word, so 32 bits). Each entry uses the following format (starting from strongest bit):

- Bit 0: pass-through in supervisor mode
- Bit 1: pass-through in userland mode
- Bit 2: `READ` permission in supervisor mode
- Bit 3: `WRITE` permission in supervisor mode
- Bit 4: `EXEC` permission in supervisor mode
- Bit 5: `READ` permission in userland mode
- Bit 6: `WRITE` permission in userland mode
- Bit 7: `EXEC` permission in userland mode
- Bits 8-31: virtual address page (VAP)'s number

If the pass-through bit is set for the current mode, the virtual address is returned as it is (so the steps below are not performed).

When the CPU tries to access a memory address, it also tells the MMU what action it wants to perform (`READ`, `WRITE` or `EXEC`). If the permission is not set for the current mode (if the bit is equal to `0`), the instruction fails with an [exception](#exceptions).

If the permission is set (if the bit is equal to `1`), the VAP's number is taken from the weakest 24 bits of the entry as shown above and its address is computed as the page's number multiplied by the size of a page (16 KB = 16 384 bytes).

The VAP's entry number is computed as the bits 9 to 21 (0 being the strongest) of the address, so 12 bits in total which means 4096 possibilities. As an entry is made of 4 bytes, this leads us to the previous page size of 16 384 bytes = 16 KB.

The VAP's entry's address is finally computed as the VAP's address plus the VAP's entry number multiplied by size of entries (1 word = 4 bytes = 32 bits).

The said entry is then read from the memory and decoded using the same format as above, which means we have an additional layer of pass-through and permissions.

If pass-through is disabled and the required permission is set, the entry's 24 weakest bits are used as the physical page number. The physical address is then computed as the physical page number multiplied by the size of a physical page (1024 bytes) plus the 10 weakest bits of the virtual address.

As virtual pages can contain 4 096 entries (16 KB / 4 bytes per entry), and physical pages address 1 KB each, we can map up to 4 MB of memory per virtual page. And as the index can contain 1 024 entries, we can address up to 4 GB of memory, which is 2^32 bytes: the total of the addressable space of the VM.

## Memory-Mapped Input/Output

The MMIO allows to map contiguous block of the memory to _bus_, which allow synchronous read and write operations.
It does not tolerate any exception except than out-of-range addresses ; therefore a bus can never fail an operation.

Memory can only read and write whole words (32-bits values).
It also only supports aligned addresses, which means each address must be a multiple of 4.

## The stack

The CPU contains a `PUSH` and a `POP` instruction that enables the use of a _stack_. It works very simply:

The stack's next item's address is stored in the `ssp` register (for supervisor mode) or `usp` register (for userland mode).

When a value is `PUSH`ed to it, the provided value is written to the current stack address and the register is decreased by 4.  
When a value is `POP`ed from it, the register is increased by 4 and the new address's value is written to the provided register.

## Processor instructions

The process interprets instructions on 32 bits.
When an instruction ends, the program counter `pc` is increased by 4, _unless_ it has been written by the current instruction.

### Instructions format

Each is composed as follows:

- 5 bits for the _opcode_
- 1 bit to indicate if the first parameter is a register (otherwise it's a constant value) - only for supported instructions
- 1 bit to indicate if the second parameter is a register (otherwise it's a constant value) - only for supported instructions
- 1 bit to indicate if the third parameter is a register (otherwise it's a constant value) - only for supported instructions
- 8 bits per parameter (0 to 3)

If there are unused bits (< 3 parameters), the remaining (weakest) bits are simply ignored by the processor.

There are also a few rules:

- When an instruction takes parameters, the first one is always accept a register (and sometimes a constant value)
- If an instruction takes a single parameter, it will always be a register (not a constant value)
- All parameters have a size of 1 byte, except the last parameter which _can_ have a size of 1 to 3 bytes
- All parameters accepting a constant value also accept a register
- All parameters accepting a register or a constant value will pad the constant value with zeros on its left to fill the 32-bit space

When an instruction accepts either a register or a constant on multiple bytes as its final parameter, the register shall be specified on the weakest byte.

### Assembly language

The LASM (Lightweight ASseMbly) language is a 1:1 translation of the CPU's supported instructions.
Below is the detailed list of its instructions along with their description.

#### Constants

A few constants are defined by the language, and can be used anywhere in the program.
They cannot be re-declared manually.

##### Flags

| Name  | Value  | Description      |
| ----- | ------ | ---------------- |
| `ZF`  | `0x00` | Flag: Zero       |
| `CF`  | `0x01` | Flag: Carry      |
| `OF`  | `0x02` | Flag: Overflow   |
| `SF`  | `0x03` | Flag: Sign       |
| `EF`  | `0x04` | Flag: Even       |
| `ZUF` | `0x05` | Flag: Zero-Upper |
| `ZLF` | `0x06` | Flag: Zero-Lower |

#### Division modes

| Name          | Value  | Description                                                         |
| ------------- | ------ | ------------------------------------------------------------------- |
| `DIV_USG`     | `0x00` | Perform an unsigned division/modulus                                |
| `DIV_SIG`     | `0x10` | Perform a signed division/modulus                                   |
| `DIV_ZRO_FRB` | `0x00` | Forbid div/mod by zero                                              |
| `DIV_ZRO_MIN` | `0x04` | Make div/mod by zero result in the minimum value                    |
| `DIV_ZRO_ZRO` | `0x08` | Make div/mod by zero result in zero                                 |
| `DIV_ZRO_MAX` | `0x0C` | Make div/mod by zero result in the maximum value                    |
| `DIV_OFW_FRB` | `0x00` | Forbid div/mod by zero                                              |
| `DIV_OFW_MIN` | `0x01` | Make div/mod of the minimum value by -1 result in the minimum value |
| `DIV_OFW_ZRO` | `0x02` | Make div/mod of the minimum value by -1 result in zero              |
| `DIV_OFW_MAX` | `0x03` | Make div/mod of the minimum value by -1 result in the maximum value |

#### `IF2` conditions

| Name        | Value  | Description                                    |
| ----------- | ------ | ---------------------------------------------- |
| `CMP_OR`    | `0x01` | Checks if at least one of the two flags is set |
| `CMP_AND`   | `0x02` | Checks if both flags are set                   |
| `CMP_XOR`   | `0x03` | Checks if exactly one of the two flags is set  |
| `CMP_NOR`   | `0x04` | Checks if none of the two flags is set         |
| `CMP_NAND`  | `0x05` | Checks if up to one of the two flags is set    |
| `CMP_LEFT`  | `0x06` | Checks if only the first flag is set           |
| `CMP_RIGHT` | `0x07` | Checks if only the second flag is set          |

#### `HWD` hardware information codes

| Name             | Value  | Description                                                                                        |
| ---------------- | ------ | -------------------------------------------------------------------------------------------------- |
| `HWD_COUNT`      | `0x00` | Get the number of auxiliary components                                                             |
| `HWD_UID_UPPER`  | `0x01` | Get the component's unique identifier's 32 strongest bits                                          |
| `HWD_UID_LOWER`  | `0x02` | Get the component's unique identifier's 32 weakest bits                                            |
| `HWD_NAME_LEN`   | `0x10` | Get the component's name's (UTF8-encoded) length, in bytes (maximum is 32 bytes)                   |
| `HWD_NAME_W1`    | `0x11` | Get the component's name's strongest bytes 00 to 03                                                |
| `HWD_NAME_W2`    | `0x12` | Get the component's name's strongest bytes 04 to 07                                                |
| `HWD_NAME_W3`    | `0x13` | Get the component's name's strongest bytes 08 to 11                                                |
| `HWD_NAME_W4`    | `0x14` | Get the component's name's strongest bytes 12 to 15                                                |
| `HWD_NAME_W5`    | `0x15` | Get the component's name's strongest bytes 16 to 19                                                |
| `HWD_NAME_W6`    | `0x16` | Get the component's name's strongest bytes 20 to 23                                                |
| `HWD_NAME_W7`    | `0x17` | Get the component's name's strongest bytes 24 to 27                                                |
| `HWD_NAME_W8`    | `0x18` | Get the component's name's strongest bytes 28 to 31                                                |
| `HWD_SIZE`       | `0x20` | Get the component's size (maximum is 2^32-1 bytes)                                                 |
| `HWD_CAT`        | `0x21` | Get the component's category                                                                       |
| `HWD_TYPE`       | `0x22` | Get the component's type                                                                           |
| `HWD_MODEL`      | `0x23` | Get the component's model                                                                          |
| `HWD_DATA_UPPER` | `0x24` | Get the component's additional data's 32 strongest bits                                            |
| `HWD_DATA_LOWER` | `0x25` | Get the component's additional data's 32 weakest bits                                              |
| `HWD_IS_MAPPED`  | `0xA0` | Check if the component is mapped (writes `0x01` in the destination register if it is, `0x00` else) |
| `HWD_MAP_START`  | `0xA1` | Get the component's mapping start address (raises `0x0D` exception if component is not mapped)     |
| `HWD_MAP_END`    | `0xA2` | Get the component's mapping end address (raises `0x0D` exception if component is not mapped)       |

These constants may be provided to use as parameters or masks in some instructions ; see the related instructions for more details.

#### Notation

The notation used to describe instructions is as follows:

- The instruction's name is capitalized ;
- Parameters are separated by a comma (`,`) ;
- Parameters that require a register are prefixed by `reg` ;
- Parameters that require a value have a different name, prefixed by a number to indicate the required number of bytes

The "upper bits" of a register refer its 16 strongest bits, while its "lower bits" refer to its 16 weakest ones.

Double brackets indicate the parameter is optional ; while it must be provided to get a 4-byte-long instruction, the assembler will accept usages of the instruction without the said parameter.

The `{S}` notation after an instruction's name indicates it requires to be in supervisor mode (otherwise an exception is raised).

#### Reading hardware informations

The [`HWD` instruction](#hardware-access-instructions) allows to get informations about the hardware. Informations about a specific component can be retrieved with the `hw_info` parameter, which indicates which type of information must be retrieved, between:

- `0x00` = when provided with ID 0, get the number of connected components
- `0x01` = get the component's unique identifier's 32 strongest bits
- `0x02` = get the component's unique identifier's 32 weakest bits
- `0x10` = get the component's name's (UTF8-encoded) length, in bytes (maximum is 32 bytes)
- `0x11` = get the component's name's strongest bytes 00 to 03
- `0x12` = get the component's name's strongest bytes 04 to 07
- `0x13` = get the component's name's strongest bytes 08 to 11
- `0x14` = get the component's name's strongest bytes 12 to 15
- `0x15` = get the component's name's strongest bytes 16 to 19
- `0x16` = get the component's name's strongest bytes 20 to 23
- `0x17` = get the component's name's strongest bytes 24 to 27
- `0x18` = get the component's name's strongest bytes 28 to 31
- `0x20` = get the component's size (maximum is 2^32-1 bytes)
- `0x21` = get the component's category
- `0x22` = get the component's type
- `0x23` = get the component's model
- `0x24` = get the component's additional data's 32 strongest bits
- `0x25` = get the component's additional data's 32 weakest bits
- `0xA0` = check if the component is mapped (writes `0x01` in the destination register if it is, `0x00` else)
- `0xA1` = get the component's mapping start address (raises `0x0E` exception if component is not mapped)
- `0xA2` = get the component's mapping end address (raises `0x0E` exception if component is not mapped)

#### Assignment instructions

The assignment instructions allow to modify all are part of a register:

- `CPY reg_dest, [reg_value | 2-bytes]` (CoPY) | opcode: `0x01`  
  Copy the provided value into `reg_dest`
  If a value is provided, this zeroes the register's upper bits and assigns the value to its lower bits
  **Affects** `reg_dest`

- `EX reg_a, reg_b` (EXchange) | opcode: `0x02`  
  Swap the value stored in `reg_a` with one to `reg_b`  
  Equivalent to: `CPY <tmp>, reg_a` + `CPY reg_a, reg_b` + `CPY reg_b, <tmp>`
  **Affects** `reg_a`, `reg_b`

#### Arithmetic instructions

The arithmetic instructions allow to perform integer-based mathematical instructions and set the `af` register.
All operations use two's complement representation for calculus, which means it has no impact when the two operands have their first bit off.

Below is the list of arithmetic instructions:

- `ADD reg, [reg_val | 2-bytes]` (ADD) | opcode: `0x03`  
  Add the provided value to `reg` and put the result in `reg`
  **Affects** `reg`, `af`

- `SUB reg, [reg_val | 2-bytes]` (SUBstract) | opcode: `0x04`  
  Subtract the provided value to `reg` and put the result in `reg`
  **Affects** `reg`, `af`

- `MUL reg, [reg_val | 2-bytes]` (MULtiply by) | opcode: `0x05`  
  Multiply `reg` by the provided value and put the result in `reg`
  **Affects** `reg`, `af`

- `DIV reg, [reg_val | 1-byte], [[reg_mode | 1-mode]]` (DIVide by) | opcode: `0x06`  
  Divide `reg` by the provided value and put the result in `reg`

  **MODE**  
  By default, this instruction computes an unsigned division.  
  By default, this instruction raises an exception if the right operand is zero or if the left operand is -2^32 and the right operand is -1.

  Considering bit 0 of the mode as its strongest bit:  
  If bit 3 is set, this instruction will perform a signed division instead of an unsigned one

  If bit 4-5 are equal to `01`, this instruction will accept division by zero and make the result equal to -2^32  
  If bit 4-5 are equal to `10`, this instruction will accept division by zero and make the result equal to 0  
  If bit 4-5 are equal to `11`, this instruction will accept division by zero and make the result equal to 2^32-1

  If bit 6-7 are equal to `01`, this instruction will accept divisions of -2^32 by -1 and make the result equal to -2^32  
  If bit 6-7 are equal to `10`, this instruction will accept divisions of -2^32 by -1 and make the result equal to 0  
  If bit 6-7 are equal to `11`, this instruction will accept divisions of -2^32 by -1 and make the result equal to 2^32

  When a division by zero or -2^32 by -1 happens and is accepted by the mode, both carry and overflow flags are set.

  **Affects** `reg`, `af`  
  **Exceptions** `0x0A` for forbidden division by zero, `0x0B` for forbidden overflowing division by -1 (in signed mode)

- `MOD reg, [reg_val | 1-byte], [[reg_mode | 1-mode]]` (MODulus) | opcode: `0x07`  
  Compute the modulus of `reg` by the provided value and put the result in `reg`  
  As computations are integer-based, this is equal to the remainder of `reg` by the provided value  
  The provided mode is interpreted the same way as for the `DIV` instruction
  **Affects** `reg`, `af`  
  **Exceptions** `0x0A` for forbidden modulus by zero, `0x0B` for forbidden overflowing modulus by -1 (in signed mode)

#### Bitwise instructions

The bitwise instructions allow to perform bit-by-bit instructions.

- `AND reg, [reg_val | 2-bytes]` (AND) | opcode: `0x08`  
  Perform a bit-by-bit AND operation between `reg` and the provided value, and put the result in `reg`
  **Affects** `reg`, `af`

- `BOR reg, [reg_val | 2-bytes]` (Bit-by-bit OR) | opcode: `0x09`  
  Perform a bit-by-bit OR operation between `reg` and the provided value, and put the result in `reg`
  **Affects** `reg`, `af`

- `XOR reg, [reg_val | 2-bytes]` (EXclusive OR) | opcode: `0x0A`  
  Perform a bit-by-bit XOR operation between `reg` and the provided value, and put the result in `reg`
  **Affects** `reg`, `af`

- `SHL reg, [reg_val | 1-byte]` (Left SHift) | opcode: `0x0B`  
  Perform a left shift operation of the provided number of bits on `reg`, and put the result in `reg`
  **Affects** `reg`, `af`

- `SHR reg, [reg_val | 1-byte]` (Right SHift) | opcode: `0x0C`  
  Perform a right shift operation of the provided number of bits on `reg`, and put the result in `reg`
  **Affects** `reg`, `af`

#### Logical instructions

The logical instructions are:

- `CMP reg, [reg_val | 2-bytes]` (CoMPare) | opcode: `0x0D`  
  Compare the value of `reg` to the provided value  
  Concretely, this operation does the same as `SUB` but stores the result nowhere - though it still sets the arithmetic flags  
  This means that, if for instance the 'zero' flag is set, `reg_val` and the provided value are equal.  
  **Affects** `af`

#### Control flow instructions

The control flow instructions allow to control the program's flow by changing the address of the next instruction:

- `JPR [reg_jump | 2-bytes]` (JumP Relatively) | opcode: `0x0E`  
  Jump by as many bytes as specified  
  The number of bytes is interpreted using two's complement representation  
  **Affects** `pc`

- `LSM [reg_addr | 2-bytes]` (Leave Supervisor Mode) {S} | opcode: `0x0F`  
  Jump at the provided address and disable the supervisor mode just after  
  Jumping by assigning to `pc` and then disabling manually the supervisor mode would result in a page fault when the MMU is enabled,
  as the second assignment instruction's address could not be read due to userland privileges.  
  **Affects** `pc`, `smt`

- `ITR [reg_code | 1-byte]` (InTeRruption) | opcode: `0x10`  
  Raise an interruption with the provided code (exception with code `0xAA`)  
  See the in-depth explanations on exceptions to see what this instruction actually does  
  **Affects** `pc`, `et`, `era`, `smt`

#### Conditional instructions

The conditional instructions allow to instruction following them only if a condition is met.
Under the hood, they simply jump four bytes forward if the condition is not met, and nothing otherwise.

- `IF [reg_flag]` (IF) | opcode: `0x11`  
  Run the next instruction only if the specified flag is set  
  **Affects** `pc`  
  **Exceptions** `0x0C` if the provided flag does not exist

- `IFN [reg_flag]` (IF Not) | opcode: `0x12`  
  Run the next instruction only if the specified flag is _not_ set  
  **Affects** `pc`  
  **Exceptions** `0x0C` if the provided flag does not exist

- `IF2 [reg_flag_a], [reg_flag_b], [reg_cond | 1-byte]` (IF) | opcode: `0x13`
  Run the next instruction only if the specified condition is met

  `0x01` at least one of the two flags is set (OR)  
  `0x02` both flags are set (AND)  
  `0x03` exactly one of the two flags is set (XOR)  
  `0x04` none of the two flags is set (NOR)  
  `0x05` up to one of the two flags is set (NAND)  
  `0x06` only the first flag is set  
  `0x07` only the second flag is set

  Providing an invalid code will raise an `0x0F` exception.

  **Affects** `pc`  
  **Exceptions** `0x0C` if any of the provided flags does not exist, `0x0D` if the provided condition does not exist

#### Memory read/write instructions

The memory instructions allow to manipulate the memory:

- `LSA reg_dest, [reg_addr | 1-byte], [[reg_add | 1-byte]]` (Load Simple Address) | opcode: `0x14`  
  Read the word at address (`addr` + `add` (signed)) in `reg_dest`  
  By default `add` is zero  
  The address must be aligned or an exception will be raised  
  **Affects** `reg_dest`

- `LEA [reg_addr | 1-byte], [[reg_add | 1-byte], [[reg_mul | 1-byte]]]` (Load Effective Address) | opcode: `0x15`  
  Read the word at address (`addr` + `add` (signed) \* `mul` (signed)) in `avr`  
  By default `add` is zero and `mul` is one  
  As the `avr` is used for atomic instructions, its value is expected to be moved to another register to operate on  
  The address must be aligned or an exception will be raised
  **Affects** `avr`

- `WSA [reg_addr | 1-byte], [[reg_add | 1-byte], [[reg_val | 1-byte]]]` (Write Simple Address) | opcode: `0x16`  
  Write the provided value to address at (`addr` + `add` (signed))  
  By default `add` is zero  
  The address must be aligned or an exception will be raised

- `WEA [reg_addr | 1-byte], [[reg_add | 1-byte], [[reg_mul | 1-byte]]]` (Write Effective Address) | opcode: `0x17`  
  Write the value of `avr` to address at (`addr` + `add` (signed) \* `mul` (signed))  
  By default `add` is zero  
  The address must be aligned or an exception will be raised

- `SRM [reg_addr | 1-byte], [reg_add | 1-byte], reg_swap` (Swap Register and Memory) | opcode: `0x18`  
  Swap the values of `reg_swap` and address at (`reg_addr` + `reg_add` (signed))  
  Equivalent to `LSA <temporary>, addr, add`, `WSA addr, add, reg_swap`, `CPY reg_swap, <temporary>`  
  **Affects** `reg_swap`

- `PUSH [reg_value | 2-bytes]` (PUSH) | opcode: `0x19`  
  Decrease the address stored in current mode's stack pointer by 4, then write the provided value to the new address.  
  If the memory cannot be written, the stack pointer register is left unchanged.  
  **Affects** `ssp` OR `usp`

- `POP reg_dest` (POP) | opcode: `0x1A`  
  Read the value at the address pointed by the stack pointer, then increases it by 4.  
  If the memory cannot be read, the stack pointer register is left unchanged.  
  **Affects** `reg_dest` and `ssp` OR `usp`

- `CALL [reg_addr | 2-bytes]` (CALL) | opcode: `0x1B`  
  Push the current address + 4 to the stack and jump to the provided address.  
  If the memory cannot be written, the stack pointer register is left unchanged.  
  **Affects** `pc` and `ssp` OR `usp`

#### Hardware access instructions

- `HWD reg_dest, [reg_id | 1-byte], [reg_hw_info | 1-byte]` (HardWare Data) | opcode: `0x1C`  
  Read a hardware information from a component.  
  The provided hardware identifier is the component's index, first component mounted to the motherboard getting ID 0.  
  To get the number of connected components, provide ID `0` and info number `0`.  
  The `hw_info` indicates which information must be retrieved, see [hardware informations](#reading-hardware-informations).

  **Affects** `reg_dest`

  **Exceptions**

  - `0x10` if the provided ID is unknown
  - `0x11` if the provided hardware information code is unknown
  - `0x12` if the component when retrieving mapping if the component is not mapped

#### Processor control instructions

These instructions allow to control how the processor behave or to get informations about it:

- `CYCLES reg_dest` (CYCLES) {S} | opcode: `0x1D`  
  Copy the number of cycles performed since the CPU awoken in the provided registry  
  **Affects** `reg_dest`

- `HALT` (HALT) {S} | opcode: `0x1E`  
  Halt the processor

- `RESET [reg_mode | 1-byte]` (RESET) {S} | opcode: `0x1F`  
  The mode is split in two bytes to indicate reset informations for the processor and for the auxiliary components.

  Strongest byte:

  - `0x0`: reset the processor
  - Other: doesn't reset the process

  Weakest byte:

  - `0x0`: resets all components
  - `0x1`: resets the component whose ID is in `avr`
  - `0x2`: resets all components except the one in `avr`
  - `0x3`: resets all components with an ID smaller than or equal to the one in `avr`
  - `0x4`: resets all components with an ID greater than or equal to the one in `avr`
  - Other: doesn't reset any component

  The processor is always reset _after_ the specified components (if any).

#### Alias instructions

There are a few _alias instructions_, which are strict aliases of existing instructions which pre-use some common parameters/conditions:

- `ZRO reg` (ZeRO)  
  Zeroes a registry  
  Alias of: `XOR reg, reg`

- `INC reg` (INCrement)  
  Increase a registry  
  Alias of: `ADD reg, 1`

- `DEC reg` (DECrement)  
  Decrease a registry  
  Alias of: `SUB reg, 1`

- `IFEQ` (IF EQual)
  Alias of: `IF ZF`

- `IFNQ` (IF Not eQual)
  Alias of: `IFN ZF`

- `IFGT` (If GreaTer)  
  Alias of: `IF2 ZF, CF, CMP_NOR`

- `IFGE` (If Greater or Equal)  
  Alias of: `IFN CF`

- `IFLS` (If LeSs)  
  Alias of: `IF CF`

- `IFLE` (If Less or Equal)  
  Alias of: `IF2 ZF, CF, CMP_OR`

- `IFOR [reg_flag_a | 1-byte], [reg_flag_b | 1-byte]` (IF OR)  
  Alias of: `IF2 flag_a, flag_b, CMP_OR`

- `IFAND [reg_flag_a | 1-byte], [reg_flag_b | 1-byte]` (IF AND)  
  Alias of: `IF2 flag_a, flag_b, CMP_AND`

- `IFXOR [reg_flag_a | 1-byte], [reg_flag_b | 1-byte]` (IF XOR)  
  Alias of: `IF2 flag_a, flag_b, CMP_XOR`

- `IFNOR [reg_flag_a | 1-byte], [reg_flag_b | 1-byte]` (IF NOR)  
  Alias of: `IF2 flag_a, flag_b, CMP_NOR`

- `IFNAND [reg_flag_a | 1-byte], [reg_flag_b | 1-byte]` (IF NAND)  
  Alias of: `IF2 flag_a, flag_b, CMP_NAND`

- `IFLEFT [reg_flag_a | 1-byte], [reg_flag_b | 1-byte]` (IF LEFT)  
  Alias of: `IF2 flag_a, flag_b, CMP_LEFT`

- `IFRIGHT [reg_flag_a | 1-byte], [reg_flag_b | 1-byte]` (IF RIGHT)  
  Alias of: `IF2 flag_a, flag_b, CMP_RIGHT`

- `JP [reg_addr | 2-bytes]` (JumP)
  Go to the provided address  
  ALias of: `CPY cp, [reg_addr | 2-bytes]`

- `RET` (RETurn)
  Pop the stack in `pc`  
  If the memory cannot be read, the stack pointer register is left unchanged.  
  Alias of: `POP <pc>`
