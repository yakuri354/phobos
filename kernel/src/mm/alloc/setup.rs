use alloc::boxed::Box;
use core::{
    alloc::Layout,
    mem::{align_of, size_of},
    ptr::{slice_from_raw_parts_mut, NonNull},
};

/// This very basic allocator is needed for bootstrapping paging and another better allocator
pub struct BumpAlloc {
    curr: *mut u8,
    start: *mut u8,
    end: *mut u8,
}

impl BumpAlloc {
    pub unsafe fn new(start: *mut u8, end: *mut u8) -> Self {
        Self {
            curr: start,
            start,
            end,
        }
    }

    pub fn alloc_fallible(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        let offset = self.curr.align_offset(layout.align());
        let new = self.curr as usize + offset + layout.size();
        if new > self.end as usize {
            None
        } else {
            let old = self.curr as usize + offset;
            self.curr = new as _;
            NonNull::new(old as _)
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> NonNull<u8> {
        self.alloc_fallible(layout).unwrap()
    }

    pub fn alloc_zeroed(&mut self, layout: Layout) -> NonNull<u8> {
        let addr = self.alloc(layout);
        unsafe { addr.as_ptr().write_bytes(0, layout.size()) };
        addr
    }
}

/// A temporary allocator which cannot free
pub unsafe trait BootMemAllocator {
    fn alloc_raw(&mut self, layout: Layout) -> *mut u8;
}

pub fn alloc_slice<T>(alloc: &mut dyn BootMemAllocator, count: usize) -> Box<[T]> {
    unsafe {
        Box::from_raw(slice_from_raw_parts_mut(
            alloc.alloc_raw(
                Layout::from_size_align(count * size_of::<T>(), align_of::<T>()).unwrap(),
            ) as _,
            count,
        ))
    }
}

unsafe impl BootMemAllocator for BumpAlloc {
    fn alloc_raw(&mut self, layout: Layout) -> *mut u8 {
        self.alloc_zeroed(layout).as_ptr()
    }
}
