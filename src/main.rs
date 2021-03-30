#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}


// Called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // If it panics, loop forever
    // Like executing a nop forever?
    loop{}
}