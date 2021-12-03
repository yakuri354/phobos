#![feature(asm)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

use log::info;

mod arch;
mod data;
/// Kernel diagnostic facilities, such as panics, logging, etc.
mod diag;
mod mm;
mod sync;

pub fn kernel_main() -> ! {
    loop {}
}
