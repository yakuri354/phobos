use crate::{
    arch::{PhysAddr, PAGE_SIZE},
    data::late_init::LateInit,
    sync::irq_lock::IRQLocked,
};
use arrayvec::ArrayVec;
use boot_lib::PHYS_MAP_OFFSET;
use core::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};
use uefi::table::boot::MemoryDescriptor;
use x86_64::{
    structures::paging::{Page, PhysFrame},
    VirtAddr,
};

pub mod alloc;
mod aux;

pub const SYSTEM_MEMORY_MAP: IRQLocked<LateInit<&'static mut ArrayVec<MemoryDescriptor, 512>>> =
    IRQLocked::new(LateInit::new());
