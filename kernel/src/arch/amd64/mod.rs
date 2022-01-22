use alloc::{fmt::format, format, string::String, vec::Vec};
use boot_lib::KernelArgs;
use core::{iter::repeat, ptr::NonNull};
use log::{error, info, trace, warn};

pub use x86_64::{PhysAddr, VirtAddr};

use crate::kernel_main;

pub mod bit_ops;
pub mod debug;
pub mod fb;
pub mod interrupt;
pub mod mem;

use crate::diag::reinit_with_fb;
pub use mem::PAGE_SIZE;

#[no_mangle]
pub unsafe extern "efiapi" fn _start(args: *mut KernelArgs) -> ! {
    crate::diag::init();

    info!("phobos kernel v{} on x86_64", env!("CARGO_PKG_VERSION"));
    let args = args.as_mut().unwrap();

    info!("Initializing basic exception handlers");

    interrupt::idt::init_basic_ex_handling();

    info!("Initializing memory manager");

    mem::setup::init(args);

    info!("Initializing framebuffer");

    reinit_with_fb(NonNull::new(args.fb_addr).unwrap(), args.fb_info);

    info!("phobos v{} running on x86_64", env!("CARGO_PKG_VERSION"));

    error!("\tTEST_E\nTEST_E\nTEST_E");
    warn!(
        "W: {}",
        (0..1000)
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    );
    trace!("TRACE");
    kernel_main()
}
