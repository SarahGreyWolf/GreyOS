use core::marker::PhantomData;

pub trait PortRead {
    unsafe fn read_from_port(port: u16) -> Self;
}

pub trait PortWrite {
    unsafe fn write_to_port(port: u16, value: Self);
}

impl PortRead for u8 {
    // Inline means that whenever this code is "called" it just dumps the function code into the file
    // rather than jumping subroutines and such
    #[inline]
    unsafe fn read_from_port(port: u16) -> Self {
        let mut value: Self = 0;
        // al is a single byte register
        // Read into register al(value), whatever is on port found at address dx(port)
        // Options: nomem means not affecting memory, nostack means does not push to stack
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
        value
    }
}

impl PortRead for u16 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> Self {
        let mut value: Self = 0;
        // ax is a 2 byte register
        // Read into register al(value), whatever is on port found at address dx(port)
        // Options: nomem means not affecting memory, nostack means does not push to stack
        asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack));
        value
    }
}

impl PortRead for u32 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> Self {
        let mut value: Self = 0;
        // eax is a 4 byte register
        // Read into register al(value), whatever is on port found at address dx(port)
        // Options: nomem means not affecting memory, nostack means does not push to stack
        asm!("in eax, dx", out("ax") value, in("dx") port, options(nomem, nostack));
        value
    }
}

impl PortWrite for u8 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: Self) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
    }
}

impl PortWrite for u16 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: Self) {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack));
    }
}

impl PortWrite for u32 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: Self) {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortReadOnly<T> {
    port: u16,
    // Zero-sized type that marks that it acts like it owns a type
    phantom: PhantomData<T>,
}

impl <T> PortReadOnly<T> {
    #[inline]
    pub const fn new(port: u16) -> PortReadOnly<T> {
        PortReadOnly {
            port,
            // Acts like it owns a T type
            phantom: PhantomData,
        }
    }
}

impl <T: PortRead> PortReadOnly<T> {
    #[inline]
    pub unsafe fn read(&mut self) -> T {
        T::read_from_port(self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortWriteOnly<T> {
    port: u16,
    // Zero-sized type that marks that it acts like it owns a type
    phantom: PhantomData<T>,
}

impl <T> PortWriteOnly<T> {
    #[inline]
    pub const fn new(port: u16) -> PortWriteOnly<T> {
        PortWriteOnly {
            port,
            // Acts like it owns a T type
            phantom: PhantomData,
        }
    }
}

impl<T: PortWrite> PortWriteOnly<T> {
    #[inline]
    pub unsafe fn write(&mut self, value: T) {
        T::write_to_port(self.port, value);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port<T> {
    port: u16,
    // Zero-sized type that marks that it acts like it owns a type
    phantom: PhantomData<T>,
}

impl<T> Port<T> {
    #[inline]
    pub const fn new(port: u16) -> PortWriteOnly<T> {
        PortWriteOnly {
            port,
            // Acts like it owns a T type
            phantom: PhantomData,
        }
    }
}

impl <T: PortRead> Port<T> {
    #[inline]
    pub unsafe fn read(&mut self) -> T {
        T::read_from_port(self.port)
    }
}

impl<T: PortWrite> Port<T> {
    #[inline]
    pub unsafe fn write(&mut self, value: T) {
        T::write_to_port(self.port, value);
    }
}