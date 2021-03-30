use volatile::Volatile;
use core::fmt;
use core::fmt::{Arguments, write, Write};

const WIDTH: usize = 80;
const HEIGHT: usize = 25;

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
    pub cursor: usize,
    buffer: &'static mut Buffer,
    foreground: [Colour; WIDTH*HEIGHT],
    background: [Colour; WIDTH*HEIGHT],
}

impl Writer {
    pub fn init() -> Self {
        Self {
            cursor: 0,
            // Get a mutable reference to a pointer that is a mutable pointer ????? Madness
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            foreground: [Colour::Black; WIDTH*HEIGHT],
            background: [Colour::Black; WIDTH*HEIGHT],
        }
    }

    pub fn write_byte(&mut self, character_byte: &u8, background: Colour, foreground: Colour) {
        let mut row = self.cursor / WIDTH;
        let mut col = (self.cursor - (row * WIDTH));
        if self.cursor >= WIDTH * HEIGHT {
            self.shift_line_up();
        }
        row = self.cursor / WIDTH;
        col = (self.cursor - (row * WIDTH));
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
                self.foreground[self.cursor] = foreground;
                self.background[self.cursor] = background;
                self.cursor+=1;
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
        let mut row = self.cursor / WIDTH;
        let mut col = (self.cursor - (row * WIDTH));
        self.cursor -= col;
    }

    pub fn clear_line(&mut self, row: usize) {
        let blank = ScreenChar{ ascii_character: 0, colour_code: 0 };
        for col in 0..WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn new_line(&mut self) {
        let row = self.cursor / WIDTH;
        let col = (self.cursor - (row * WIDTH));
        self.cursor = self.cursor - col + WIDTH;
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