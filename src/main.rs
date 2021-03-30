#![feature(asm)]
#![no_std]
#![no_main]

mod vga;
mod prelude;

use core::panic::PanicInfo;
use core::fmt::Write;
use prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Test");
    loop {}
}


// Called on Panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // If it panics, loop forever
    // Like executing a nop forever?
    loop{}
}