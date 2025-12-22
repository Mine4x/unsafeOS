use vga::colors::{Color16, TextModeColor};
use vga::writers::{ScreenCharacter, TextWriter, Text80x25, Graphics640x480x16, GraphicsWriter};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

pub use vga::colors::Color16 as Color;

pub enum DisplayMode {
    Text(TextMode),
    Graphics(GraphicsMode),
}

pub struct TextMode {
    text_mode: Text80x25,
    color: TextModeColor,
    pub column_position: usize,
    row_position: usize,
}

pub struct GraphicsMode {
    graphics_mode: Graphics640x480x16,
    fg_color: Color16,
    bg_color: Color16,
    cursor_x: usize,
    cursor_y: usize,
}

impl TextMode {
    pub fn new() -> Self {
        let color = TextModeColor::new(Color16::White, Color16::Black);
        let text_mode = Text80x25::new();
        text_mode.set_mode();
        text_mode.clear_screen();
        
        TextMode {
            text_mode,
            color,
            column_position: 0,
            row_position: 0,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            }
            byte => {
                if self.column_position >= 80 {
                    self.new_line();
                }

                let screen_char = ScreenCharacter::new(byte, self.color);
                self.text_mode.write_character(self.column_position, self.row_position, screen_char);
                self.column_position += 1;
                self.update_hardware_cursor(self.row_position, self.column_position);
            }
        }
    }

    fn new_line(&mut self) {
        if self.row_position >= 24 {
            for row in 1..25 {
                for col in 0..80 {
                    let ch = self.read_character(col, row);
                    self.text_mode.write_character(col, row - 1, ch);
                }
            }
            self.clear_row(24);
            self.row_position = 24;
        } else {
            self.row_position += 1;
        }
        self.column_position = 0;
        self.update_hardware_cursor(self.row_position, self.column_position);
    }

    fn read_character(&self, col: usize, row: usize) -> ScreenCharacter {
        unsafe {
            let vga_buffer = 0xb8000 as *const u16;
            let offset = row * 80 + col;
            let value = *vga_buffer.add(offset);
            let ascii = (value & 0xFF) as u8;
            let color_code = (value >> 8) as u8;
            let fg = match color_code & 0x0F {
                0 => Color16::Black, 1 => Color16::Blue, 2 => Color16::Green, 3 => Color16::Cyan,
                4 => Color16::Red, 5 => Color16::Magenta, 6 => Color16::Brown, 7 => Color16::LightGrey,
                8 => Color16::DarkGrey, 9 => Color16::LightBlue, 10 => Color16::LightGreen, 11 => Color16::LightCyan,
                12 => Color16::LightRed, 13 => Color16::Pink, 14 => Color16::Yellow, _ => Color16::White,
            };
            let bg = match (color_code >> 4) & 0x0F {
                0 => Color16::Black, 1 => Color16::Blue, 2 => Color16::Green, 3 => Color16::Cyan,
                4 => Color16::Red, 5 => Color16::Magenta, 6 => Color16::Brown, 7 => Color16::LightGrey,
                8 => Color16::DarkGrey, 9 => Color16::LightBlue, 10 => Color16::LightGreen, 11 => Color16::LightCyan,
                12 => Color16::LightRed, 13 => Color16::Pink, 14 => Color16::Yellow, _ => Color16::White,
            };
            ScreenCharacter::new(ascii, TextModeColor::new(fg, bg))
        }
    }

    pub fn backspace(&mut self) {
        if self.column_position > 0 {
            self.column_position -= 1;
            let blank = ScreenCharacter::new(b' ', self.color);
            self.text_mode.write_character(self.column_position, self.row_position, blank);
            self.update_hardware_cursor(self.row_position, self.column_position);
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenCharacter::new(b' ', self.color);
        for col in 0..80 {
            self.text_mode.write_character(col, row, blank);
        }
    }

    pub fn clear_buffer(&mut self) {
        self.text_mode.clear_screen();
        self.column_position = 0;
        self.row_position = 0;
        self.update_hardware_cursor(self.row_position, self.column_position);
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn set_cursor_position(&mut self, row: usize, col: usize) {
        if row < 25 && col < 80 {
            self.row_position = row;
            self.column_position = col;
        }
    }

    pub fn move_cursor(&mut self, row: usize, col: usize) {
        self.set_cursor_position(row, col);
        self.update_hardware_cursor(row, col);
    }

    fn update_hardware_cursor(&self, row: usize, col: usize) {
        use x86_64::instructions::port::Port;
        
        let pos = row * 80 + col;
        
        unsafe {
            let mut cmd_port = Port::<u8>::new(0x3D4);
            let mut data_port = Port::<u8>::new(0x3D5);
            
            cmd_port.write(0x0F);
            data_port.write((pos & 0xFF) as u8);
            
            cmd_port.write(0x0E);
            data_port.write(((pos >> 8) & 0xFF) as u8);
        }
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.row_position, self.column_position)
    }

    pub fn get_hardware_cursor_position(&self) -> (usize, usize) {
        use x86_64::instructions::port::Port;
        
        unsafe {
            let mut cmd_port = Port::<u8>::new(0x3D4);
            let mut data_port = Port::<u8>::new(0x3D5);
            
            cmd_port.write(0x0F);
            let pos_low = data_port.read() as u16;
            
            cmd_port.write(0x0E);
            let pos_high = data_port.read() as u16;
            
            let pos = (pos_high << 8) | pos_low;
            let row = (pos as usize) / 80;
            let col = (pos as usize) % 80;
            
            (row, col)
        }
    }

    pub fn set_color(&mut self, foreground: Color16, background: Color16) {
        self.color = TextModeColor::new(foreground, background);
    }
}

impl GraphicsMode {
    pub fn new() -> Self {
        let graphics_mode = Graphics640x480x16::new();
        graphics_mode.set_mode();
        graphics_mode.clear_screen(Color16::Black);
        
        GraphicsMode {
            graphics_mode,
            fg_color: Color16::White,
            bg_color: Color16::Black,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn clear_screen(&self, color: Color16) {
        self.graphics_mode.clear_screen(color);
    }

    pub fn draw_character(&mut self, x: usize, y: usize, character: char, color: Color16) {
        self.graphics_mode.draw_character(x, y, character, color);
    }

    pub fn draw_line(&self, start: (usize, usize), end: (usize, usize), color: Color16) {
        // Bresenham's line algorithm
        let dx = (end.0 as isize - start.0 as isize).abs();
        let dy = (end.1 as isize - start.1 as isize).abs();
        let sx = if start.0 < end.0 { 1 } else { -1 };
        let sy = if start.1 < end.1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = start.0 as isize;
        let mut y = start.1 as isize;

        loop {
            self.graphics_mode.set_pixel(x as usize, y as usize, color);
            
            if x == end.0 as isize && y == end.1 as isize {
                break;
            }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn set_pixel(&self, x: usize, y: usize, color: Color16) {
        self.graphics_mode.set_pixel(x, y, color);
    }

    pub fn write_string(&mut self, s: &str) {
        for ch in s.chars() {
            if ch == '\n' {
                self.cursor_x = 0;
                self.cursor_y += 16;
            } else {
                self.draw_character(self.cursor_x, self.cursor_y, ch, self.fg_color);
                self.cursor_x += 8;
                if self.cursor_x >= 640 {
                    self.cursor_x = 0;
                    self.cursor_y += 16;
                }
            }
        }
    }

    pub fn set_color(&mut self, foreground: Color16, background: Color16) {
        self.fg_color = foreground;
        self.bg_color = background;
    }

    pub fn set_cursor_position(&mut self, x: usize, y: usize) {
        self.cursor_x = x;
        self.cursor_y = y;
    }
}

pub struct Writer {
    mode: DisplayMode,
}

impl Writer {
    pub fn new_text() -> Self {
        Writer {
            mode: DisplayMode::Text(TextMode::new()),
        }
    }

    pub fn new_graphics() -> Self {
        Writer {
            mode: DisplayMode::Graphics(GraphicsMode::new()),
        }
    }

    pub fn switch_to_text(&mut self) {
        self.mode = DisplayMode::Text(TextMode::new());
    }

    pub fn switch_to_graphics(&mut self) {
        self.mode = DisplayMode::Graphics(GraphicsMode::new());
    }

    pub fn write_byte(&mut self, byte: u8) {
        match &mut self.mode {
            DisplayMode::Text(text) => text.write_byte(byte),
            DisplayMode::Graphics(gfx) => {
                let binding = &[byte];
                let s = core::str::from_utf8(binding).unwrap_or("?");
                gfx.write_string(s);
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        match &mut self.mode {
            DisplayMode::Text(text) => text.write_string(s),
            DisplayMode::Graphics(gfx) => gfx.write_string(s),
        }
    }

    pub fn clear_buffer(&mut self) {
        match &mut self.mode {
            DisplayMode::Text(text) => text.clear_buffer(),
            DisplayMode::Graphics(gfx) => gfx.clear_screen(gfx.bg_color),
        }
    }

    pub fn backspace(&mut self) {
        match &mut self.mode {
            DisplayMode::Text(text) => text.backspace(),
            DisplayMode::Graphics(gfx) => {
                if gfx.cursor_x >= 8 {
                    gfx.cursor_x -= 8;
                    gfx.draw_character(gfx.cursor_x, gfx.cursor_y, ' ', gfx.bg_color);
                }
            }
        }
    }

    pub fn set_color(&mut self, foreground: Color16, background: Color16) {
        match &mut self.mode {
            DisplayMode::Text(text) => text.set_color(foreground, background),
            DisplayMode::Graphics(gfx) => gfx.set_color(foreground, background),
        }
    }

    pub fn get_graphics(&mut self) -> Option<&mut GraphicsMode> {
        match &mut self.mode {
            DisplayMode::Graphics(gfx) => Some(gfx),
            _ => None,
        }
    }

    pub fn get_text(&mut self) -> Option<&mut TextMode> {
        match &mut self.mode {
            DisplayMode::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn column_position(&self) -> usize {
        match &self.mode {
            DisplayMode::Text(text) => text.column_position,
            DisplayMode::Graphics(gfx) => gfx.cursor_x / 8,
        }
    }

    pub fn set_column_position(&mut self, col: usize) {
        match &mut self.mode {
            DisplayMode::Text(text) => {
                if col < 80 {
                    text.column_position = col;
                    text.update_hardware_cursor(text.row_position, col);
                }
            }
            DisplayMode::Graphics(gfx) => {
                gfx.cursor_x = col * 8;
            }
        }
    }

    pub fn move_cursor(&mut self, row: usize, col: usize) {
        match &mut self.mode {
            DisplayMode::Text(text) => text.move_cursor(row, col),
            DisplayMode::Graphics(gfx) => gfx.set_cursor_position(col * 8, row * 16),
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| { 
        WRITER.lock().write_fmt(args).unwrap();
    });
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new_text());
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
    });
}