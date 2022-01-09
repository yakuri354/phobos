use alloc::borrow::ToOwned;
use core::ptr::NonNull;
use x86_64::PhysAddr;
use x86_64::structures::paging::{FrameAllocator, PhysFrame, Size4KiB};
use x86_64::structures::paging::frame::PhysFrameRange;
use crate::arch::mem::PHYS_MAP_OFFSET;
use crate::data::list::CDLListHead;
use crate::mm::alloc::SinglePageAllocator;
use crate::mm::{PageFrameRange, Pointable};

pub struct StackAllocator {
    head: CDLListHead,
}

unsafe impl SinglePageAllocator for StackAllocator {
    fn alloc_frame(&mut self) -> PhysFrame<Size4KiB> {
        PhysFrame::from_pointer(self.head.pop().unwrap().cast())
    }

    unsafe fn free_page(&mut self, addr: PhysFrame<Size4KiB>) {
        self.head.push(addr.pointer().cast())
    }

    unsafe fn init(&mut self, range: PhysFrameRange, exclude: &[PhysFrameRange]) {
        let mut sorted = exclude.to_vec();
        sorted.sort_by_key(|r| r.start.start_address().as_u64());
        let mut curr = range.start;
        for i in sorted {
            if curr <= i.start {
                for j in PhysFrame::range(curr, i.start) {
                    self.free_page(j)
                }
            }
            if i.end > range.end {
                break
            }
            if curr < i.end {
                curr = i.end
            }
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for StackAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        Some(self.alloc_frame())
    }
}