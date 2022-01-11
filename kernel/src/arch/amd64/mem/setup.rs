use boot_lib::*;
use core::mem::{replace, size_of, swap, take, MaybeUninit};
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
    ptr::{null, null_mut, NonNull},
};
use uefi::{
    table::{Runtime, SystemTable},
    ResultExt,
};

use crate::{
    arch::{interrupt, mem::get_pt},
    graphics::fb::{FrameBuffer, GLOBAL_FB},
    mm::{
        alloc::{
            phys::init_phys_alloc_from_mmap,
            virt::{
                alloc_and_map_at, alloc_and_map_at_range, KERNEL_MAP_OFFSET
            },
        },
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

#[inline(always)]
pub unsafe fn init(args: &mut KernelArgs) {
    info!("Initializing interrupts");

    interrupt::idt::init();

    info!("Initializing physical memory allocator");

    init_phys_alloc_from_mmap(args.mmap.iter());

    info!("Clearing UEFI page tables");

    get_pt().clean_up_addr_range(
        PageRangeInclusive {
            start: Page::containing_address(VirtAddr::new(0)),
            end: Page::containing_address(VirtAddr::new(KERNEL_MAP_OFFSET - 1)),
        },
        &mut (DummyFrameDeallocator()),
    );

    flush_all();

    info!("Fixing UEFI memory map");

    args.mmap
        .iter_mut()
        .for_each(|x| x.virt_start = x.phys_start + PHYS_MAP_OFFSET);

    SYSTEM_MEMORY_MAP.lock().init(&mut args.mmap);

    args.uefi_rst = ((&mut args.uefi_rst) as *mut SystemTable<Runtime>)
        .read()
        .set_virtual_address_map(
            args.mmap.as_mut_slice(),
            args.uefi_rst.get_current_system_table_addr() + PHYS_MAP_OFFSET,
        )
        .expect_success("Setting UEFI memory map failed");

    info!("Initializing framebuffer");

    GLOBAL_FB.lock().init(FrameBuffer::new(
        NonNull::new_unchecked((args.fb.as_mut_ptr() as u64 + PHYS_MAP_OFFSET) as *mut _),
        args.fb.size() as u64,
    ));
}
