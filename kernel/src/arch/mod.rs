mod amd64;
#[cfg(target_arch = "x86_64")]
pub use amd64::*;

mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(not(target_os = "none"))]
compile_error!("Looks like the kernel is being compiled for an invalid target");
