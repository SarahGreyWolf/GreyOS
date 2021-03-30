use crate::prelude::*;

pub trait Testable {
    fn run(&self) -> ();
}

// Implements Testable for all Functions when in scope
impl<T> Testable for T
where
    T: Fn(), {
    fn run(&self) -> () {
        serial_println!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[test_case]
fn test_assert() {
    assert_eq!(1, 1);
}