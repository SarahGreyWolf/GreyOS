
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(code: QemuExitCode) {
    use crate::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(code as u32);
    }
}