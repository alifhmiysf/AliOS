use core::mem::MaybeUninit;
use core::sync::atomic::{
    AtomicU8,
    Ordering,
};

use pic8259::ChainedPics;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};

// ============================================================
// KEYBOARD SCANCODE BUFFER
// ============================================================
//
// Scancode yang diterima oleh interrupt keyboard
// disimpan di sini.
//
// 0 berarti belum ada scancode.
//
// Untuk AliOS v0.1 kita menggunakan satu-byte buffer
// terlebih dahulu.
//
// Nanti bisa dikembangkan menjadi ring buffer.
// ============================================================

static KEYBOARD_SCANCODE: AtomicU8 =
    AtomicU8::new(0);

// ============================================================
// PIC
// ============================================================
//
// PIC pertama:
// IRQ 0..7 -> interrupt vector 32..39
//
// PIC kedua:
// IRQ 8..15 -> interrupt vector 40..47
//
// Keyboard:
// IRQ1 -> interrupt vector 33
// ============================================================

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = 40;

// ============================================================
// PIC INSTANCE
// ============================================================
//
// PIC dibuat sekali dan tetap hidup selama kernel berjalan.
// ============================================================

static mut PICS: ChainedPics = unsafe {
    ChainedPics::new(
        PIC_1_OFFSET,
        PIC_2_OFFSET,
    )
};

// ============================================================
// INTERRUPT DESCRIPTOR TABLE
// ============================================================
//
// IDT disimpan secara statis agar tetap hidup selama
// kernel berjalan.
//
// MaybeUninit digunakan karena IDT baru diinisialisasi
// ketika init_idt() dipanggil.
// ============================================================

static mut IDT: MaybeUninit<InterruptDescriptorTable> =
    MaybeUninit::uninit();

// ============================================================
// INITIALIZE IDT
// ============================================================

pub fn init_idt() {
    let mut idt =
        InterruptDescriptorTable::new();

    // --------------------------------------------------------
    // CPU EXCEPTIONS
    // --------------------------------------------------------

    idt.breakpoint
        .set_handler_fn(
            breakpoint_handler
        );

    idt.double_fault
        .set_handler_fn(
            double_fault_handler
        );

    // --------------------------------------------------------
    // HARDWARE IRQ
    // --------------------------------------------------------
    //
    // Keyboard:
    //
    // IRQ1
    //   ↓
    // PIC offset 32
    //   ↓
    // vector 33
    //   ↓
    // IDT[33]
    //
    // --------------------------------------------------------

    idt[
        InterruptIndex::Keyboard.as_usize()
    ]
        .set_handler_fn(
            keyboard_handler
        );

    // --------------------------------------------------------
    // SIMPAN DAN LOAD IDT
    // --------------------------------------------------------

    unsafe {
        IDT.write(idt);

        IDT
            .assume_init_ref()
            .load();
    }
}

// ============================================================
// INTERRUPT INDEX
// ============================================================

#[repr(u8)]
#[derive(Clone, Copy)]
enum InterruptIndex {

    // --------------------------------------------------------
    // Keyboard IRQ1
    // --------------------------------------------------------
    //
    // PIC_1_OFFSET = 32
    //
    // IRQ1 = 1
    //
    // 32 + 1 = 33
    //
    // --------------------------------------------------------

    Keyboard =
        PIC_1_OFFSET + 1,
}

// ============================================================
// INTERRUPT INDEX HELPER
// ============================================================

impl InterruptIndex {

    // --------------------------------------------------------
    // Convert ke u8
    // --------------------------------------------------------

    fn as_u8(self) -> u8 {
        self as u8
    }

    // --------------------------------------------------------
    // Convert ke usize
    //
    // Dibutuhkan ketika mengakses:
    //
    // idt[33]
    // --------------------------------------------------------

    fn as_usize(self) -> usize {
        self.as_u8() as usize
    }
}

// ============================================================
// INITIALIZE PIC
// ============================================================
//
// PIC harus diinisialisasi sebelum hardware interrupt
// digunakan oleh CPU.
// ============================================================

pub fn init_pics() {

    unsafe {
        PICS.initialize();
    }
}

// ============================================================
// TAKE KEYBOARD SCANCODE
// ============================================================
//
// Fungsi ini dipanggil oleh main loop.
//
// Jika ada scancode:
//
//     Some(scancode)
//
// Jika belum ada:
//
//     None
//
// AtomicU8 digunakan karena scancode ditulis oleh
// interrupt handler dan dibaca oleh main loop.
// ============================================================

pub fn take_scancode() -> Option<u8> {

    let scancode =
        KEYBOARD_SCANCODE.swap(
            0,
            Ordering::Acquire,
        );

    if scancode == 0 {
        None
    } else {
        Some(scancode)
    }
}

// ============================================================
// BREAKPOINT EXCEPTION
// ============================================================

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame,
) {
    let _ = stack_frame;
}

// ============================================================
// DOUBLE FAULT
// ============================================================

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {

    let _ = stack_frame;

    loop {
        core::hint::spin_loop();
    }
}

// ============================================================
// KEYBOARD INTERRUPT
// ============================================================
//
// Keyboard menggunakan IRQ1.
//
// Alur:
//
// Keyboard
//     ↓
// IRQ1
//     ↓
// PIC
//     ↓
// IDT[33]
//     ↓
// keyboard_handler()
//     ↓
// port 0x60
//     ↓
// KEYBOARD_SCANCODE
//     ↓
// EOI
//
// ============================================================

extern "x86-interrupt" fn keyboard_handler(
    _stack_frame: InterruptStackFrame,
) {

    // --------------------------------------------------------
    // Baca scancode dari keyboard
    // --------------------------------------------------------

    let mut port =
        Port::new(0x60);

    let scancode: u8 = unsafe {
        port.read()
    };

    // --------------------------------------------------------
    // Simpan scancode
    // --------------------------------------------------------
    //
    // Main loop nantinya akan mengambil scancode
    // menggunakan:
    //
    // interrupts::take_scancode()
    //
    // --------------------------------------------------------

    KEYBOARD_SCANCODE.store(
        scancode,
        Ordering::Release,
    );

    // --------------------------------------------------------
    // Beritahu PIC bahwa interrupt sudah selesai
    // --------------------------------------------------------
    //
    // EOI = End Of Interrupt
    //
    // Tanpa EOI, PIC dapat berhenti meneruskan
    // interrupt berikutnya.
    // --------------------------------------------------------

    unsafe {
        PICS.notify_end_of_interrupt(
            InterruptIndex::Keyboard.as_u8()
        );
    }
}