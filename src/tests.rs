use crate::prelude::*;

#[test_case]
fn test_assert() {
    serial_print!("Test assert... ");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}