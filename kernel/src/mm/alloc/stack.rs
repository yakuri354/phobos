use crate::{data::list::CDLListHead, mm::alloc::SinglePageAllocator};

use crate::data::misc::Pointable;
use x86_64::structures::paging::{frame::PhysFrameRange, FrameAllocator, PhysFrame, Size4KiB};

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
                break;
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
