use core::cell::RefCell;
use core::fmt::Write;
use core::mem::MaybeUninit;

use lazy_static::lazy_static;
use log::info;
use spin::Mutex as Spinlock;
use uart_16550::SerialPort;
use uefi::table::runtime::ResetType;
use uefi::Status;
pub use x86_64::{PhysAddr, VirtAddr};

use boot_ffi::KernelArgs;
use mem::setup;

use crate::kernel_main;

pub mod const_data;
pub mod debug;
pub mod fb;
pub mod interrupt;
pub mod mem;
pub mod bit_ops;

pub use mem::PAGE_SIZE;

#[no_mangle]
pub unsafe extern "C" fn _start(args: *mut KernelArgs) -> ! {
    super::debug::init_debug_logger().unwrap(); // Cannot `expect` it :(
    info!("phobos kernel v{} on x86_64", env!("CARGO_PKG_VERSION"));

    setup::init(&mut *args);

    crate::kernel_main()
}