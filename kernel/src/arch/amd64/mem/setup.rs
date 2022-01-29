use boot_lib::*;

use log::info;

use x86_64::{
    structures::paging::{page::PageRange, Page, PhysFrame, Size4KiB},
    VirtAddr,
};

use crate::{
    arch::mem::get_pt,
    mm::{
        alloc::{init_liballoc, phys::init_phys_alloc_from_mmap, virt::KERNEL_MAP_OFFSET},
        mapping::unmap_range,
    },
};

use x86_64::structures::paging::{mapper::CleanUp, page::PageRangeInclusive, FrameDeallocator};

struct DummyFrameDeallocator();

impl FrameDeallocator<Size4KiB> for DummyFrameDeallocator {
    unsafe fn deallocate_frame(&mut self, _frame: PhysFrame<Size4KiB>) {}
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
