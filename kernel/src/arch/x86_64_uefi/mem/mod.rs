use x86_64::PhysAddr;
use crate::mm::{PhysMemRange, PAGE_SIZE};

/// This very basic allocator is needed for bootstrapping paging and another better allocator
/// ### Safety notice: The caller must ensure that identity paging is enabled
pub struct WatermarkAlloc {
    curr: PhysAddr,
    range: PhysMemRange,
}

impl WatermarkAlloc {
    pub fn new(range: PhysMemRange) -> Self {
        Self {
            curr: range.start,
            range
        }
    }
    pub unsafe fn alloc_pages(&mut self, pages: usize) -> Option<*mut u8> {
        let new = self.curr + pages * PAGE_SIZE;
        if new > self.range.end {
            None
        } else {
            let curr = self.curr;
            self.curr = new;
            Some(curr.as_u64() as _)
        }
    }
}

pub fn init_paging() {

}