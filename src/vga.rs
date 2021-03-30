use volatile::Volatile;

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
    cursor: usize,
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

    pub fn write_byte(&mut self, character_byte: u8, background: Colour, foreground: Colour) {
        if self.cursor >= WIDTH * HEIGHT {
            return;
        } else {
            self.buffer.chars[self.cursor / WIDTH][self.cursor].write(ScreenChar {
                ascii_character: character_byte,
                colour_code: foreground as u8 | (background as u8) << 4,
            });
            self.foreground[self.cursor] = foreground;
            self.background[self.cursor] = background;
            self.cursor+=1;
        }
    }
}