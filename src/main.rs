#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(asm)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod prelude;
mod instructions;

#[cfg(test)]
mod tests;

use core::panic::PanicInfo;
use core::fmt::Write;
use prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Test");

    #[cfg(test)]
    test_main();
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

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}