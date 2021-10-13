#![feature(asm)]
#![feature(array_chunks)]
#![no_std]

pub mod config_reg;
pub mod mm;

pub struct PhysicalAddress(u64);

pub fn init() {
    unsafe { asm!("nop") }
}
