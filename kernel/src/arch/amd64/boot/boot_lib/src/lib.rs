#![feature(abi_efiapi)]
#![no_std]

use arrayvec::ArrayVec;
use core::fmt::{Debug, Formatter};
use uefi::{
    proto::console::gop::ModeInfo,
    table::{boot::MemoryDescriptor, Runtime, SystemTable},
};

pub type KernelEntryPoint = extern "efiapi" fn(*mut KernelArgs) -> !;

pub const KERNEL_ARGS_MDL_SIZE: u64 = 512;

pub const KERNEL_MEM_TYPE_RANGE_START: u32 = 0x80000000;
pub const KERNEL_RX_MEM_TYPE: u32 = 0x80000001;
pub const KERNEL_RW_MEM_TYPE: u32 = 0x80000002;
pub const KERNEL_RO_MEM_TYPE: u32 = 0x80000003;
pub const KERNEL_RWX_MEM_TYPE: u32 = 0x80000004;
pub const KERNEL_STACK_MEM_TYPE: u32 = 0x80000005;
pub const PTE_MEM_TYPE: u32 = 0x80000006;
pub const KERNEL_ARGS_MEM_TYPE: u32 = 0x80000007;
pub const PHYS_MAP_OFFSET: u64 = 0xFFFFFFF000000000;
pub const KERNEL_STACK_SIZE_PAGES: u64 = 256;
pub const KERNEL_STACK_BOTTOM: u64 = 0xFFFFFFF000000000 - 0x1000;

#[repr(C)]
pub struct KernelArgs {
    pub mmap: ArrayVec<MemoryDescriptor, 512>,
    pub uefi_rst: SystemTable<Runtime>,
    pub fb_addr: *mut u8,
    pub fb_info: ModeInfo,
}

impl Debug for KernelArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("KernelArgs with MDL: ")?;

        for i in self.mmap.iter() {
            f.write_fmt(format_args!("{:?}\n", i))?
        }

        Result::Ok(())
    }
}
