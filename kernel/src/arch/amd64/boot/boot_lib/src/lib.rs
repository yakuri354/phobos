#![no_std]

use core::fmt::{Debug, Formatter};
use uefi::table::boot::MemoryDescriptor;
use uefi::table::{Runtime, SystemTable};

pub type KernelEntryPoint = extern "C" fn(*mut KernelArgs) -> !;

pub const KERNEL_ARGS_MDL_SIZE: usize = 512;

pub const KERNEL_MEM_TYPE_RANGE_START: u32 = 0x80000000;
pub const KERNEL_RX_MEM_TYPE: u32 = 0x80000001;
pub const KERNEL_RW_MEM_TYPE: u32 = 0x80000002;
pub const KERNEL_RO_MEM_TYPE: u32 = 0x80000003;
pub const KERNEL_RWX_MEM_TYPE: u32 = 0x80000004;

#[repr(C)]
pub struct KernelArgs {
    pub mmap: [MemoryDescriptor; KERNEL_ARGS_MDL_SIZE],
    pub uefi_rst: SystemTable<Runtime>,
}

impl Debug for KernelArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("KernelArgs with MDL: ")?;

        for i in self.mmap {
            f.write_fmt(format_args!("{:?}\n", i))?
        }

        Result::Ok(())
    }
}
