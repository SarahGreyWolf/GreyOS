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

mod vga {
    use crate::prelude::*;
    #[test_case]
    fn test_println_simple() {
        println!("test_println_simple output, this is the full length of a line, a line being 80..");
    }
    #[test_case]
    fn test_println_multi() {
        for _ in 0..200 {
            println!("test_println_multi output");
        }
    }

    #[test_case]
    fn test_println_output() {
        let s = "Some test string that fits on a single line, here it might overlap if I keep";
        println!("{}", s);
        for (i, c) in s.chars().enumerate() {
            let screen_char = VGA_WRITER.lock().buffer.chars[crate::vga::HEIGHT - 1][i].read();;
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    }
}