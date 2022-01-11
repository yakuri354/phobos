use crate::{
    arch::{
        mem::{PAGE_OFFSET_MASK, PAGE_SHIFT},
        PAGE_SIZE,
    },
    data::{
        list::{CDLListHead, SLListNode},
        misc::Pointable,
    },
    sync::irq_lock::IRQLocked,
};
use boot_lib::PHYS_MAP_OFFSET;
use core::ptr::NonNull;
use log::info;
use uefi::table::boot::{MemoryDescriptor, MemoryType};
use x86_64::structures::paging::{
    frame::PhysFrameRange, FrameAllocator, PageSize, PhysFrame, Size4KiB,
};

pub static GLOBAL_PHYS_ALLOC: IRQLocked<LinkedListAllocator> =
    IRQLocked::new(LinkedListAllocator::new());

unsafe impl Send for LinkedListAllocator {}

pub struct LinkedListAllocator {
    pub dirty: SLListNode,
    pub clean: SLListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        Self {
            dirty: SLListNode { next: None },
            clean: SLListNode { next: None },
        }
    }

    pub fn get_clean(&mut self) -> Option<NonNull<u8>> {
        if let Some(clean) = self.clean.pop() {
            return Some(clean);
        }
        if let Some(dirty) = self.dirty.pop() {
            unsafe {
                clear_page(dirty);
            }
            return Some(dirty);
        }
        return None;
    }
}

pub fn init_phys_alloc_from_mmap<'a, T>(mmap: T)
where
    T: IntoIterator<Item = &'a MemoryDescriptor>,
{
    let mut g_all = GLOBAL_PHYS_ALLOC.lock();
    for i in mmap {
        info!("Adding range {:#x} -- {:#x} of type {:?}", i.phys_start, i.phys_start + i.page_count * Size4KiB::SIZE, i.ty);
        match i.ty {
            MemoryType::CONVENTIONAL
            | MemoryType::BOOT_SERVICES_CODE
            | MemoryType::BOOT_SERVICES_DATA
            | MemoryType::LOADER_CODE
            | MemoryType::LOADER_DATA => unsafe {
                for j in 0..i.page_count {
                    g_all.dirty.push(NonNull::new_unchecked(
                        (PHYS_MAP_OFFSET + i.phys_start + j * Size4KiB::SIZE) as *mut _,
                    ))
                }
            },
            _ => {}
        }
    }
}

pub unsafe fn clear_page(va: NonNull<u8>) {
    unsafe { va.as_ptr().write_bytes(0, PAGE_SIZE as usize) };
}

pub struct GlobalFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for GlobalFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        GLOBAL_PHYS_ALLOC
            .lock()
            .get_clean()
            .map(|p| {
                info!("Got a frame: {:?}", p);
                p
            })
            .map(PhysFrame::from_pointer)
    }
}
