use alloc::{fmt::format, format, string::String, vec::Vec};
use core::{arch::x86_64::__cpuid, iter::repeat, ptr::NonNull};

use log::{debug, error, info, trace, warn};
use raw_cpuid::CpuId;
use x86_64::instructions::interrupts::int3;
pub use x86_64::{PhysAddr, VirtAddr};

use boot_lib::KernelArgs;
pub use mem::PAGE_SIZE;

use crate::{diag::reinit_with_fb, kernel_main};

pub mod bit_ops;
pub mod context;
pub mod debug;
pub mod interrupt;
pub mod mem;

#[no_mangle]
pub unsafe extern "efiapi" fn _start(args: *mut KernelArgs) -> ! {
    crate::diag::init();

    info!("phobos kernel v{} on x86_64", env!("CARGO_PKG_VERSION"));
    let args = args.as_mut().unwrap();

    info!("Initializing arch specific structures");

    interrupt::idt::init_cpu_structures();

    info!("Initializing memory manager");

    mem::setup::init(args);

    info!("Initializing framebuffer");

    reinit_with_fb(NonNull::new(args.fb_addr).unwrap(), args.fb_info);

    info!("phobos v{} running on x86_64", env!("CARGO_PKG_VERSION"));

    kernel_main()
}
