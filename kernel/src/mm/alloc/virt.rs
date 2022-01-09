//! The virtual memory manager is responsible for managing pages, etc.

use core::ptr::NonNull;
use x86_64::structures::paging::{Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags, RecursivePageTable, Size4KiB};
use x86_64::structures::paging::mapper::MappedFrame::Size4KiB;
use x86_64::VirtAddr;
use crate::arch::mem::get_pt;
use crate::arch::PAGE_SIZE;
use crate::mm::alloc::buddy::BuddyAlloc;
use crate::mm::alloc::SinglePageAllocator;
use crate::mm::alloc::stack::StackAllocator;
use crate::mm::Pointable;

pub struct VirtualMemoryAllocator {
    phys_alloc: StackAllocator,
    curr: VirtAddr,
    end: VirtAddr,
}

impl VirtualMemoryAllocator {
    pub fn alloc_pages(&mut self, count: usize, flags: PageTableFlags) -> NonNull<u8> {
        let begin = self.curr;
        for _ in 0..count {
            if self.curr == end {
                panic!("Out of virtual memory")
            }
            let frame = self.phys_alloc.alloc_frame();
            unsafe { get_pt().map_to(Page::containing_address(curr), frame, flags, &mut self.phys_alloc) }
            self.curr = VirtAddr::new(self.curr.as_u64() + Size4KiB::SIZE)
        }
        return begin.pointer();
    }

    pub fn free_pages(&mut self, _: NonNull<u8>, _: usize) {} // Does nothing

    pub fn init(phys_alloc: StackAllocator, start: VirtAddr, end: VirtAddr) -> Self {
        Self {
            phys_alloc,
            curr: start,
            end,
        }
    }
}
