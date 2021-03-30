#![feature(asm)]
#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;
use vga::{Writer, Colour};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga_buffer = vga::Writer::init();

    vga_buffer.write_string("This is a test", Colour::Grey,
                            Colour::LightMagenta);
    loop {}
}


// Called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // If it panics, loop forever
    // Like executing a nop forever?
    loop{}
}