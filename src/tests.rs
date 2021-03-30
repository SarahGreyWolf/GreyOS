use crate::prelude::*;

#[test_case]
fn test_assert() {
    print!("Test assert... ");
    assert_eq!(1, 1);
    println!("[ok]");
}