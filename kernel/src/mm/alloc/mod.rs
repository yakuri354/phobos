use crate::arch::PAGE_SIZE;
use crate::mm::alloc::buddy::{BuddyAlloc, PagedBuddyAllocator};
use crate::sync::irq_lock::{IRQSpinlock, InterruptGuard};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use core::sync::atomic::AtomicBool;
use liballoc::LiballocAllocator;
use spin::{Mutex, Spin};

mod buddy;
pub mod setup;
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

struct Locked<T> {
    inner: IRQSpinlock<T>,
}

impl<T> Locked<T> {
    pub const fn new(data: T) -> Self {
        Self {
            inner: IRQSpinlock::new(data),
        }
    }

    pub fn lock(&self) -> InterruptGuard<T> {
        self.inner.lock()
    }
}

static ALLOCATOR_INITIALIZED: AtomicBool = AtomicBool::new(false);

// TODO Fix global_allocator collision and implement a global allocator

#[global_allocator]
static GLOBAL_ALLOC: Locked<GlobalAllocator> = Locked::new(GlobalAllocator);

pub struct GlobalAllocator;

unsafe impl GlobalAlloc for Locked<GlobalAllocator> {
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
