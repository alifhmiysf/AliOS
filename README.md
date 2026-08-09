# AliOS v0.1

A minimal experimental operating system kernel written in **Rust**.

AliOS is a learning project created to explore low-level programming, operating system concepts, framebuffer graphics, keyboard input, and terminal implementation using Rust.

> **Status:** 🚧 AliOS v0.1 — Experimental / In Development

---

## 🖥️ Features

Current features:

* Bootable x86_64 kernel
* Custom framebuffer rendering
* Custom bitmap font
* Basic terminal interface
* Keyboard input through PS/2 port
* Command buffer
* Command execution
* Cursor rendering
* Cursor blinking
* Terminal line management
* Automatic terminal scrolling
* Terminal clear
* Basic commands

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
* x86_64
* Framebuffer
* PS/2 Keyboard
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
│   │   └── font.rs
│   │
│   └── Cargo.toml
│
├── Cargo.toml
├── Cargo.lock
└── README.md
```

### `main.rs`

Kernel entry point and main operating system loop.

Responsible for:

* Kernel initialization
* Keyboard input
* Scancode processing
* Command processing
* Terminal interaction
* Cursor handling

### `framebuffer.rs`

Low-level graphics abstraction.

Responsible for:

* Drawing pixels
* Drawing rectangles
* Drawing characters
* Drawing text
* Scrolling framebuffer regions
* Clearing screen areas

### `font.rs`

Contains the bitmap font used by AliOS for rendering text.

### `terminal.rs`

Manages the terminal state.

Responsible for:

* Current cursor line
* Line movement
* Terminal scrolling
* Terminal clearing
* Terminal boundaries

---

## ⌨️ Keyboard

AliOS currently reads keyboard input directly through the traditional PS/2 controller ports.

Keyboard scancodes are converted into characters inside the kernel.

Currently supported:

* `A-Z`
* `0-9`
* Space
* `-`
* `=`
* Enter
* Backspace

---

## 🎨 Interface

AliOS currently uses a minimal dark interface with:

* Dark background
* Light text
* Blue accent color
* Boot title
* Terminal prompt
* Blinking cursor

Example:

```text
WELCOME TO ALIOS!
────────────────────────────

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
* Required Rust target/toolchain for the bootloader configuration

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

A successful build should look similar to:

```text
Compiling alios-kernel v0.1.0
Compiling alios v0.1.0
Finished `dev` profile
```

---

## 🧪 Development

AliOS is currently an experimental operating system project.

The goal is not to build a production-ready OS, but to understand how operating systems work from the lowest level.

Development will gradually move from a simple framebuffer terminal toward a more complete kernel architecture.

---

## 🗺️ Roadmap

### v0.1 — Basic Kernel

* [x] Boot kernel
* [x] Framebuffer
* [x] Bitmap font
* [x] Text rendering
* [x] Keyboard input
* [x] Terminal
* [x] Command buffer
* [x] `HELP`
* [x] `CLEAR`
* [x] `ABOUT`
* [x] Cursor
* [x] Cursor blinking
* [x] Terminal scrolling

### v0.2 — Better Shell

* [ ] More commands
* [ ] Command history
* [ ] Arrow key support
* [ ] Better backspace handling
* [ ] Lowercase input
* [ ] Tab completion
* [ ] Shell error handling

### v0.3 — Kernel Improvements

* [ ] Interrupt Descriptor Table
* [ ] Hardware interrupts
* [ ] Proper keyboard interrupt
* [ ] Timer interrupt
* [ ] Better memory management
* [ ] Heap allocator

### Future

* [ ] Process management
* [ ] Virtual memory
* [ ] Filesystem
* [ ] User mode
* [ ] System calls
* [ ] Applications
* [ ] More complete shell

---

## 📚 Learning Goals

This project is mainly built to learn:

* Rust
* `no_std` programming
* Memory management
* Hardware interaction
* CPU architecture
* Framebuffers
* Keyboard controllers
* Interrupts
* Kernel architecture
* Operating system fundamentals

---

## ⚠️ Disclaimer

AliOS is an educational and experimental project.

It is not intended to replace a real operating system and should not be considered production-ready.

---

## 👨‍💻 Author

**Ali Fahmi Yusuf**

Built as a personal learning project to explore **Rust and Operating System development**.

---

## 📄 License

This project is currently intended for educational purposes.
