use x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};

// ============================================================
// INTERRUPT DESCRIPTOR TABLE
// ============================================================
//
// IDT disimpan secara statis agar tetap hidup selama kernel
// berjalan.
//
// Kita menggunakan MaybeUninit karena IDT harus dibuat
// sebelum CPU menggunakannya.
//

use core::mem::MaybeUninit;

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
    // SIMPAN IDT
    // --------------------------------------------------------

    unsafe {
        IDT.write(idt);

        IDT.assume_init_ref().load();
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