#![no_std]

use uefi::table::boot::MemoryDescriptor;
use uefi::table::{Runtime, SystemTable};

pub type KernelEntryPoint = extern "C" fn(*mut KernelArgs) -> !;

pub const KERNEL_ARGS_MDL_SIZE: usize = 512;

pub const KERNEL_RX_MEM_TYPE: u32 = 0x70000001;
pub const KERNEL_RW_MEM_TYPE: u32 = 0x70000002;
pub const KERNEL_RO_MEM_TYPE: u32 = 0x70000003;
pub const KERNEL_RWX_MEM_TYPE: u32 = 0x70000004;
pub const KERNEL_STACK_RW_MEM_TYPE: u32 = 0x70000005;

pub const PHYS_MAP_OFFSET: u64 = 0xFFFFF00000000000;

#[repr(C)]
pub struct KernelArgs {
    pub mmap: [MemoryDescriptor; KERNEL_ARGS_MDL_SIZE],
    pub uefi_rst: SystemTable<Runtime>,
}
