#![feature(asm)]
#![feature(default_alloc_error_handler)]
#![feature(int_log)]
#![feature(allocator_api)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

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
