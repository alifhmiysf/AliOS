# AliOS v0.1

A minimal experimental operating system kernel written in **Rust**.

AliOS is a learning project created to explore low-level programming, operating system concepts, framebuffer graphics, keyboard input, hardware interrupts, and terminal implementation using Rust.

> **Status:** 🚧 AliOS v0.1 — Experimental / In Development

---

## 🖥️ Features

Current features:

* Bootable x86_64 kernel
* Custom framebuffer rendering
* Custom bitmap font
* Basic terminal interface
* PS/2 keyboard input
* Keyboard hardware interrupt (`IRQ1`)
* Interrupt Descriptor Table (IDT)
* PIC 8259 initialization
* Keyboard interrupt handler
* Scancode processing
* Command buffer
* Command execution
* Cursor rendering
* Blinking cursor
* Terminal line management
* Automatic terminal scrolling
* Terminal clearing
* Basic shell commands

### Available Commands

```text
CLEAR
HELP
ABOUT
```

Example:

```text
> HELP

AVAILABLE COMMANDS:
CLEAR - CLEAR THE SCREEN
HELP - SHOW AVAILABLE COMMANDS
ABOUT - ABOUT ALIOS
```

---

## 🛠️ Tech Stack

* **Rust**
* `no_std`
* `bootloader`
* `x86_64`
* Framebuffer
* PS/2 Keyboard
* 8259 PIC
* Interrupt Descriptor Table (IDT)
* QEMU
* Cargo

---

## 📁 Project Structure

```text
AliOS/
│
├── kernel/
│   ├── src/
│   │   ├── main.rs
│   │   ├── framebuffer.rs
│   │   ├── terminal.rs
│   │   ├── font.rs
│   │   └── interrupts.rs
│   │
│   └── Cargo.toml
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

### `main.rs`

The main kernel entry point and operating system loop.

Responsible for:

* Kernel initialization
* Framebuffer initialization
* Terminal initialization
* Interrupt initialization
* Keyboard input processing
* Scancode-to-character conversion
* Command processing
* Cursor handling
* Shell interaction

### `framebuffer.rs`

Provides the low-level graphics abstraction used by AliOS.

Responsible for:

* Drawing pixels
* Drawing rectangles
* Drawing characters
* Drawing text
* Clearing screen areas
* Scrolling framebuffer regions

### `font.rs`

Contains the bitmap font used by AliOS to render text on the framebuffer.

### `terminal.rs`

Manages terminal state and screen positioning.

Responsible for:

* Current terminal position
* Line movement
* Terminal boundaries
* Terminal scrolling
* Terminal clearing
* Background management

### `interrupts.rs`

Handles CPU and hardware interrupts.

Currently responsible for:

* Creating and loading the Interrupt Descriptor Table (IDT)
* Registering CPU exception handlers
* Initializing the 8259 PIC
* Handling keyboard `IRQ1`
* Reading keyboard scancodes from port `0x60`
* Passing keyboard scancodes to the kernel
* Sending End Of Interrupt (EOI) signals to the PIC

---

## ⚡ Interrupt Architecture

AliOS now receives keyboard input through a hardware interrupt instead of continuously polling the keyboard port from the main kernel loop.

The current keyboard flow is:

```text
Physical Keyboard
       │
       ▼
    PS/2 Controller
       │
       ▼
      IRQ1
       │
       ▼
   PIC 8259
       │
       ▼
   Interrupt 33
       │
       ▼
      IDT
       │
       ▼
keyboard_handler()
       │
       ▼
   Port 0x60
       │
       ▼
Keyboard Scancode
       │
       ▼
Atomic Scancode Buffer
       │
       ▼
    main.rs
       │
       ▼
Scancode → Character
       │
       ▼
Terminal / Shell
```

### Keyboard Interrupt

The keyboard is connected to **IRQ1**.

AliOS remaps the master PIC so that:

```text
IRQ0 → Interrupt 32
IRQ1 → Interrupt 33
IRQ2 → Interrupt 34
...
```

Therefore the keyboard interrupt is handled through:

```text
IDT[33]
```

The keyboard handler reads the scancode directly from:

```text
Port 0x60
```

After processing the interrupt, AliOS sends an **EOI (End Of Interrupt)** signal to the PIC so that subsequent hardware interrupts can continue.

---

## ⌨️ Keyboard

AliOS currently supports keyboard input through the traditional PS/2 controller.

Supported input includes:

* `A-Z`
* `0-9`
* Space
* `-`
* `=`
* Enter
* Backspace

Keyboard scancodes are received by the interrupt handler and then converted into characters by the kernel.

Currently, the keyboard scancode buffer uses a single-byte atomic buffer. This is intentionally simple for the early version of AliOS and can later be replaced with a proper keyboard ring buffer.

---

## 🖥️ Terminal

AliOS provides a simple framebuffer-based terminal.

The terminal currently supports:

* Command input
* Cursor positioning
* Cursor blinking
* Backspace
* Enter
* New lines
* Automatic scrolling
* Screen clearing
* Command output

The shell uses a simple command buffer with a maximum length of **64 characters**.

Example:

```text
> ABOUT

ALIOS V0.1
A SIMPLE OPERATING SYSTEM
WRITTEN IN RUST.
```

---

## 🎨 Interface

AliOS currently uses a minimal dark interface with:

* Dark background
* Light text
* Blue accent color
* Centered boot title
* Accent line
* Terminal prompt
* Blinking cursor

Example:

```text
             WELCOME TO ALIOS!
             ──────────────────────────

> HELP

AVAILABLE COMMANDS:
CLEAR - CLEAR THE SCREEN
HELP - SHOW AVAILABLE COMMANDS
ABOUT - ABOUT ALIOS
```

---

## 🚀 Running AliOS

### Requirements

Make sure the following are installed:

* Rust
* Cargo
* QEMU
* Required Rust nightly toolchain
* Required bootloader dependencies

Check Rust:

```bash
rustc --version
cargo --version
```

Check QEMU:

```bash
qemu-system-x86_64 --version
```

---

## 🔨 Build

From the project root:

```bash
cargo build
```

For checking the kernel without producing the final boot image:

```bash
cargo check
```

A successful build should finish with output similar to:

```text
Finished `dev` profile
```

---

## 🧪 Development

AliOS is an experimental operating system project built primarily for learning.

The goal is not to create a production-ready operating system, but to understand how an operating system interacts with hardware and manages its own execution environment.

Development currently focuses on building the kernel from the lowest level upward:

```text
Boot
 ↓
Framebuffer
 ↓
Terminal
 ↓
Keyboard
 ↓
Interrupts
 ↓
Shell
 ↓
Kernel subsystems
```

---

## 🗺️ Roadmap

### v0.1 — Basic Kernel

* [x] Boot kernel
* [x] Framebuffer
* [x] Bitmap font
* [x] Text rendering
* [x] Terminal
* [x] Keyboard input
* [x] PS/2 keyboard support
* [x] Command buffer
* [x] `HELP`
* [x] `CLEAR`
* [x] `ABOUT`
* [x] Cursor
* [x] Cursor blinking
* [x] Terminal scrolling
* [x] Interrupt Descriptor Table
* [x] 8259 PIC initialization
* [x] Hardware interrupt support
* [x] Keyboard `IRQ1`
* [x] Keyboard interrupt handler

### v0.2 — Better Shell

* [ ] More commands
* [ ] Command history
* [ ] Arrow key support
* [ ] Better backspace handling
* [ ] Lowercase input
* [ ] Tab completion
* [ ] Shell error handling
* [ ] Command parser
* [ ] Better keyboard scancode handling
* [ ] Keyboard ring buffer

### v0.3 — Kernel Improvements

* [ ] Timer interrupt
* [ ] Programmable Interval Timer (PIT)
* [ ] Keyboard modifier support
* [ ] Memory management
* [ ] Heap allocator
* [ ] Global allocator
* [ ] Better panic handling
* [ ] CPU exception reporting

### Future

* [ ] Physical memory manager
* [ ] Virtual memory
* [ ] Process management
* [ ] User mode
* [ ] System calls
* [ ] Filesystem
* [ ] Applications
* [ ] More complete shell
* [ ] Basic multitasking

---

## 📚 Learning Goals

This project is mainly built to learn:

* Rust
* `no_std` programming
* Low-level memory management
* x86_64 architecture
* CPU exceptions
* Hardware interrupts
* Interrupt Descriptor Tables
* PIC 8259
* PS/2 keyboard controllers
* Port I/O
* Framebuffer graphics
* Terminal implementation
* Kernel architecture
* Operating system fundamentals

---

## ⚠️ Disclaimer

AliOS is an educational and experimental project.

It is not intended to replace a real operating system and should not be considered production-ready.

The project is actively developed and its architecture may change significantly between versions.

---

## 👨‍💻 Author

**Ali Fahmi Yusuf**

Built as a personal learning project to explore **Rust, low-level programming, and Operating System development**.

---

## 📄 License

This project is currently intended for educational purposes.
