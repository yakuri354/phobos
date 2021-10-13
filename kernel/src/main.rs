#![feature(asm)]
#![feature(llvm_asm)]
#![no_std]
#![no_main]

mod diag;

#[cfg(target_arch = "aarch64")]
pub use aarch64 as arch;

#[cfg(target_arch = "x86_64")]
pub use x86_64 as arch;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    arch::init();
    loop {}
}
