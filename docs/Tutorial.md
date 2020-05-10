# Tutorial

This tutorial will show you how to use MRVM and associated tools to run and debug a VM.

The final code can be found in the [`examples/hello_world`](../examples/hello_world/) directory.

## 0. Preparing a Rust project

First, create a new cargo project and add [`mrvm`](../mrvm/), [`mrvm_aux`](../mrvm_aux/) and [`mrvm_tools`](../mrvm_tools/) as dependencies.
Also add the [`rand`](https://crates.io/crates/rand) crate dependency as we will use it to generate pseudo-unique identifiers.

## 1. Setting up the VM

It's time to create our VM. In the `src/main.rs` file, let's prepare the motherboard that will receive the components:

```rust
use mrvm::board::MotherBoard;

fn main() {
    let motherboard = MotherBoard::new(vec![
        // TODO
    ]);
}
```

Now we have to choose the components we want to connect to the motherboard. We first need a BootROM, which is a read-only memory that will contain our program's instructions.

As we will not be able to access the components once the motherboard has been created, we need to prepare the data we will put in the BootROM first. For now, let's put that aside and create an empty storage.

Because it's easier to work with easy-to-memorize addresses, we will make our BootROM `0x1000` bytes long. This means that, while our program is shorter than this size, the start address of the next component will always be `0x1000` if we map them contiguously in the memory.

We also need to choose an _hardware identifier_ for each component, which is a 64-bit-long identifier that is unique to each component - we must not have any duplicate. The de-facto way for this is to generate random unique identifiers, but as that's a bit complex we'll simply generate pseudo-unique identifiers using the [`rand`](https://crates.io/crates/rand) crate, the chances of getting a collision with as few components as we'll have being extremely low.

```rust
use rand::Rng;
use mrvm::board::{MotherBoard, Bus};
use mrvm_aux::storage::BootROM;
use mrvm_aux::display::BufferedDisplay;

fn main() {
    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(vec![], 0x1000, rng.gen()).unwrap())
    ];

    let motherboard = MotherBoard::new(components);
}

```

We'll also need a writable memory to store informations, as the BootROM is read-only - which prevents us from rewriting our own program accidentally.

```rust
use rand::Rng;
use mrvm::board::{MotherBoard, Bus};
use mrvm_aux::storage::BootROM;
use mrvm_aux::memory::VolatileMem;

fn main() {
    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(vec![], 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap())
    ];

    let motherboard = MotherBoard::new(components);
}
```

Finally, we'll add a small display component called a _buffered display_. The concept is pretty simple: we write the bytes to display in a buffer, then ask the component to display the buffer's content.

```rust
use rand::Rng;
use mrvm::board::{MotherBoard, Bus};
use mrvm_aux::storage::BootROM;
use mrvm_aux::memory::VolatileMem;
use mrvm_aux::display::BufferedDisplay;

fn main() {
    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(vec![], 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid UTF-8 input received>"))
        ), rng.gen()).unwrap())
    ];

    let motherboard = MotherBoard::new(components);
}
```

Our motherboard is now ready. But we still have a thing to do: prepare our program.

## 2. Prepare the program

MRVM uses a small assembly language called LASM (Lightweight Assembly). We'll start by making a simple program that displays `Hello, world!`.

So, we first need to encode the message in our program. A first method is to check what bytes `Hello, world!` is made of and write these bytes directly in our assembly program. But that's tricky, and we'll not be able to change this message easily later.

So we'll use a simple directive called `#str`, provided by the assembling library MRVM uses under the hood, [CustomASM](https://github.com/hlorenzi/customasm).

Let's make a label that contains the message:

```asm
message:
    #str "Hello, world!"
```

We will copy the message's bytes, one by one, to the display's buffer. This means we also need to know _when_ we reached the end of the message. The easiest way is to simply put a `0x00000000` value at the end of the message:

```asm
message:
    #str "Hello, world!"
    #d32 0
```

To copy the bytes, we'll need to write the message's address in the CPU's registers:

```asm
main:
    ; ac0 = address computation 0
    ; message = address of the "message" label
    cpy ac0, message
```

We also need to know the address of the display's buffer:

```asm
main:
    cpy ac0, message
    cpy ac1, 0x2000 ; Address of the buffer
```

Because our writing instruction cannot take literal addresses higher than `0xFF`, we'll also put the address of the buffer's last word (in which we write the action we want the display to perform) in `ac2`:

```asm
main:
    cpy ac0, message
    cpy ac1, 0x2000
    cpy ac2, 0x2100 - 0x04
```

Alright. Now we will write the loop that copies the bits. Just after our `main` label, we can make a _local label_ - a label that is only available inside the label it is declared in -. Let's call it `.copy_byte`.

The first thing we need to do is to read the current byte from the message to a register, `a0` (`a` stands for arithmetic). This can be achieved using the `lsa` instruction, which takes the destination register, the register containing the address to read, and a value to add to the provided address.

```asm
.copy_byte:
    lsa a0, ac0, 0
```

If the read word is equal to `0x00000000`, we reached the end of the message, so we can stop reading. We can do this comparison using the `cmp` instruction:

```asm
.copy_byte:
    lsa a0, ac0, 0
    cmp a0, 0
```

This instruction substracts the second value to the first, and sets the CPU's _arithmetic flags_. As we want to check if our value is equal to `0`, we simply need to check if the zero flag (`ZF`) is set: if `a0 - 0 = 0`, this means `a0 = 0`.

```asm
.copy_byte:
    lsa a0, ac0, 0
    cmp a0, 0
    if zf
```

But we're lucky! There is an alias for `if zf` that makes it more readable, `ifeq`. It does exactly the same thing (in fact it results in the same machine code), but it's more easy to read.

```asm
.copy_byte:
    lsa a0, ac0, 0
    cmp a0, 0

    ifeq
```

If the values are equal, we will jump to a new local label called `.display`, which asks the display to print the buffer's content.

```asm
.copy_byte:
    lsa a0, ac0, 0
    cmp a0, 0

    ifeq
    jmpa .display
```

The `if` instructions only run the instruction below them if the specified flag is set. This means that, if the `ZF` flag is not set (so if `a0` is not equal to `0x00000000`), our `jmpa` instruction won't be run and the program won't jump. Which means the instructions we'll put below `jmpa` will be run in turn.

So the first thing we'll do in this case is to write the read word to the display's buffer:

```asm
.copy_byte:
    lsa a0, ac0, 0
    cmp a0, 0

    ifeq
    jmpa .display

    wsa ac1, 0, a0
```

Then, we'll increment `ac0` (which contains the address of the word to read) and `ac1` (which contains the address to write at) by `4`, as we write words (groups of 4 bytes) and not single bytes.

```asm
.copy_byte:
    lsa a0, ac0, 0
    cmp a0, 0

    ifeq
    jmpa .display

    wsa ac1, 0, a0

    add ac0, 4
    add ac1, 4
```

And finally, we loop to `.copy_byte`:

```asm
.copy_byte:
    lsa a0, ac0, 0
    cmp a0, 0

    ifeq
    jmpa .display

    wsa ac1, 0, a0

    add ac0, 4
    add ac1, 4

    jmpa .copy_byte
```

Finally, let's make the `.display` label. To ask a buffered display to print its content, we need to write `0xAA` at its very last word:

```asm
.display:
    ; We set the value of `ac2` to the address of the buffered display's last word at the beginning of `main:`
    wsa ac2, 0, 0xAA
```

If we stop here, the program will continue to execute instructions later on. Which means it will reach the `message:` label, which contains not instructions but characters, which will be interpreted as (invalid) instructions by the CPU. In best case, this will result in an exception and make our program go back to its beginning, but in the worst the program will simply run invalid instructions.

Apart from this, we also need to indicate the VM our program is finished. So, to achieve both of these goals, we add a simple `halt` instructions, which stops the CPU.

```asm
.display:
    wsa ac2, 0, 0xAA
    halt
```

Now, let's choose the labels' order. In fact, we don't have much choice: `main:` is the entrypoint of our program, and as the CPU always starts reading instructions at address `0x00000000`, we need to put it first. Or we can put it at another place and write a `jmpa` instruction at the very beginning, but there's no point to do this here.

This gives us the following, final program:

```asm
main:
    cpy ac0, message
    cpy ac1, 0x2000
    cpy ac2, 0x2100 - 0x04

.copy_byte:
    lsa a0, ac0, 0
    cmp a0, 0

    ifeq
    jmpa .display

    wsa ac1, 0, a0

    add ac0, 4
    add ac1, 4

    jmpa .copy_byte

.display:
    wsa ac2, 0, 0xAA
    halt

message:
    #str "Hello, world!"
    #d32 0
```

Hurray! Now it's time to run it!

## 3. Starting up the VM

Let's take back our previous Rust code:

```rust
// import statements

fn main() {
    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(vec![], 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid UTF-8 input received>"))
        ), rng.gen()).unwrap())
    ];

    let motherboard = MotherBoard::new(components);
}
```

We now need to _map_ the components to memory in order for our program to be able to read and write to them.

We could map each component one by one, but the easiest way is to ask the memory to map each component contiguously, in order. We can achieve this using the `.map_contiguous` method:

```rust
fn main() {
    // ...

    // The '.map' method requires the motherboard variable to be declared as mutable
    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        // We ask the memory to map all components contiguously
        mem.map_contiguous(0x00000000, [ 0, 1, 2 ]).mapping.unwrap();
    });
}
```

By default, the CPU is in a "halted" state, meaning it won't do anything if we ask it to run instructions. So, we first need to "wake" him up, by asking the motherboard to send a _reset_ signal to all components. That will also force all connected components to initialize.

```rust
fn main() {
    // ...
    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        // We ask the memory to map all components contiguously
        mem.map_contiguous(0x00000000, [ 0, 1, 2 ]).mapping.unwrap();
    });

    motherboard.reset();
}
```

Great! Now we can run our program by getting a reference to the CPU and asking it to run instructions until it halts, which will happen when it encounters our `halt` instruction.

```rust
fn main() {
    // ...
    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        // We ask the memory to map all components contiguously
        mem.map_contiguous(0x00000000, [ 0, 1, 2 ]).mapping.unwrap();
    });

    let cpu = motherboard.cpu();

    while !cpu.halted() {
        cpu.next().unwrap();
    }
}
```

The `.unwrap()` makes our program panic if an exception occurred. This is better than leaving the VM run in an invalid state, as our program should _not_ generate any exception.

We now have this code:

```rust
fn main() {
    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(vec![], 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid UTF-8 input received>"))
        ), rng.gen()).unwrap())
    ];

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        mem.map_contiguous(0x00000000, [ 0, 1, 2 ]).mapping.unwrap();
    });

    motherboard.reset();

    let cpu = motherboard.cpu();

    while !cpu.halted() {
        cpu.next().unwrap();
    }
}
```

If you try to run this program, the VM will run undefinitely and nothing will be displayed. Why? Because our BootROM is still empty, and so the CPU will encounter an exception (`0x00` is not a valid instruction, but it's the first word it will read from the BootROM). This will make him jump to the exception vector, which is the address written in the `ev` register, that indicates the CPU where to jump after an exception occurred so the program can handle it.

But as we didn't use any exception handling, this will make the CPU jump to the default value of `ev`, which is `0x00000000` - the start address of the BootROM. This is an infinite loop: the CPU reads this address, finds an unknown instruction, jumps to the address provided by `ev`, which is the same that caused the error.

So, let's now prepare our BootROM. First, create in the same directory as your `main.rs` file a source file named `display.lasm`, and put inside the
assembly program we made.

We'll now be able to call MRVM's assembler to generate machine code from our source file. This can be achieved this way:

```rust
// ...
use mrvm_tools::lasm::assemble_words;

fn main() {
    let program = assemble_words(include_str!("display.lasm")).unwrap();

    // ...
}
```

The `assemble_words` function generates a list of machine code words (more specifically, a `Vec<u32>`) from our source file. If the source file is invalid, our program will panic and display the error (CustomASM has an excellent error output so it'll be easy to figure what the error is).

Now our program is assembled, we can put it in our BootROM:

```rust
// ...

fn main() {
    let program = assemble_words(include_str!("display.lasm")).unwrap();

    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(program /* <- HERE */, 0x1000, rng.gen()).unwrap()),
        // ...
    ];

    // ...
}
```

And our BootROM is ready! The final code is:

```rust
use rand::Rng;
use mrvm::board::{MotherBoard, Bus};
use mrvm_aux::storage::BootROM;
use mrvm_aux::memory::VolatileMem;
use mrvm_aux::display::BufferedDisplay;
use mrvm_tools::lasm::assemble_words;

fn main() {
    let program = assemble_words(include_str!("display.lasm")).unwrap();

    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid UTF-8 input received>"))
        ), rng.gen()).unwrap())
    ];

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        mem.map_contiguous(0x00000000, [ 0, 1, 2 ]).mapping.unwrap();
    });

    motherboard.reset();

    let cpu = motherboard.cpu();

    while !cpu.halted() {
        cpu.next().unwrap();
    }
}
```

If you run this program, you should see after a little while a `[Display] Hello, world!` message appear. This can take several seconds as we assemble the source LASM code before actually starting the VM.

### Adding a little more output

In order to know exactly what our program is doing, and display most errors, we can add some debug instructions.

First, let's add `println!()` statements to indicate what our program is doing at keypoints:

- When assembling the source LASM code ;
- When preparing the components and mapping the motherboard's memory ;
- When starting the VM ;
- When the VM stops.

We may also add a little code in our `while !cpu.halted() {` loop to indicate, if an exception happens, what was the faulty address as well as the exception code (they are all details in the [architecture document](Architecture.md#exceptions)).

We can simply replace our `cpu.next();` line by:

```rust
cpu.next().expect(&format!("Exception occurred at address {:#010X}", cpu.regs.pc));
```

As the `pc` registers contains the current instruction. But that wouldn't work, as when an exception occurrs, the CPU instantly jumps to the exception vector, so we would always get the `0x00000000` address in our debug output.

The solution is to get the value of `pc` _before_ asking the CPU to run the instruction, as the register contains the address of the instruction that is _going to be_ run on the next CPU cycle. Which leads us to the following code:

```rust
let was_at = cpu.regs.pc;
cpu.next().expect(&format!("Exception occurred at address {:#010X}", was_at));
```

Here is the final code:

```rust
use rand::Rng;
use mrvm::board::{MotherBoard, Bus};
use mrvm_aux::storage::BootROM;
use mrvm_aux::memory::VolatileMem;
use mrvm_aux::display::BufferedDisplay;
use mrvm_tools::lasm::assemble_words;

fn main() {
    println!("> Assembling LASM code...");

    let program = assemble_words(include_str!("display.lasm")).unwrap();

    println!("> Preparing components and motherboard...");

    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid UTF-8 input received>"))
        ), rng.gen()).unwrap())
    ];

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        mem.map_contiguous(0x00000000, [ 0, 1, 2 ]).mapping.unwrap();
    });

    motherboard.reset();

    println!("> Running the program...");

    let cpu = motherboard.cpu();

    while !cpu.halted() {
        let was_at = cpu.regs.pc;
        cpu.next().expect(&format!("> Exception occurred at address {:#010X}", was_at));
    }

    println!("> CPU halted.");
}
```

This gives us the following display when running our program:

```
> Assembling LASM code...
> Preparing components and motherboard...
> Running the program...
[Display] Hello, world!
> CPU halted.
```

Hurray! You can find this example in the [`examples/hello_world`](../examples/hello_world/) directory.

### A note on performances

If you simply run the program with `cargo run`, you will find it is quite slow - a whole 5 seconds on a `Core i7-9700F` CPU (which is similar to the more known `Core i7-9700K`).

This is especially because CustomASM is quite slow in debug mode (which is Cargo's default build mode), and as assembling the source LASM code is the most part of this program, this slows it down considerably.

So, if you want to check the real performances of the program, run with `cargo run --release`. The results should be a lot better, but at the cost of a longer build time. On the same CPU (`Core i7-9700F`) we go from a bit more than 5 seconds to half a second!

## 4. Using the native debugging tools

The `mrvm_tools` crate also provides useful debugging tools for MRVM in its `mrvm_tools::debug` module. For instance, the `prepare_vm` function takes a list of components and returns a fully-ready motherboard, with contiguously-mapped memory and already reset components. It also displays in the console the memory mappings of each component, along with their hardware identifier.

To use it, we simply need to replace this part of the code:

```rust
// ...

fn main() {
    // ...
    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid input received>"))
        ), rng.gen()).unwrap())
    ];

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        mem.map_contiguous(0x00000000, [ 0, 1, 2 ]).mapping.unwrap();
    });

    motherboard.reset();
    // ...
```

By this one:

```rust
// ...
use mrvm_tools::debug::prepare_vm;

fn main() {
    // ...
    let mut motherboard = prepare_vm(vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid input received>"))
        ), rng.gen()).unwrap())
    ]);
    // ...
```

The output will be something like:

```
=> Component 0000 'BootROM                         ': ✓ 0x00000000 -> 0x00000FFF (HW ID: 0x67 2A 07 37 83 99 3A E3)
=> Component 0001 'Volatile Memory                 ': ✓ 0x00001000 -> 0x00001FFF (HW ID: 0x69 4F 8C B5 28 C3 72 42)
=> Component 0002 'Buffered Display                ': ✓ 0x00002000 -> 0x000020FF (HW ID: 0xF1 51 D3 C2 F4 91 DC AE)
```

The addresses are the component mapping's start and end address, and the `HW ID` is their hardware identifier (the unique identifier we generated using the `rand` crate).

We can also run the VM using the builtin `run_vm` tool, which we can use to display human-readable informations when an exception occurs or when the VM simply halts.

Let's replace this part:

```rust
// ...

fn main() {
    // ...
    let cpu = motherboard.cpu();

    while !cpu.halted() {
        let was_at = cpu.regs.pc;
        cpu.next().expect(&format!("Exception occurred at address {:#010X}", was_at));
    }
    // ...
```

By this one:

```rust
// ...
use mrvm_tools::debug::{run_vm, RunConfig};

fn main() {
  // ...
  run_vm(motherboard.cpu(), &RunConfig::halt_on_ex());
  // ...
```

If the VM halts normally, we will get a message like:

```
Cycle 0x00000025: CPU halted at address 0x00000030
```

But if one happens (let's say we replace the `halt` instruction by a `#d32 0x00`, which gives an invalid opcode of `0x00`), we'll get something like:

```
Cycle 0x00000025: CPU halted at address 0x00000030 because of exception in supervisor mode: Unknown opcode 0x00
```

Which is a lot more readable than the old debug message we had in our previous version.

We now have the following code:

```rust
use rand::Rng;
use mrvm_aux::storage::BootROM;
use mrvm_aux::memory::VolatileMem;
use mrvm_aux::display::BufferedDisplay;
use mrvm_tools::lasm::assemble_words;
use mrvm_tools::debug::{prepare_vm, run_vm, RunConfig};

fn main() {
    println!("> Assembling LASM code...");

    let program = assemble_words(include_str!("display.lasm")).unwrap();

    println!("> Preparing components and motherboard...");

    let mut rng = rand::thread_rng();

    let mut motherboard = prepare_vm(vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid input received>"))
        ), rng.gen()).unwrap())
    ]);

    println!("> Running the program...");

    run_vm(motherboard.cpu(), &RunConfig::halt_on_ex());

    println!("> CPU halted.");
}
```

For comparison with the previous versions, this is what we get if we remove the `println!` statements:

```rust
// ...

fn main() {
    let program = assemble_words(include_str!("display.lasm")).unwrap();

    let mut rng = rand::thread_rng();

    let mut motherboard = prepare_vm(vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid input received>"))
        ), rng.gen()).unwrap())
    ]);

    run_vm(motherboard.cpu(), &RunConfig::halt_on_ex());
}
```

We can even use the `exec_vm` function from the `mrvm_tools::debug` module as we don't do anything between the `prepare_vm` and the `run_vm` calls:

```rust
// ...

fn main() {
    let program = assemble_words(include_str!("display.lasm")).unwrap();

    let mut rng = rand::thread_rng();

    exec_vm(vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid input received>"))
        ), rng.gen()).unwrap())
    ], &RunConfig::halt_on_ex());
}
```

Compared to our previous code:

```rust
// ...

fn main() {
    let program = assemble_words(include_str!("display.lasm")).unwrap();

    let mut rng = rand::thread_rng();

    let components: Vec<Box<dyn Bus>> = vec![
        Box::new(BootROM::with_size(program, 0x1000, rng.gen()).unwrap()),
        Box::new(VolatileMem::new(0x1000, rng.gen()).unwrap()),
        Box::new(BufferedDisplay::new(0x100, Box::new(
            |string| println!("[Display] {}", string.unwrap_or("<invalid input received>"))
        ), rng.gen()).unwrap())
    ];

    let mut motherboard = MotherBoard::new(components);

    motherboard.map(|mut mem| {
        mem.map_contiguous(0x00000000, [ 0, 1, 2 ]).mapping.unwrap();
    });

    motherboard.reset();

    let cpu = motherboard.cpu();

    while !cpu.halted() {
        let was_at = cpu.regs.pc;
        cpu.next().expect(&format!("Exception occurred at address {:#010X}", was_at));
    }
}
```
