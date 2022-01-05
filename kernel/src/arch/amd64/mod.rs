use log::info;
pub use x86_64::{PhysAddr, VirtAddr};

use boot_lib::KernelArgs;
use mem::setup;

use crate::kernel_main;

pub mod bit_ops;
pub mod const_data;
pub mod debug;
pub mod fb;
pub mod interrupt;
pub mod mem;

pub use mem::PAGE_SIZE;

#[no_mangle]
pub unsafe extern "C" fn _start(args: *mut KernelArgs) -> ! {
    super::debug::init_debug_logger().unwrap(); // Cannot `expect` it :(
    info!("phobos kernel v{} on x86_64", env!("CARGO_PKG_VERSION"));

    setup::init(&mut *args);

    kernel_main()
}
