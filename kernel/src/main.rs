
#![allow(static_mut_refs)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

// ============================================================
// MODULE
// ============================================================

mod framebuffer;
mod font;
mod terminal;
mod interrupts;

use bootloader_api::{
    entry_point,
    BootInfo,
};

use framebuffer::FrameBuffer;
use terminal::Terminal;

// Entry point kernel.
entry_point!(kernel_main);

// ============================================================
// WARNA TEMA
// ============================================================

const BG_R: u8 = 20;
const BG_G: u8 = 20;
const BG_B: u8 = 26;

const TEXT_R: u8 = 235;
const TEXT_G: u8 = 235;
const TEXT_B: u8 = 240;

const ACCENT_R: u8 = 120;
const ACCENT_G: u8 = 180;
const ACCENT_B: u8 = 230;

// ============================================================
// PORT I/O
// ============================================================

#[inline(always)]

// ============================================================
// KEYBOARD
// ============================================================


// ============================================================
// SCANCODE → CHAR
// ============================================================

fn scancode_to_char(
    sc: u8,
) -> Option<char> {
    match sc {

        // ----------------------------------------------------
        // ANGKA
        // ----------------------------------------------------

        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0A => Some('9'),
        0x0B => Some('0'),

        // ----------------------------------------------------
        // Q - P
        // ----------------------------------------------------

        0x10 => Some('Q'),
        0x11 => Some('W'),
        0x12 => Some('E'),
        0x13 => Some('R'),
        0x14 => Some('T'),
        0x15 => Some('Y'),
        0x16 => Some('U'),
        0x17 => Some('I'),
        0x18 => Some('O'),
        0x19 => Some('P'),

        // ----------------------------------------------------
        // A - L
        // ----------------------------------------------------

        0x1E => Some('A'),
        0x1F => Some('S'),
        0x20 => Some('D'),
        0x21 => Some('F'),
        0x22 => Some('G'),
        0x23 => Some('H'),
        0x24 => Some('J'),
        0x25 => Some('K'),
        0x26 => Some('L'),

        // ----------------------------------------------------
        // Z - M
        // ----------------------------------------------------

        0x2C => Some('Z'),
        0x2D => Some('X'),
        0x2E => Some('C'),
        0x2F => Some('V'),
        0x30 => Some('B'),
        0x31 => Some('N'),
        0x32 => Some('M'),

        // ----------------------------------------------------
        // KARAKTER
        // ----------------------------------------------------

        0x39 => Some(' '),
        0x0C => Some('-'),
        0x0D => Some('='),

        _ => None,
    }
}

// ============================================================
// SPECIAL SCANCODE
// ============================================================

const SC_ENTER: u8 = 0x1C;
const SC_BACKSPACE: u8 = 0x0E;

// ============================================================
// COMMAND
// ============================================================

const MAX_COMMAND_LENGTH: usize = 64;

// ============================================================
// COMMAND EQUALS
// ============================================================

fn command_equals(
    buffer: &[u8; MAX_COMMAND_LENGTH],
    length: usize,
    command: &[u8],
) -> bool {
    if length != command.len() {
        return false;
    }

    for i in 0..length {
        if buffer[i] != command[i] {
            return false;
        }
    }

    true
}

// ============================================================
// DRAW TEXT SPACED
// ============================================================

fn draw_text_spaced(
    screen: &mut FrameBuffer,
    x: usize,
    y: usize,
    text: &str,
    scale: usize,
    spacing: usize,
    r: u8,
    g: u8,
    b: u8,
) -> usize {
    let char_width =
        (5 * scale) + spacing;

    let mut cursor_x = x;

    for c in text.chars() {
        screen.draw_char(
            cursor_x,
            y,
            c,
            scale,
            r,
            g,
            b,
        );

        cursor_x += char_width;
    }

    if text.is_empty() {
        0
    } else {
        cursor_x
            .saturating_sub(x)
            .saturating_sub(spacing)
    }
}

// ============================================================
// DRAW PROMPT
// ============================================================

fn draw_prompt(
    screen: &mut FrameBuffer,
    x: usize,
    y: usize,
    scale: usize,
) {
    let thickness = scale;

    // --------------------------------------------------------
    // >
    // --------------------------------------------------------

    for i in 0..(4 * scale) {
        screen.fill_rect(
            x + i,
            y + i,
            thickness,
            thickness,
            ACCENT_R,
            ACCENT_G,
            ACCENT_B,
        );
    }

    for i in 0..(4 * scale) {
        screen.fill_rect(
            x + i,
            y + (8 * scale).saturating_sub(i),
            thickness,
            thickness,
            ACCENT_R,
            ACCENT_G,
            ACCENT_B,
        );
    }
}

// ============================================================
// DRAW OUTPUT
// ============================================================

fn draw_output(
    screen: &mut FrameBuffer,
    x: usize,
    y: usize,
    text: &str,
    scale: usize,
    spacing: usize,
) {
    draw_text_spaced(
        screen,
        x,
        y,
        text,
        scale,
        spacing,
        TEXT_R,
        TEXT_G,
        TEXT_B,
    );
}

// ============================================================
// TERMINAL DRAW HELPERS
// ============================================================
//
// Helper ini dibuat untuk menghindari:
//
// error[E0502]: cannot borrow `terminal` as immutable
// because it is also borrowed as mutable
//
// Kita mengambil posisi Y terlebih dahulu.
//
// ============================================================

fn terminal_draw_prompt(
    terminal: &mut Terminal,
    x: usize,
    scale: usize,
) {
    let y = terminal.y();

    draw_prompt(
        terminal.screen(),
        x,
        y,
        scale,
    );
}

// ============================================================

fn terminal_draw_output(
    terminal: &mut Terminal,
    x: usize,
    text: &str,
    scale: usize,
    spacing: usize,
) {
    let y = terminal.y();

    draw_output(
        terminal.screen(),
        x,
        y,
        text,
        scale,
        spacing,
    );
}

// ============================================================

fn terminal_fill_rect(
    terminal: &mut Terminal,
    x: usize,
    width: usize,
    height: usize,
    r: u8,
    g: u8,
    b: u8,
) {
    let y = terminal.y();

    terminal
        .screen()
        .fill_rect(
            x,
            y,
            width,
            height,
            r,
            g,
            b,
        );
}

// ============================================================

fn terminal_draw_char(
    terminal: &mut Terminal,
    x: usize,
    c: char,
    scale: usize,
    r: u8,
    g: u8,
    b: u8,
) {
    let y = terminal.y();

    terminal
        .screen()
        .draw_char(
            x,
            y,
            c,
            scale,
            r,
            g,
            b,
        );
}

// ============================================================
// KERNEL MAIN
// ============================================================

fn kernel_main(
    boot_info: &'static mut BootInfo,
) -> ! {

    if let Some(framebuffer) =
        boot_info.framebuffer.as_mut()
    {
        let info = framebuffer.info();

        // ====================================================
        // FRAMEBUFFER
        // ====================================================

        let mut screen =
            FrameBuffer::new(
                framebuffer.buffer_mut(),
                info.width,
                info.height,
                info.stride,
                info.bytes_per_pixel,
            );
            interrupts::init_idt();
            interrupts::init_pics();
            x86_64::instructions::interrupts::enable();
        // ====================================================
        // BACKGROUND
        // ====================================================

        screen.fill_rect(
            0,
            0,
            info.width,
            info.height,
            BG_R,
            BG_G,
            BG_B,
        );

        // ====================================================
        // TITLE
        // ====================================================

        let title =
            "WELCOME TO ALIOS!";

        let title_scale =
            4;

        let title_spacing =
            8;

        let title_char_width =
            (5 * title_scale)
                + title_spacing;

        let title_width =
            title.len()
                * title_char_width
                - title_spacing;

        let title_height =
            7 * title_scale;

        let title_x =
            info.width
                .saturating_sub(title_width)
                / 2;

        let title_y =
            info.height
                .saturating_sub(title_height)
                / 2;

        draw_text_spaced(
            &mut screen,
            title_x,
            title_y,
            title,
            title_scale,
            title_spacing,
            TEXT_R,
            TEXT_G,
            TEXT_B,
        );

        // ====================================================
        // ACCENT LINE
        // ====================================================

        let line_y =
            title_y
                + title_height
                + 24;

        screen.fill_rect(
            title_x,
            line_y,
            title_width,
            2,
            ACCENT_R,
            ACCENT_G,
            ACCENT_B,
        );

        // ====================================================
        // TERMINAL CONFIG
        // ====================================================

        let input_scale =
            2;

        let input_spacing =
            5;

        let input_char_width =
            (5 * input_scale)
                + input_spacing;

        let input_char_height =
            7 * input_scale;

        let input_line_height =
            input_char_height + 12;

        let input_x =
            title_x;

        let input_y_start =
            line_y + 50;

        // ====================================================
        // TERMINAL
        // ====================================================

        //
        // Terminal::new() versi sekarang
        // menerima 7 argument.
        //

        let mut terminal =
            Terminal::new(
                &mut screen,
                input_x,
                input_y_start,
                input_line_height,
                info.height,
                BG_R,
                BG_G,
                BG_B,
            );

        // ====================================================
        // PROMPT CONFIG
        // ====================================================

        let prompt_scale =
            2;

        let prompt_width =
            8 * prompt_scale;

        let prompt_gap =
            input_char_width;

        let command_x =
            input_x
                + prompt_width
                + prompt_gap;

        // ====================================================
        // CURSOR
        // ====================================================

        let cursor_width =
            2;

        let cursor_height =
            input_char_height;

        let mut cursor_x =
            command_x;

        let mut cursor_visible =
            true;

        let mut blink_timer: u32 =
            0;

        let blink_interval =
            150_000;

        // ====================================================
        // COMMAND BUFFER
        // ====================================================

        let mut command_buffer =
            [0u8; MAX_COMMAND_LENGTH];

        let mut command_length =
            0usize;

        // ====================================================
        // INITIAL PROMPT
        // ====================================================

        terminal_draw_prompt(
            &mut terminal,
            input_x,
            prompt_scale,
        );

        // ====================================================
        // INITIAL CURSOR
        // ====================================================

        terminal_fill_rect(
            &mut terminal,
            cursor_x,
            cursor_width,
            cursor_height,
            ACCENT_R,
            ACCENT_G,
            ACCENT_B,
        );

        // ====================================================
        // MAIN LOOP
        // ====================================================

        loop {

            // =================================================
            // KEYBOARD
            // =================================================

            if let Some(sc) =
    interrupts::take_scancode()
{
                // ---------------------------------------------
                // Abaikan key release.
                // ---------------------------------------------

                if sc & 0x80 == 0 {

                    // -----------------------------------------
                    // HAPUS CURSOR LAMA
                    // -----------------------------------------

                    terminal_fill_rect(
                        &mut terminal,
                        cursor_x,
                        cursor_width,
                        cursor_height,
                        BG_R,
                        BG_G,
                        BG_B,
                    );

                    // =========================================
                    // ENTER
                    // =========================================

                    if sc == SC_ENTER {

                        // -------------------------------------
                        // Cek command sebelum buffer dihapus.
                        // -------------------------------------

                        let is_clear =
                            command_equals(
                                &command_buffer,
                                command_length,
                                b"CLEAR",
                            );

                        let is_help =
                            command_equals(
                                &command_buffer,
                                command_length,
                                b"HELP",
                            );

                        let is_about =
                            command_equals(
                                &command_buffer,
                                command_length,
                                b"ABOUT",
                            );

                        // =====================================
                        // CLEAR
                        // =====================================

                        if is_clear {

                            terminal.clear();
                        }

                        // =====================================
                        // HELP
                        // =====================================

                        else if is_help {

                            terminal.next_line();

                            terminal_draw_output(
                                &mut terminal,
                                input_x,
                                "AVAILABLE COMMANDS:",
                                input_scale,
                                input_spacing,
                            );

                            terminal.next_line();

                            terminal_draw_output(
                                &mut terminal,
                                input_x,
                                "CLEAR - CLEAR THE SCREEN",
                                input_scale,
                                input_spacing,
                            );

                            terminal.next_line();

                            terminal_draw_output(
                                &mut terminal,
                                input_x,
                                "HELP - SHOW AVAILABLE COMMANDS",
                                input_scale,
                                input_spacing,
                            );

                            terminal.next_line();

                            terminal_draw_output(
                                &mut terminal,
                                input_x,
                                "ABOUT - ABOUT ALIOS",
                                input_scale,
                                input_spacing,
                            );
                        }

                        // =====================================
                        // ABOUT
                        // =====================================

                        else if is_about {

                            terminal.next_line();

                            terminal_draw_output(
                                &mut terminal,
                                input_x,
                                "ALIOS V0.1",
                                input_scale,
                                input_spacing,
                            );

                            terminal.next_line();

                            terminal_draw_output(
                                &mut terminal,
                                input_x,
                                "A SIMPLE OPERATING SYSTEM",
                                input_scale,
                                input_spacing,
                            );

                            terminal.next_line();

                            terminal_draw_output(
                                &mut terminal,
                                input_x,
                                "WRITTEN IN RUST.",
                                input_scale,
                                input_spacing,
                            );
                        }

                        // =====================================
                        // UNKNOWN COMMAND
                        // =====================================

                        else if command_length > 0 {

                            terminal.next_line();

                            terminal_draw_output(
                                &mut terminal,
                                input_x,
                                "UNKNOWN COMMAND:",
                                input_scale,
                                input_spacing,
                            );

                            if let Ok(command) =
                                core::str::from_utf8(
                                    &command_buffer[
                                        ..command_length
                                    ]
                                )
                            {
                                terminal.next_line();

                                terminal_draw_output(
                                    &mut terminal,
                                    input_x,
                                    command,
                                    input_scale,
                                    input_spacing,
                                );
                            }
                        }

                        // =====================================
                        // RESET COMMAND BUFFER
                        // =====================================

                        command_length =
                            0;

                        command_buffer.fill(0);

                        // =====================================
                        // BARIS COMMAND BERIKUTNYA
                        // =====================================

                        if !is_clear {
                            terminal.next_line();
                        }

                        // =====================================
                        // PROMPT BARU
                        // =====================================

                        terminal_draw_prompt(
                            &mut terminal,
                            input_x,
                            prompt_scale,
                        );

                        // -------------------------------------
                        // Cursor kembali ke posisi awal.
                        // -------------------------------------

                        cursor_x =
                            command_x;
                    }

                    // =========================================
                    // BACKSPACE
                    // =========================================

                    else if sc == SC_BACKSPACE {

                        if command_length > 0 {

                            // ---------------------------------
                            // Kurangi panjang command.
                            // ---------------------------------

                            command_length -= 1;

                            // ---------------------------------
                            // Hapus byte terakhir.
                            // ---------------------------------

                            command_buffer[
                                command_length
                            ] = 0;

                            // ---------------------------------
                            // Geser cursor ke kiri.
                            // ---------------------------------

                            cursor_x =
                                cursor_x
                                    .saturating_sub(
                                        input_char_width
                                    );

                            // ---------------------------------
                            // Hapus karakter dari layar.
                            // ---------------------------------

                            terminal_fill_rect(
                                &mut terminal,
                                cursor_x,
                                input_char_width,
                                input_char_height,
                                BG_R,
                                BG_G,
                                BG_B,
                            );
                        }
                    }

                    // =========================================
                    // CHARACTER
                    // =========================================

                    else if let Some(c) =
                        scancode_to_char(sc)
                    {
                        if command_length
                            < MAX_COMMAND_LENGTH
                        {
                            let fits =
                                cursor_x
                                    + input_char_width
                                    <= info.width;

                            if fits {

                                // -----------------------------
                                // Simpan karakter.
                                // -----------------------------

                                command_buffer[
                                    command_length
                                ] = c as u8;

                                command_length += 1;

                                // -----------------------------
                                // Gambar karakter.
                                // -----------------------------

                                terminal_draw_char(
                                    &mut terminal,
                                    cursor_x,
                                    c,
                                    input_scale,
                                    TEXT_R,
                                    TEXT_G,
                                    TEXT_B,
                                );

                                // -----------------------------
                                // Geser cursor.
                                // -----------------------------

                                cursor_x +=
                                    input_char_width;
                            }
                        }
                    }

                    // =========================================
                    // DRAW CURSOR
                    // =========================================

                    terminal_fill_rect(
                        &mut terminal,
                        cursor_x,
                        cursor_width,
                        cursor_height,
                        ACCENT_R,
                        ACCENT_G,
                        ACCENT_B,
                    );

                    cursor_visible =
                        true;

                    blink_timer =
                        0;
                }
            }

            // =================================================
            // CURSOR BLINK
            // =================================================

            blink_timer += 1;

            if blink_timer
                >= blink_interval
            {
                blink_timer =
                    0;

                cursor_visible =
                    !cursor_visible;

                let (
                    r,
                    g,
                    b,
                ) =
                    if cursor_visible {
                        (
                            ACCENT_R,
                            ACCENT_G,
                            ACCENT_B,
                        )
                    } else {
                        (
                            BG_R,
                            BG_G,
                            BG_B,
                        )
                    };

                terminal_fill_rect(
                    &mut terminal,
                    cursor_x,
                    cursor_width,
                    cursor_height,
                    r,
                    g,
                    b,
                );
            }

            // =================================================
            // CPU DELAY
            // =================================================

            for _ in 0..2_000 {
                core::hint::spin_loop();
            }
        }
    }

    // ========================================================
    // TIDAK ADA FRAMEBUFFER
    // ========================================================

    loop {}
}

// ============================================================
// PANIC HANDLER
// ============================================================

#[panic_handler]
fn panic(
    _info: &core::panic::PanicInfo,
) -> ! {
    loop {}
}

