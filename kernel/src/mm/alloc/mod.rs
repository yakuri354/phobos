use crate::{
    data::{late_init::LateInit, misc::Pointable},
    mm::alloc::{
        setup::{BootMemAllocator, BumpAlloc},
        virt::{VAllocFlags, GLOBAL_VM_ALLOC},
    },
    sync::irq_lock::InterruptGuard,
};
use alloc::vec;
use core::alloc::{GlobalAlloc, Layout};
use liballoc::LiballocAllocator;
use spin::mutex::{SpinMutex, SpinMutexGuard};
use x86_64::{
    structures::paging::{frame::PhysFrameRange, PageTableFlags, PhysFrame, Size4KiB},
    VirtAddr,
};

use crate::{data::misc::Global, sync::irq_lock::IRQLocked};

pub mod phys;
pub mod setup;
mod stack;
pub mod virt;

unsafe trait SinglePageAllocator {
    fn alloc_frame(&mut self) -> PhysFrame<Size4KiB>;
    unsafe fn free_page(&mut self, ptr: PhysFrame<Size4KiB>);
    unsafe fn init(&mut self, range: PhysFrameRange, exclude: &[PhysFrameRange]);
}

enum AllocState {
    None,
    BootMem(BumpAlloc),
    Default,
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

#[global_allocator]
static GLOBAL_ALLOC: IRQLocked<GlobalAllocator> =
    IRQLocked::new(GlobalAllocator(Global::new(AllocState::None)));

pub struct GlobalAllocator(Global<AllocState>);

static GLOBAL_LIB_ALLOC_LOCK: SpinMutex<()> = SpinMutex::new(());

pub unsafe fn init_liballoc() {
    // TODO AllocType
    liballoc::init(
        || {
            SpinMutexGuard::leak(GLOBAL_LIB_ALLOC_LOCK.lock());
            true
        },
        || {
            GLOBAL_LIB_ALLOC_LOCK.force_unlock();
            true
        },
        move |count| {
            GLOBAL_VM_ALLOC
                .lock()
                .alloc(
                    count as _,
                    VAllocFlags::COMMIT,
                    PageTableFlags::WRITABLE | PageTableFlags::PRESENT,
                )
                .map(|x| x.start.pointer())
        },
        |ptr, count| {
            GLOBAL_VM_ALLOC
                .lock()
                .free(VirtAddr::from_pointer(ptr), count as _);
            true
        },
    );
    // test
    let _ = vec![0u32; 100];
}

unsafe impl GlobalAlloc for IRQLocked<GlobalAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        LiballocAllocator.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        LiballocAllocator.dealloc(ptr, layout)
    }
}
