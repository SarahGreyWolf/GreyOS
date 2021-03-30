
const WIDTH: usize = 80;
const HEIGHT: usize = 25;

#[derive(Copy, Clone)]
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

pub struct VGATextBuffer {
    cursor: usize,
    foreground: [Colour; WIDTH*HEIGHT],
    background: [Colour; WIDTH*HEIGHT],
    contents: [u8; WIDTH*HEIGHT],
}

impl VGATextBuffer {
    pub fn init() -> Self {
        Self {
            cursor: 0,
            foreground: [Colour::Black; WIDTH*HEIGHT],
            background: [Colour::Black; WIDTH*HEIGHT],
            contents: [0u8; WIDTH*HEIGHT],
        }
    }

    pub fn write_char(&mut self, character_byte: u8, background: Colour, foreground: Colour) {
        // Create a pointer to the memory address of the VGA Buffer
        let pointer = 0xb8000 as *mut u8;
        if self.cursor == WIDTH * HEIGHT {
            return;
        } else {
            unsafe {
                // Write to the memory address of the buffer with an offset
                // EG. Write the ascii value of e to 0xb8000 + 1
                *pointer.offset(self.cursor as isize) = character_byte;
                // Make the background colour a light cyan
                *pointer.offset(self.cursor as isize + 1) = foreground as u8 | (background as u8) << 4;
                self.contents[self.cursor] = character_byte;
                self.foreground[self.cursor] = foreground;
                self.background[self.cursor] = background;
                self.cursor+=2;
            }
        }
    }
}