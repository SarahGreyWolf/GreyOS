#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Bytes of ascii Hello World to write to VGA Text Buffer
static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    // Create a pointer to the memory address of the VGA Buffer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // Write to the memory address of the buffer with an offset
            // EG. Write the ascii value of e to 0xb8000 + 1
            *vga_buffer.offset(i as isize * 2) = byte;
            // Make the foreground/text colour a light cyan
            *vga_buffer.offset(i as isize * 2 + 1) = 0x9 | 0x40;
        }
    }
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // Write to the memory address of the buffer with an offset
            // EG. Write the ascii value of e to 0xb8000 + 1
            *vga_buffer.offset(i as isize * 2 + 160) = byte;
            // Make the background colour a light cyan
            *vga_buffer.offset(i as isize * 2 + 1 + 160) = 0xb;
        }
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