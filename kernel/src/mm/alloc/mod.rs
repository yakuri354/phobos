use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::AtomicBool;
use liballoc::LiballocAllocator;
use spin::{Mutex, Spin};
use crate::sync::irq_lock::IRQSpinlock;
use crate::arch::PAGE_SIZE;
use crate::mm::alloc::buddy::{PagedBuddyAllocator, BuddyAlloc, log2_ceil};

mod buddy;
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
            user: 0,
        }
    }
}

static ALLOCATOR_INITIALIZED: AtomicBool = AtomicBool::new(false);

// TODO Fix global_allocator collision and implement a global allocator

// #[global_allocator]
// static GLOBAL_ALLOC: GlobalAllocator = GlobalAllocator;
// static GLOBAL_ALLOCATOR_LOCK: IRQSpinlock<()> = IRQSpinlock::new(());
// static mut GLOBAL_BUDDY: BuddyAlloc = BuddyAlloc::new();

pub struct GlobalAllocator;

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

pub fn init() {
    todo!()
}
