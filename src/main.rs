#![feature(asm)]
#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use vga::{VGATextBuffer, Colour};

// Bytes of ascii Hello World to write to VGA Text Buffer
static HELLO: &[u8] = b"Hello World ";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga_buffer = vga::VGATextBuffer::init();

    for &byte in HELLO.iter() {
        vga_buffer.write_char(byte, Colour::Grey,
                              Colour::LightMagenta);
    }

    loop {}
}


// Called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // If it panics, loop forever
    // Like executing a nop forever?
    loop{}
}