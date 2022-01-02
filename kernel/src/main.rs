#![feature(asm)]
#![feature(default_alloc_error_handler)]
#![feature(int_log)]
#![no_std]
#![no_main]

use log::info;

/// Kernel diagnostic facilities, such as panics, logging, etc.
#[macro_use]
mod diag;
mod arch;
mod data;
mod mm;
mod sync;

pub fn kernel_main() -> ! {
    loop {}
}
