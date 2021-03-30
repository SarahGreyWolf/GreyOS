use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;
use core::fmt::{Arguments, write, Write};
use crate::{print, println};
use crate::instructions::port::{PortWriteOnly, Port};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

lazy_static! {
    // Lazy Mutex, instead of blocking,
    // threads will repeatedly attempt to lock it till it's possible
    pub static ref VGA_WRITER: Mutex<Writer> = Mutex::new(Writer::init());
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black=0x0,
    Blue=0x1,
    Green=0x2,
    Cyan=0x3,
    Red=0x4,
    Magenta=0x5,
    Brown=0x6,
    LightGrey=0x7,
    Grey=0x8,
    LightBlue=0x9,
    LightGreen=0xA,
    LightCyan=0xB,
    LightRed=0xC,
    LightMagenta=0xD,
    Yellow=0xE,
    White=0xF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    colour_code: u8,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; WIDTH]; HEIGHT]
}

pub struct Writer {
    pub cursor_pos: usize,
    buffer: &'static mut Buffer,
    foreground: [Colour; WIDTH*HEIGHT],
    background: [Colour; WIDTH*HEIGHT],
}

impl Writer {
    pub fn init() -> Self {
        // FIXME: Actually make the cursor visible and update with writing
        unsafe {
            use crate::instructions::port::Port;
            let mut port = Port::new(0x3D4);
            port.write(0x0A as u8);
            let mut port = Port::new(0x3D5);
            port.write(0x20 as u8);
        }
        Self {
            cursor_pos: 0,
            // Get a mutable reference to a pointer that is a mutable pointer ????? Madness
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            foreground: [Colour::Black; WIDTH*HEIGHT],
            background: [Colour::Black; WIDTH*HEIGHT],
        }
    }

    pub fn write_byte(&mut self, character_byte: &u8, background: Colour, foreground: Colour) {
        if self.cursor_pos >= WIDTH * HEIGHT {
            self.shift_line_up();
        }
        let mut row = self.cursor_pos / WIDTH;
        let col = (self.cursor_pos - (row * WIDTH));
        match character_byte {
            b'\n' => {
                self.new_line();
            }
            _ => {
                if col >= WIDTH {
                    self.new_line();
                }
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: *character_byte,
                    colour_code: foreground as u8 | (background as u8) << 4,
                });
                self.foreground[self.cursor_pos] = foreground;
                self.background[self.cursor_pos] = background;
                self.cursor_pos +=1;
            }
        }
    }

    pub fn shift_line_up(&mut self) {
        self.clear_line(0);
        for row in 1..HEIGHT {
            for col in 0..WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        let mut row = self.cursor_pos / WIDTH;
        let mut col = (self.cursor_pos - (row * WIDTH));
        self.cursor_pos -= col;
    }

    pub fn clear_line(&mut self, row: usize) {
        let blank = ScreenChar{ ascii_character: 0, colour_code: 0 };
        for col in 0..WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn new_line(&mut self) {
        let row = self.cursor_pos / WIDTH;
        let col = (self.cursor_pos - (row * WIDTH));
        self.cursor_pos = self.cursor_pos - col + WIDTH;
    }

    pub fn write_string(&mut self, s: &str, background: Colour, foreground: Colour) {
        for byte in s.as_bytes() {
            match byte {
                // Only attempt to print ascii bytes or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte, background, foreground),
                _ => self.write_byte(&0xfe, background, foreground),
            }
        }
    }

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s, Colour::Black, Colour::White);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA_WRITER.lock().write_fmt(args).unwrap();
}