// ============================================================
// TERMINAL
// ============================================================

use crate::framebuffer::FrameBuffer;

// ============================================================
// TERMINAL STRUCT
// ============================================================

pub struct Terminal<'a, 'b> {
    screen: &'a mut FrameBuffer<'b>,

    start_x: usize,
    start_y: usize,

    y: usize,

    line_height: usize,
    screen_height: usize,

    bg_r: u8,
    bg_g: u8,
    bg_b: u8,
}

// ============================================================
// IMPLEMENTATION
// ============================================================

impl<'a, 'b> Terminal<'a, 'b> {

    // --------------------------------------------------------
    // NEW
    // --------------------------------------------------------

    pub fn new(
        screen: &'a mut FrameBuffer<'b>,
        start_x: usize,
        start_y: usize,
        line_height: usize,
        screen_height: usize,
        bg_r: u8,
        bg_g: u8,
        bg_b: u8,
    ) -> Self {
        Self {
            screen,
            start_x,
            start_y,
            y: start_y,
            line_height,
            screen_height,
            bg_r,
            bg_g,
            bg_b,
        }
    }

    // --------------------------------------------------------
    // SCREEN
    // --------------------------------------------------------

    pub fn screen(&mut self) -> &mut FrameBuffer<'b> {
        self.screen
    }

    // --------------------------------------------------------
    // Y
    // --------------------------------------------------------

    pub fn y(&self) -> usize {
        self.y
    }

    // --------------------------------------------------------
    // NEXT LINE
    // --------------------------------------------------------

    pub fn next_line(&mut self) {
        if self.line_height == 0 {
            return;
        }

        let next_y = self.y.saturating_add(self.line_height);

        let bottom = next_y.saturating_add(self.line_height);

        if bottom > self.screen_height {
            self.scroll();
        } else {
            self.y = next_y;
        }
    }

    // --------------------------------------------------------
    // SCROLL
    // --------------------------------------------------------

    fn scroll(&mut self) {
        let region_height =
            self.screen_height.saturating_sub(self.start_y);

        if region_height == 0 {
            self.y = self.start_y;
            return;
        }

        let pixels = self.line_height.min(region_height);

        self.screen.scroll_region_up(
            self.start_y,
            self.screen_height,
            pixels,
            self.bg_r,
            self.bg_g,
            self.bg_b,
        );

        self.y = self
            .screen_height
            .saturating_sub(self.line_height);

        if self.y < self.start_y {
            self.y = self.start_y;
        }
    }

    // --------------------------------------------------------
    // CLEAR
    // --------------------------------------------------------

    pub fn clear(&mut self) {
        let height =
            self.screen_height.saturating_sub(self.start_y);

        if height == 0 {
            self.y = self.start_y;
            return;
        }

        // Bersihkan seluruh area terminal.
        //
        // Jangan menggunakan width = usize::MAX.
        // Itu bisa menyebabkan loop sangat lama.

        self.screen.scroll_region_up(
            self.start_y,
            self.screen_height,
            height,
            self.bg_r,
            self.bg_g,
            self.bg_b,
        );

        self.y = self.start_y;
    }
}