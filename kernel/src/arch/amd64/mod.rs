use boot_lib::KernelArgs;
use core::{
    arch::asm,
    ops::{Deref, DerefMut},
    ptr::{addr_of_mut, null_mut, NonNull},
};
use log::{info, Log};
use x86_64::registers::{control::Cr3, read_rip};
pub use x86_64::{PhysAddr, VirtAddr};

use crate::kernel_main;

pub mod bit_ops;
pub mod debug;
pub mod fb;
pub mod interrupt;
pub mod mem;

use crate::{
    arch::mem::KERNEL_STACK,
    diag::{logger, logger::FB_LOGGER},
    graphics::fb::{FbDisplay, GLOBAL_FB},
};
pub use mem::PAGE_SIZE;

#[no_mangle]
pub unsafe extern "efiapi" fn _start(args: *mut KernelArgs) -> ! {
    crate::diag::init();

    info!("phobos kernel v{} on x86_64", env!("CARGO_PKG_VERSION"));
    let mut args = args.as_mut().unwrap();

    info!("Initializing basic exception handlers");

    interrupt::idt::init_basic_ex_handling();

    info!("Initializing memory manager");

    mem::setup::init(args);

    info!("Initializing framebuffer");

    let mut fb = GLOBAL_FB.lock();

    fb.deref_mut().init(FbDisplay::new(
        NonNull::new(args.fb_addr).unwrap().cast(),
        args.fb_info,
    ));

    crate::graphics::fb::clear();
    FB_LOGGER.lock().reinit_with_fb();

    info!("phobos kernel v{} on x86_64", env!("CARGO_PKG_VERSION"));

    kernel_main()
}
