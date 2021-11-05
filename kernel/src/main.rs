#![feature(asm)]
#![feature(default_alloc_error_handler)]
#![no_std]
#![no_main]

use log::info;

mod arch;
/// Kernel diagnostic facilities, such as panics, logging, etc.
mod diag;
mod mm;
mod sync;

pub fn kernel_main() -> ! {
    diag::init();
    info!("phobos kernel v{}", env!("CARGO_PKG_VERSION"));

    loop {}
}
