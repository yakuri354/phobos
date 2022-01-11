use core::arch::asm;
use boot_lib::KernelArgs;
use core::ptr::{addr_of_mut, null_mut};
use log::info;
use mem::setup;
pub use x86_64::{PhysAddr, VirtAddr};

use crate::kernel_main;

pub mod bit_ops;
pub mod debug;
pub mod fb;
pub mod interrupt;
pub mod mem;

pub use mem::PAGE_SIZE;
use crate::arch::mem::KERNEL_STACK;

#[no_mangle]
// #[naked]
pub unsafe extern "efiapi" fn _start(args: *mut KernelArgs) -> ! {
    // Switch stack pointer to
    // let args = null_mut();
    // asm!(
    //     "mov rsp, {}",
    //     "",
    //     in(reg) (&mut KERNEL_STACK as *mut _),
    //     out(reg) _args
    // );

    // FIXME
    super::debug::init_debug_logger().unwrap(); // Cannot `expect` it :(
    info!("phobos kernel v{} on x86_64", env!("CARGO_PKG_VERSION"));

    setup::init(args.as_mut().unwrap());

    kernel_main()
}
