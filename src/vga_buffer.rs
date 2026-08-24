use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Aliases for the missing colors used in cosmic modules
impl Color {
    pub const BrightBlack: Color = Color::DarkGray;
    pub const BrightGreen: Color = Color::LightGreen;
    pub const BrightCyan: Color = Color::LightCyan;
    pub const BrightRed: Color = Color::LightRed;
    pub const BrightMagenta: Color = Color::Pink;
    pub const BrightWhite: Color = Color::White;
    pub const Grey: Color = Color::LightGray;
    pub const BrightYellow: Color = Color::Yellow;
    pub const BrightBlue: Color = Color::LightBlue;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Double-buffered compositor - all Deck rendering goes here then flushed once per frame
pub struct Compositor {
    front: &'static mut Buffer,
    back: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
    dirty: bool,
}

impl Compositor {
    pub fn new(front: &'static mut Buffer) -> Self {
        let blank = ScreenChar { ascii_character: b' ', color_code: ColorCode::new(Color::LightGray, Color::Black) };
        Self { front, back: [[blank; BUFFER_WIDTH]; BUFFER_HEIGHT], dirty: true }
    }
    pub fn put(&mut self, x: usize, y: usize, c: u8, color: ColorCode) {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            self.back[y][x] = ScreenChar { ascii_character: c, color_code: color };
            self.dirty = true;
        }
    }
    pub fn put_str(&mut self, mut x: usize, y: usize, s: &str, fg: Color, bg: Color) {
        let cc = ColorCode::new(fg, bg);
        for b in s.bytes() {
            if x >= BUFFER_WIDTH { break; }
            let ch = if (0x20..=0x7e).contains(&b) { b } else { 0xfe };
            self.put(x, y, ch, cc);
            x += 1;
        }
    }
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, ch: u8, color: ColorCode) {
        for dy in 0..h { for dx in 0..w { self.put(x+dx, y+dy, ch, color); } }
    }
    pub fn clear(&mut self, color: ColorCode) {
        let blank = ScreenChar { ascii_character: b' ', color_code: color };
        for row in &mut self.back { for c in row.iter_mut() { *c = blank; } }
        self.dirty = true;
    }
    pub fn flush(&mut self) {
        if self.dirty {
            self.front.chars.copy_from_slice(&self.back);
            self.dirty = false;
        }
    }
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    // For set_position compat - cursor position
    cursor_x: usize,
    cursor_y: usize,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar { ascii_character: byte, color_code };
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar { ascii_character: b' ', color_code: self.color_code };
        for col in 0..BUFFER_WIDTH { self.buffer.chars[row][col] = blank; }
    }

    /// Compatibility shims for all 12 cosmic modules - they call these but they never existed
    pub fn set_position(&mut self, x: usize, y: usize) {
        self.cursor_x = x.min(BUFFER_WIDTH - 1);
        self.cursor_y = y.min(BUFFER_HEIGHT - 1);
        // also sync column_position for legacy println
        self.column_position = self.cursor_x;
    }

    pub fn write_char(&mut self, c: char, color: Color) {
        let cc = ColorCode::new(color, Color::Black);
        let b = if (c as u8) >= 0x20 && (c as u8) <= 0x7e { c as u8 } else { 0xfe };
        // Handle unicode box chars: map to CP437
        let mapped = match c {
            '╔' | '╗' | '╚' | '╝' | '║' | '═' | '╠' | '╣' | '╬' | '╦' | '╩' => b'+',
            '█' => 0xDB, '▓' => 0xB2, '▒' => 0xB1, '░' => 0xB0,
            '○' | '●' | '◉' | '◐' | '◑' | '◈' | '◇' | '◆' => b'*',
            '▶' | '•' | '·' | '∙' => b'.',
            '⚛' | '☀' | '✦' | '✶' | '◎' | '☾' | '⬡' | '△' | '⬢' | '❄' => b'*',
            _ => b,
        };
        if self.cursor_y < BUFFER_HEIGHT && self.cursor_x < BUFFER_WIDTH {
            self.buffer.chars[self.cursor_y][self.cursor_x] = ScreenChar { ascii_character: mapped, color_code: cc };
            self.cursor_x = (self.cursor_x + 1).min(BUFFER_WIDTH - 1);
        }
    }

    // Overload used as write_str(&str, Color) in all cosmic files
    pub fn write_str(&mut self, s: &str, color: Color) {
        let cc = ColorCode::new(color, Color::Black);
        for &b in s.as_bytes() {
            if self.cursor_y >= BUFFER_HEIGHT { break; }
            if self.cursor_x >= BUFFER_WIDTH {
                self.cursor_y = (self.cursor_y + 1).min(BUFFER_HEIGHT - 1);
                self.cursor_x = 0;
            }
            let ch = if b >= 0x20 && b <= 0x7e { b } else { b' ' };
            // map fancy unicode bytes inside &str (they are multi-byte) -> simplified
            self.buffer.chars[self.cursor_y][self.cursor_x] = ScreenChar { ascii_character: ch, color_code: cc };
            self.cursor_x += 1;
        }
    }

    pub fn draw_pulse(&mut self, text: &str, color: Color) {
        let color_code = ColorCode::new(Color::White, color);
        for col in 0..BUFFER_WIDTH {
            let character = if col < text.len() { text.as_bytes()[col] } else { b' ' };
            self.buffer.chars[0][col] = ScreenChar { ascii_character: character, color_code };
        }
    }

    pub fn clear_screen(&mut self) {
        let blank = ScreenChar { ascii_character: b' ', color_code: ColorCode::new(Color::LightGray, Color::Black) };
        for row in 0..BUFFER_HEIGHT { for col in 0..BUFFER_WIDTH { self.buffer.chars[row][col] = blank; } }
        self.column_position = 0;
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result { self.write_string(s); Ok(()) }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        cursor_x: 0,
        cursor_y: 0,
    });
}

lazy_static! {
    pub static ref COMPOSITOR: Mutex<Compositor> = Mutex::new(Compositor::new(unsafe { &mut *(0xb8000 as *mut Buffer) }));
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
    WRITER.lock().write_fmt(args).unwrap();
}

pub fn init_status_bar() {
    WRITER.lock().draw_pulse(" NIMBUS OS | CYBERDECK ONLINE | [TAB] Deck [1-4] View ", Color::Blue);
}

pub fn clear() { WRITER.lock().clear_screen(); }
