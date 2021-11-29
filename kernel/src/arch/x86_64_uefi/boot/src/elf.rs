use crate::UefiAlloc;
use alloc::vec::Vec;
use boot_ffi::*;
use core::mem::transmute;
use core::panic;
use goblin::elf::Elf;
use log::{debug, info};
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};
use uefi::table::cfg::MEMORY_TYPE_INFORMATION_GUID;
use uefi::table::{Boot, SystemTable};
use uefi::ResultExt;
use x86_64::structures::paging::{
    FrameAllocator, Mapper, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

fn count_pages_needed(size: usize) -> usize {
    let rem = size % 0x1000;
    if rem == 0 {
        size / 0x1000
    } else {
        ((size - rem) / 0x1000) + 1
    }
}

/// This function can only be called with Boot Services enabled
pub fn map_elf<M>(
    raw: &[u8],
    mapper: &mut M,
    st: &mut SystemTable<Boot>,
) -> (KernelEntryPoint, Vec<MemoryDescriptor>)
where
    M: Mapper<Size4KiB>,
{
    let mut mdl = Vec::new();
    match Elf::parse(raw) {
        Ok(elf) => {
            if !elf.is_64 {
                panic!("Kernel image is 32-bit")
            }

            let mut alloc = UefiAlloc {};

            for ph in elf.program_headers {
                if ph.p_type == goblin::elf::program_header::PT_LOAD {
                    debug!("Loading a program header --> {:?}", ph);
                    let pages = count_pages_needed(ph.p_memsz as usize) as u64;

                    debug!("Allocating {} pages", pages);

                    let mut mem_ty = KERNEL_RO_MEM_TYPE;

                    let mut page_flags = PageTableFlags::empty()
                        | PageTableFlags::WRITABLE
                        | PageTableFlags::GLOBAL
                        | PageTableFlags::PRESENT;

                    if (ph.p_flags & 1) == 0 {
                        page_flags |= PageTableFlags::NO_EXECUTE;
                    } else {
                        mem_ty = KERNEL_RX_MEM_TYPE;
                    }

                    if (ph.p_flags & 2) << 1 == 1 {
                        page_flags |= PageTableFlags::WRITABLE;
                        if mem_ty == KERNEL_RX_MEM_TYPE {
                            mem_ty = KERNEL_RWX_MEM_TYPE
                        } else {
                            mem_ty = KERNEL_RW_MEM_TYPE
                        }
                    }

                    let pages_addr = st
                        .boot_services()
                        .allocate_pages(
                            AllocateType::AnyPages,
                            MemoryType::custom(mem_ty),
                            pages as usize,
                        )
                        .expect_success("Failed to allocate pages for loading the kernel")
                        as *mut u8;

                    let mut tmp = MemoryDescriptor::default();
                    tmp.ty = MemoryType::custom(mem_ty);
                    tmp.phys_start = pages_addr as _;
                    tmp.virt_start = ph.p_vaddr;
                    tmp.page_count = pages;

                    mdl.push(tmp);

                    debug!("Zeroing allocated pages");
                    unsafe {
                        pages_addr.write_bytes(0, ph.p_memsz as usize);
                    }
                    debug!("Copying the header");
                    let raw_ptr = raw.as_ptr();
                    unsafe {
                        raw_ptr
                            .offset(ph.p_offset as isize)
                            .copy_to_nonoverlapping(pages_addr, ph.p_filesz as usize)
                    }

                    for i in 0..pages {
                        unsafe {
                            let vaddr = VirtAddr::new(ph.p_vaddr + i * 0x1000);
                            let paddr =
                                PhysAddr::new(pages_addr.offset((i * 0x1000) as isize) as u64);
                            info!("Mapping kernel page {:?} to paddr {:?}", vaddr, paddr);
                            mapper
                                .map_to(
                                    Page::from_start_address(vaddr)
                                        .expect("Allocated page not aligned"),
                                    PhysFrame::from_start_address(paddr)
                                        .expect("Allocated page not aligned"),
                                    page_flags,
                                    &mut alloc,
                                )
                                .unwrap()
                                .flush()
                        }
                    }
                }
            }
            info!("Kernel entry point at {:#x}", elf.entry);
            unsafe { (transmute(elf.entry), mdl) }
        }
        Err(e) => panic!("Kernel image is not a valid ELF file"),
    }
}
