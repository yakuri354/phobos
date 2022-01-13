use boot_lib::*;
use core::{
    mem::{replace, size_of, swap, take, MaybeUninit},
    ops::{Deref, DerefMut},
};
use embedded_graphics::geometry::Dimensions;
use log::{debug, info};
use uefi::table::boot::{MemoryAttribute, MemoryDescriptor, MemoryType};
use x86_64::{
    structures::paging::{
        page::PageRange, Page, PageTable, PageTableFlags, PageTableIndex, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use super::PAGE_SIZE;
use crate::mm::alloc::setup::{BootMemAllocator, BumpAlloc};
use arrayvec::ArrayVec;
use core::{
    alloc::Layout,
    arch::asm,
    convert::TryFrom,
    ptr::{addr_of, null, null_mut, NonNull},
};
use uefi::{
    table::{Runtime, SystemTable},
    ResultExt,
};

use crate::{
    arch::{fb, interrupt, mem::get_pt},
    graphics::fb::{FbDisplay, GLOBAL_FB},
    mm::{
        alloc::{
            init_liballoc,
            phys::init_phys_alloc_from_mmap,
            virt::{alloc_and_map_at, alloc_and_map_at_range, KERNEL_MAP_OFFSET},
        },
        mapping::unmap_range,
        SYSTEM_MEMORY_MAP,
    },
};
use boot_lib::PHYS_MAP_OFFSET;
use x86_64::{
    instructions::{interrupts::int3, tlb::flush_all},
    registers::control::Cr3,
    structures::paging::{
        mapper::CleanUp, page::PageRangeInclusive, FrameDeallocator, Mapper, PageSize,
    },
};

struct DummyFrameDeallocator();

impl FrameDeallocator<Size4KiB> for DummyFrameDeallocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {}
}

pub unsafe fn init(args: &mut KernelArgs) {
    info!("Clearing old page tables");

    // FIXME Better implementation

    let mut pt = get_pt();

    let range = PageRange {
        start: Page::containing_address(VirtAddr::new(0)),
        end: Page::containing_address(VirtAddr::new(KERNEL_MAP_OFFSET)),
    };

    unmap_range(range);

    pt.clean_up_addr_range(
        PageRangeInclusive {
            start: range.start,
            end: range.end - 1,
        },
        &mut (DummyFrameDeallocator()),
    );

    info!("Initializing physical memory allocator");

    init_phys_alloc_from_mmap(args.mmap.iter());

    info!("Initializing liballoc");

    init_liballoc();
}
