#![feature(default_alloc_error_handler)]
#![feature(int_log)]
#![feature(allocator_api)]
#![feature(abi_efiapi)]
#![feature(asm_const)]
#![feature(abi_x86_interrupt)]
#![feature(exclusive_range_pattern)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

use core::arch::asm;
use log::info;

/// Kernel diagnostic facilities, such as panics, logging, etc.
#[macro_use]
mod diag;
mod arch;
mod aux;
mod data;
mod graphics;
mod mm;
mod io;
mod sync;
mod fs;
mod device;

pub fn kernel_main() -> ! {
    info!("Starting main kernel loop");
    x86_64::instructions::interrupts::enable();
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
