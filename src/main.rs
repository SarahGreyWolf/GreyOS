#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(asm)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga;
mod prelude;
mod instructions;
mod qemu;
mod serial;

#[cfg(test)]
mod tests;

use core::panic::PanicInfo;
use core::fmt::Write;
use prelude::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Test");
    println!("Hello?");

    #[cfg(test)]
    test_main();
    loop {}
}


// Called on Panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // If it panics, loop forever
    // Like executing a nop forever?
    loop{}
}
// Called on Panic
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    // If it panics, loop forever
    // Like executing a nop forever?
    loop{}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn tests::Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        // Execute the run method from the Testable trait
        // Allows for print the status without every test needing to do so
        test.run();
    }
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}