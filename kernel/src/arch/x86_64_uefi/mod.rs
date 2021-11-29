use core::cell::RefCell;
use core::fmt::Write;
use core::mem::MaybeUninit;
use log::info;
use lazy_static::lazy_static;
use spin::Mutex as Spinlock;
use uart_16550::SerialPort;
use uefi::Status;
use uefi::table::runtime::ResetType;
pub use x86_64::{PhysAddr, VirtAddr};

use boot_ffi::KernelArgs;

use crate::kernel_main;

pub mod debug;
pub mod paging;
pub mod interrupt;
mod gdt;

#[no_mangle]
pub unsafe extern "C" fn _start(args: *mut KernelArgs) -> ! {
    super::debug::init_debug_logger().unwrap(); // Cannot `expect` it :(
    info!("phobos kernel v{} on x86_64", env!("CARGO_PKG_VERSION"));

    paging::init(&mut *args);

    crate::kernel_main()
}
