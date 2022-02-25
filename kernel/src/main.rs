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

use alloc::string::{String, ToString};
use core::arch::asm;

use log::info;

use crate::{
    device::ps2kb::print_keypresses,
    task::{executor::Executor, Task},
};

/// Kernel diagnostic facilities, such as panics, logging, etc.
#[macro_use]
mod diag;
/// Architecture-specific code
mod arch;
/// Auxillary code, can be useful
mod aux;
/// Data structures
mod data;
/// Drivers
mod device;
/// Filesystem code
mod fs;
/// Graphics system
mod graphics;
/// Input/Output system
mod io;
/// Memory manager
mod mm;
/// Synchronisation primitives
mod sync;
/// Async and cooperative multitasking
mod task;

pub fn kernel_main() -> ! {
    info!("Starting main kernel loop");

    let mut executor = Executor::new();
    executor.spawn(Task::new(print_keypresses()));
    executor.run()
}
