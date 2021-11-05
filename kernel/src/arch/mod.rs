mod x86_64_uefi;
#[cfg(target_arch = "x86_64")]
pub use x86_64_uefi::*;

mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(target_os = "none"))]
compile_error!("Looks like the kernel is being compiled for an invalid target");
