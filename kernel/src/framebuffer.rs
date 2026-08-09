// ============================================================
// FRAMEBUFFER
// ============================================================
//
// Framebuffer adalah area memory yang mewakili layar.
//
// Setiap pixel di layar sebenarnya adalah beberapa byte
// di dalam buffer ini.
//

pub struct FrameBuffer<'a> {
    // Memory yang digunakan untuk menggambar.
    buffer: &'a mut [u8],

    // Lebar layar dalam pixel.
    width: usize,

    // Tinggi layar dalam pixel.
    height: usize,

    // Panjang satu baris framebuffer.
    stride: usize,

    // Jumlah byte untuk satu pixel.
    bytes_per_pixel: usize,
}

// ============================================================
// IMPLEMENTASI FRAMEBUFFER
// ============================================================

impl<'a> FrameBuffer<'a> {
    // --------------------------------------------------------
    // CONSTRUCTOR
    // --------------------------------------------------------

    pub fn new(
        buffer: &'a mut [u8],
        width: usize,
        height: usize,
        stride: usize,
        bytes_per_pixel: usize,
    ) -> Self {
        Self {
            buffer,
            width,
            height,
            stride,
            bytes_per_pixel,
        }
    }

    // --------------------------------------------------------
    // CLEAR SCREEN
    // --------------------------------------------------------

    pub fn clear(&mut self, r: u8, g: u8, b: u8) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, r, g, b);
            }
        }
    }

    // --------------------------------------------------------
    // PUT PIXEL
    // --------------------------------------------------------

    pub fn put_pixel(
        &mut self,
        x: usize,
        y: usize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index =
            (y * self.stride + x)
                * self.bytes_per_pixel;

        if index + 2 >= self.buffer.len() {
            return;
        }

        // Framebuffer menggunakan format BGR.
        self.buffer[index] = b;
        self.buffer[index + 1] = g;
        self.buffer[index + 2] = r;

        // Alpha channel jika 4 byte per pixel.
        if self.bytes_per_pixel == 4
            && index + 3 < self.buffer.len()
        {
            self.buffer[index + 3] = 255;
        }
    }

    // --------------------------------------------------------
    // PUT PIXEL SIGNED
    // --------------------------------------------------------

    fn put_pixel_signed(
        &mut self,
        x: isize,
        y: isize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        if x < 0 || y < 0 {
            return;
        }

        self.put_pixel(
            x as usize,
            y as usize,
            r,
            g,
            b,
        );
    }

    // --------------------------------------------------------
    // FILL RECTANGLE
    // --------------------------------------------------------

    pub fn fill_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        if width == 0 || height == 0 {
            return;
        }

        let max_x =
            x.saturating_add(width);

        let max_y =
            y.saturating_add(height);

        for py in y..max_y {
            for px in x..max_x {
                self.put_pixel(
                    px,
                    py,
                    r,
                    g,
                    b,
                );
            }
        }
    }

    // --------------------------------------------------------
    // DRAW RECTANGLE
    // --------------------------------------------------------

    pub fn draw_rect(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        if width == 0 || height == 0 {
            return;
        }

        let right =
            x.saturating_add(width)
                .saturating_sub(1);

        let bottom =
            y.saturating_add(height)
                .saturating_sub(1);

        for px in x..=right {
            self.put_pixel(
                px,
                y,
                r,
                g,
                b,
            );

            self.put_pixel(
                px,
                bottom,
                r,
                g,
                b,
            );
        }

        for py in y..=bottom {
            self.put_pixel(
                x,
                py,
                r,
                g,
                b,
            );

            self.put_pixel(
                right,
                py,
                r,
                g,
                b,
            );
        }
    }

    // --------------------------------------------------------
    // DRAW LINE
    // --------------------------------------------------------

    pub fn draw_line(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        let mut x0 =
            x1 as isize;

        let mut y0 =
            y1 as isize;

        let x_end =
            x2 as isize;

        let y_end =
            y2 as isize;

        let dx =
            (x_end - x0).abs();

        let dy =
            -(y_end - y0).abs();

        let sx =
            if x0 < x_end {
                1
            } else {
                -1
            };

        let sy =
            if y0 < y_end {
                1
            } else {
                -1
            };

        let mut error =
            dx + dy;

        loop {
            self.put_pixel_signed(
                x0,
                y0,
                r,
                g,
                b,
            );

            if x0 == x_end
                && y0 == y_end
            {
                break;
            }

            let error2 =
                2 * error;

            if error2 >= dy {
                error += dy;
                x0 += sx;
            }

            if error2 <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }

    // --------------------------------------------------------
    // DRAW CIRCLE
    // --------------------------------------------------------

    pub fn draw_circle(
        &mut self,
        center_x: isize,
        center_y: isize,
        radius: isize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        if radius < 0 {
            return;
        }

        let mut x =
            radius;

        let mut y =
            0;

        let mut decision =
            1 - radius;

        while x >= y {
            self.put_pixel_signed(
                center_x + x,
                center_y + y,
                r,
                g,
                b,
            );

            self.put_pixel_signed(
                center_x + y,
                center_y + x,
                r,
                g,
                b,
            );

            self.put_pixel_signed(
                center_x - y,
                center_y + x,
                r,
                g,
                b,
            );

            self.put_pixel_signed(
                center_x - x,
                center_y + y,
                r,
                g,
                b,
            );

            self.put_pixel_signed(
                center_x - x,
                center_y - y,
                r,
                g,
                b,
            );

            self.put_pixel_signed(
                center_x - y,
                center_y - x,
                r,
                g,
                b,
            );

            self.put_pixel_signed(
                center_x + y,
                center_y - x,
                r,
                g,
                b,
            );

            self.put_pixel_signed(
                center_x + x,
                center_y - y,
                r,
                g,
                b,
            );

            y += 1;

            if decision <= 0 {
                decision +=
                    2 * y + 1;
            } else {
                x -= 1;

                decision +=
                    2 * (y - x) + 1;
            }
        }
    }

    // --------------------------------------------------------
    // DRAW CHAR
    // --------------------------------------------------------

    pub fn draw_char(
        &mut self,
        x: usize,
        y: usize,
        c: char,
        scale: usize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        if scale == 0 {
            return;
        }

        let Some(character) =
            crate::font::get_char(c)
        else {
            return;
        };

        for (row, bits) in
            character.iter().enumerate()
        {
            for col in 0..5 {
                let pixel =
                    (bits >> (4 - col)) & 1;

                if pixel == 1 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            self.put_pixel(
                                x + col * scale + sx,
                                y + row * scale + sy,
                                r,
                                g,
                                b,
                            );
                        }
                    }
                }
            }
        }
    }

    // --------------------------------------------------------
    // DRAW TEXT
    // --------------------------------------------------------

    pub fn draw_text(
        &mut self,
        x: usize,
        y: usize,
        text: &str,
        scale: usize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        let char_width =
            (5 + 1) * scale;

        let mut cursor_x =
            x;

        for c in text.chars() {
            self.draw_char(
                cursor_x,
                y,
                c,
                scale,
                r,
                g,
                b,
            );

            cursor_x +=
                char_width;
        }
    }

    // --------------------------------------------------------
    // SCROLL UP SELURUH FRAMEBUFFER
    // --------------------------------------------------------
    //
    // Method ini menggeser seluruh layar ke atas.
    //
    // Untuk terminal AliOS kita lebih banyak menggunakan
    // scroll_region_up(), supaya TITLE tetap berada di atas.
    //

    pub fn scroll_up(
        &mut self,
        pixels: usize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        if pixels == 0 {
            return;
        }

        if pixels >= self.height {
            self.clear(r, g, b);
            return;
        }

        let row_bytes =
            self.stride
                * self.bytes_per_pixel;

        let rows_to_move =
            self.height - pixels;

        let move_bytes =
            rows_to_move
                * row_bytes;

        let source_start =
            pixels * row_bytes;

        self.buffer.copy_within(
            source_start
                ..source_start + move_bytes,
            0,
        );

        let clear_start =
            move_bytes;

        let clear_end =
            self.height * row_bytes;

        for byte in
            &mut self.buffer[
                clear_start..clear_end
            ]
        {
            *byte = 0;
        }

        for y in
            rows_to_move..self.height
        {
            for x in 0..self.width {
                self.put_pixel(
                    x,
                    y,
                    r,
                    g,
                    b,
                );
            }
        }
    }

    // --------------------------------------------------------
    // SCROLL REGION UP
    // --------------------------------------------------------
    //
    // Menggeser hanya area tertentu.
    //
    // Ini yang digunakan Terminal AliOS.
    //
    // Contoh:
    //
    // TITLE
    // ----------------
    // LINE 1
    // LINE 2
    // LINE 3
    // LINE 4
    //
    // Setelah scroll:
    //
    // TITLE
    // ----------------
    // LINE 2
    // LINE 3
    // LINE 4
    // LINE 5
    //
    // TITLE tidak ikut bergerak.
    //

    pub fn scroll_region_up(
        &mut self,
        top: usize,
        bottom: usize,
        pixels: usize,
        r: u8,
        g: u8,
        b: u8,
    ) {
        // -----------------------------------------------
        // VALIDASI
        // -----------------------------------------------

        if top >= bottom {
            return;
        }

        if top >= self.height {
            return;
        }

        // Jangan melewati tinggi framebuffer.
        let bottom =
            bottom.min(self.height);

        let region_height =
            bottom.saturating_sub(top);

        if region_height == 0 {
            return;
        }

        // -----------------------------------------------
        // SCROLL SEBESAR / LEBIH BESAR DARI AREA
        // -----------------------------------------------

        if pixels >= region_height {
            for y in top..bottom {
                for x in 0..self.width {
                    self.put_pixel(
                        x,
                        y,
                        r,
                        g,
                        b,
                    );
                }
            }

            return;
        }

        // -----------------------------------------------
        // HITUNG BYTE PER BARIS
        // -----------------------------------------------

        let row_bytes =
            self.stride
                * self.bytes_per_pixel;

        let source_start =
            (top + pixels)
                * row_bytes;

        let destination_start =
            top * row_bytes;

        let rows_to_move =
            region_height - pixels;

        let move_bytes =
            rows_to_move
                * row_bytes;

        // -----------------------------------------------
        // GESER DATA
        // -----------------------------------------------
        //
        // copy_within aman untuk area yang overlap.
        //

        self.buffer.copy_within(
            source_start
                ..source_start + move_bytes,
            destination_start,
        );

        // -----------------------------------------------
        // BERSIHKAN AREA BAWAH
        // -----------------------------------------------

        let clear_start =
            destination_start
                + move_bytes;

        let clear_end =
            bottom * row_bytes;

        for byte in
            &mut self.buffer[
                clear_start..clear_end
            ]
        {
            *byte = 0;
        }

        // -----------------------------------------------
        // WARNA BACKGROUND
        // -----------------------------------------------

        for y in
            (bottom - pixels)..bottom
        {
            for x in 0..self.width {
                self.put_pixel(
                    x,
                    y,
                    r,
                    g,
                    b,
                );
            }
        }
    }
}

// ============================================================
// END FRAMEBUFFER
// ============================================================
