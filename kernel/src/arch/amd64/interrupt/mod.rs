pub mod idt;
pub mod timer;

pub use x86_64::instructions::interrupts::*;

pub const PIC_OFFSET: u8 = 32;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum IntIdx {
    Timer = PIC_OFFSET,
    Keyboard,
}

impl IntIdx {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
