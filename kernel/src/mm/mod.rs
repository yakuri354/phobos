use crate::{
    data::late_init::LateInit,
    sync::irq_lock::IRQLocked,
};
use arrayvec::ArrayVec;


use uefi::table::boot::MemoryDescriptor;


pub mod alloc;
mod aux;
pub mod mapping;

pub const SYSTEM_MEMORY_MAP: IRQLocked<LateInit<&'static mut ArrayVec<MemoryDescriptor, 512>>> =
    IRQLocked::new(LateInit::new());
