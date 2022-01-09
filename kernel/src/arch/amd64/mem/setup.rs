use crate::mm::{PageFrameRange, SystemMemoryInfo};
use boot_lib::*;
use core::borrow::Borrow;
use core::mem::size_of;
use core::ops::{Deref, DerefMut};
use log::{debug, info};
use uefi::table::boot::{MemoryAttribute, MemoryDescriptor, MemoryType};
use x86_64::structures::paging::page::{PageRange, PageRangeInclusive};
use x86_64::structures::paging::{
    Page, PageTable, PageTableFlags, PageTableIndex, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

use super::PAGE_SIZE;
use crate::arch::mem::PHYS_MAP_OFFSET;
use crate::mm::alloc::setup::{BootMemAllocator, BumpAlloc};
use core::alloc::Layout;
use core::ptr::NonNull;
use uefi::ResultExt;
use x86_64::registers::control::{Cr3, Cr4};
use crate::mm::alloc::alloc_page;

pub unsafe fn map_pages_to_temp_table(
    pages: PageRange<Size4KiB>,
    table: &mut PageTable,
    parent_flags: PageTableFlags,
    flags: PageTableFlags,
    alloc: &mut dyn BootMemAllocator,
) {
    for page in pages {
        map_page_to_temp_table(page, table, parent_flags, flags, alloc);
    }
}

pub unsafe fn map_page_to_temp_table(
    page: Page<Size4KiB>,
    mut pml4: &mut PageTable,
    parent_flags: PageTableFlags,
    flags: PageTableFlags,
    alloc: &mut dyn BootMemAllocator,
) {
    for i in 1..=4 {
        let idx = match i {
            1 => page.p4_index(),
            2 => page.p3_index(),
            3 => page.p2_index(),
            4 => page.p1_index(),
            _ => panic!("Paging level index out of bounds"),
        };
        let entry = &mut pml4[idx];
        let loc_flags = match i {
            4 => flags,
            _ => parent_flags,
        };
        if !entry.flags().contains(PageTableFlags::PRESENT) {
            entry.set_addr(
                PhysAddr::new(
                    NonNull::new(
                        alloc.alloc_raw(Layout::from_size_align(PAGE_SIZE, PAGE_SIZE).unwrap()),
                    )
                    .expect("Could not allocate memory for a page table")
                    .as_ptr() as _,
                ),
                loc_flags,
            );
        } else {
            entry.set_flags(loc_flags)
        }
        if i != 4 {
            pml4 = &mut *(entry.addr().as_u64() as *mut _)
        }
    }
}

unsafe fn recurse_pml4(pml4: &mut PageTable, idx: u16) {
    let ptr = pml4 as *const _ as u64;
    pml4[PageTableIndex::new(idx)].set_addr(
        PhysAddr::new(ptr),
        PageTableFlags::empty()
            | PageTableFlags::PRESENT
            | PageTableFlags::WRITABLE
            | PageTableFlags::NO_EXECUTE
            | PageTableFlags::NO_CACHE,
    );
}

pub unsafe fn init(args: &mut KernelArgs) {
    info!("Creating new paging tables");

    debug!("Args: {:?}", args);

    let temp_alloc_region = args
        .mmap
        .iter()
        .filter(|d| d.ty == MemoryType::CONVENTIONAL)
        .max_by(|a, b| a.page_count.cmp(&b.page_count))
        .expect("No usable memory found, aborting");

    debug!(
        "Initializing WatermarkAlloc with desc {:?}",
        temp_alloc_region
    );

    let mut bm_alloc = BumpAlloc::new(
        temp_alloc_region.phys_start as _,
        (temp_alloc_region.phys_start + temp_alloc_region.page_count * 4096) as _,
    );

    debug!("Allocating new PML4");

    let pml4_page = alloc_page();

    let pml4 = &mut *(pml4_page.as_ptr() as *mut PageTable);

    debug!("Setting up recursive paging");

    recurse_pml4(pml4, 511);

    debug!("Allocating new UEFI memory map");

    let new_desc = &mut *(bm_alloc
        .alloc_zeroed(Layout::from_size_align(size_of::<MemoryDescriptor>() * 512, 4096).unwrap())
        .as_ptr() as *mut [MemoryDescriptor; 512]);

    info!("Remapping kernel memory");

    for (i, desc) in args.mmap.iter().enumerate() {
        new_desc[i] = *desc;
        // Addresses reserved for kernel
        if desc.ty.0 > KERNEL_MEM_TYPE_RANGE_START {
            let flags = match desc.ty.0 {
                KERNEL_RO_MEM_TYPE => PageTableFlags::NO_EXECUTE,
                KERNEL_RX_MEM_TYPE => PageTableFlags::empty(),
                KERNEL_RW_MEM_TYPE => PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE,
                KERNEL_RWX_MEM_TYPE => PageTableFlags::WRITABLE,
                _ => continue,
            };

            map_pages_to_temp_table(
                PageRange {
                    start: Page::from_start_address(VirtAddr::new(desc.virt_start))
                        .expect("Memory map VA unaligned"),
                    end: Page::from_start_address(VirtAddr::new(
                        desc.virt_start + desc.page_count * PAGE_SIZE as u64,
                    ))
                    .unwrap(),
                },
                pml4,
                PageTableFlags::PRESENT | PageTableFlags::GLOBAL | PageTableFlags::WRITABLE,
                PageTableFlags::PRESENT | PageTableFlags::GLOBAL | flags,
                &mut bm_alloc,
            )
        } else if desc.att.contains(MemoryAttribute::RUNTIME) {
            new_desc[i].virt_start = desc.phys_start + PHYS_MAP_OFFSET as u64;

            map_pages_to_temp_table(
                PageRange {
                    start: Page::from_start_address(VirtAddr::new(
                        desc.phys_start + PHYS_MAP_OFFSET as u64,
                    ))
                    .expect("Memory map VA unaligned"),
                    end: Page::from_start_address(VirtAddr::new(
                        desc.phys_start
                            + PHYS_MAP_OFFSET as u64
                            + desc.page_count * PAGE_SIZE as u64,
                    ))
                    .unwrap(),
                },
                pml4,
                PageTableFlags::PRESENT | PageTableFlags::GLOBAL | PageTableFlags::WRITABLE,
                PageTableFlags::PRESENT | PageTableFlags::GLOBAL | PageTableFlags::WRITABLE,
                &mut bm_alloc,
            )
        }
    }

    info!("Setting UEFI virtual map");

    args.uefi_rst
        .runtime_services()
        .set_virtual_address_map(new_desc)
        .expect_success("Failed to set UEFI virtual memory map");

    info!("Setting new PML4 address into CR3");

    let (_, flags) = Cr3::read();
    Cr3::write(
        PhysFrame::from_start_address(PhysAddr::new(pml4_page.as_ptr() as _))
            .expect("PML4 unaligned"),
        flags,
    );

    info!("Creating new kernel stack and heap")
}
