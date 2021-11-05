use boot_ffi::KernelArgs;
use uefi::table::runtime::ResetType;
use uefi::Status;
use uart_16550::SerialPort;
use core::fmt::Write;
use crate::kernel_main;
use core::cell::{RefCell};
use core::mem::MaybeUninit;

pub use x86_64::PhysAddr;

pub mod mem;
pub mod debug;

static mut KERNEL_ARGS: Option<KernelArgs> = None;

#[no_mangle]
pub extern "C" fn _start(args: *mut KernelArgs) -> ! {
    unsafe { KERNEL_ARGS = Some(args.read()) }
    crate::kernel_main()
}
