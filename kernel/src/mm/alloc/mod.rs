use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::AtomicBool;

mod buddy;
mod frame;
mod virt;

#[derive(Debug)]
pub struct AllocatorStats {
    pub total: usize,
    pub allocated: usize,
    pub user: usize,
}

impl AllocatorStats {
    pub fn new() -> AllocatorStats {
        AllocatorStats {
            total: 0,
            allocated: 0,
            user: 0
        }
    }
}

static ALLOCATOR_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[global_allocator]
static GLOBAL_ALLOC: GlobalAllocator = GlobalAllocator;

pub struct GlobalAllocator;

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}