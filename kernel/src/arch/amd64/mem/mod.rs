use core::ptr::NonNull;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, PageTable};
use x86_64::VirtAddr;
use crate::arch::bit_ops::USIZE_BITS;
use crate::mm::Pointable;

mod mapper;
pub mod setup;

pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const PAGE_OFFSET_MASK: usize = PAGE_SIZE - 1;
pub const PHYS_MAP_OFFSET: usize = 0xFFFFEF0000000000;
pub const PHYS_MASK: usize = (!0) >> (USIZE_BITS - PHYS_MAP_OFFSET.trailing_zeros() as usize);

#[inline]
pub const fn page_to_pfn(addr: usize) -> usize {
    (addr & PHYS_MASK) >> PAGE_SHIFT
}

#[inline]
pub const fn pfn_to_page(pfn: usize) -> usize {
    PHYS_MAP_OFFSET | (pfn << PAGE_SHIFT) as usize
}

#[repr(align(0x1000))]
pub struct PageRepr;

pub fn get_pt() -> OffsetPageTable {
    let (pml4_frame, _) = Cr3::read();
    unsafe { OffsetPageTable::new(pml4_frame.pointer().cast().as_mut(), VirtAddr::new(PHYS_MAP_OFFSET as _)) }
}
