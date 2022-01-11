use crate::arch::bit_ops::USIZE_BITS;

use crate::data::misc::Pointable;
use boot_lib::PHYS_MAP_OFFSET;
use x86_64::{registers::control::Cr3, structures::paging::OffsetPageTable, VirtAddr};

pub mod setup;

pub const PAGE_SHIFT: u64 = 12;
pub const PAGE_SIZE: u64 = 1 << PAGE_SHIFT;
pub const PAGE_OFFSET_MASK: u64 = PAGE_SIZE - 1;
pub const PHYS_MASK: u64 = (!0) >> (USIZE_BITS - PHYS_MAP_OFFSET.trailing_zeros() as u64);

pub static mut KERNEL_STACK: [u8; 0x20000] = [0; 0x20000];

#[inline]
pub const fn page_to_pfn(addr: u64) -> u64 {
    (addr & PHYS_MASK) >> PAGE_SHIFT
}

#[inline]
pub const fn pfn_to_page(pfn: u64) -> u64 {
    PHYS_MAP_OFFSET | (pfn << PAGE_SHIFT)
}

#[repr(align(0x1000))]
pub struct PageRepr;

pub fn get_pt() -> OffsetPageTable<'static> {
    let (pml4_frame, _) = Cr3::read();
    unsafe {
        OffsetPageTable::new(
            pml4_frame.pointer().cast().as_mut(),
            VirtAddr::new(PHYS_MAP_OFFSET as _),
        )
    }
}
