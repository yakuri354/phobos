use crate::arch::PAGE_SIZE;
use crate::mm::alloc::buddy::{BuddyAlloc, PagedBuddyAllocator};
use crate::sync::irq_lock::{IRQSpinlock, InterruptGuard};
use core::alloc::{AllocError, GlobalAlloc, Layout};
use core::ops::DerefMut;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use liballoc::LiballocAllocator;
use spin::{Mutex, Spin};
use spin::mutex::{SpinMutex, SpinMutexGuard};
use x86_64::structures::paging::{PageTableFlags, PhysFrame, Size4KiB};
use crate::arch::mem::PageRepr;
use crate::data::late_init::LateInit;
use crate::mm::alloc::setup::{BootMemAllocator, BumpAlloc};
use crate::mm::alloc::stack::StackAllocator;
use crate::mm::alloc::virt::VirtualMemoryAllocator;
use crate::mm::PageFrameRange;

mod buddy;
pub mod setup;
mod stack;
mod virt;

unsafe trait SinglePageAllocator {
    fn alloc_frame(&mut self) -> PhysFrame<Size4KiB>;
    unsafe fn free_page(&mut self, ptr: PhysFrame<Size4KiB>);
    unsafe fn init(&mut self, range: PageFrameRange, exclude: &[PageFrameRange]);
}

enum AllocState {
    None,
    BootMem(BumpAlloc),
    Default
}

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

#[global_allocator]
static GLOBAL_ALLOC: Locked<GlobalAllocator> = Locked::new(GlobalAllocator(AllocState::None));

pub struct GlobalAllocator(AllocState);

unsafe impl GlobalAlloc for Locked<GlobalAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match &mut self.lock().0 {
            AllocState::None => panic!("Allocator not initialized"),
            AllocState::BootMem(bma) => bma.alloc_raw(layout),
            AllocState::Default => LiballocAllocator::alloc(&LiballocAllocator, layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match &mut self.lock().0 {
            AllocState::None => panic!("Allocator not initialized"),
            AllocState::BootMem(_) => {}
            AllocState::Default => LiballocAllocator::dealloc(&LiballocAllocator, ptr, layout)
        }
    }
}

pub fn init_bootmem_alloc() {

}

static GLOBAL_LIB_ALLOC_LOCK: SpinMutex<()> = SpinMutex::new(());

static GLOBAL_VM_ALLOC: Locked<LateInit<VirtualMemoryAllocator>> = Locked::new(LateInit::new());

pub unsafe fn init_allocator(vm_alloc: ) {

    liballoc::init(
        || { SpinMutexGuard::leak(GLOBAL_LIB_ALLOC_LOCK.lock()); true },
        || { GLOBAL_LIB_ALLOC_LOCK.force_unlock(); true },
        move |count| vm_alloc.alloc_pages(count as _, PageTableFlags::NO_EXECUTE)
    )
}

pub fn alloc_page()