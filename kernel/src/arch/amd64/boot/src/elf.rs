use crate::UefiAlloc;
use alloc::vec::Vec;
use boot_lib::*;
use core::{mem::transmute, panic};
use elf_rs::{Elf64, ElfFile, ProgramHeaderFlags};
use log::{debug, info};
use uefi::{
    table::{
        boot::{AllocateType, MemoryAttribute, MemoryDescriptor, MemoryType},
        Boot, SystemTable,
    },
    ResultExt,
};
use x86_64::{
    structures::paging::{Mapper, Page, PageSize, PageTableFlags, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

fn count_pages_needed(size: u64) -> u64 {
    let rem = size % Size4KiB::SIZE;
    if rem == 0 {
        size / Size4KiB::SIZE
    } else {
        ((size - rem) / Size4KiB::SIZE) + 1
    }
}

pub fn map_elf<M>(raw: &[u8], mapper: &mut M, st: &mut SystemTable<Boot>) -> KernelEntryPoint
where
    M: Mapper<Size4KiB>,
{
    info!("Kernel size: {}", raw.len());
    match Elf64::from_bytes(raw) {
        Ok(elf) => {
            let mut alloc = UefiAlloc {};

            for ph in elf.program_header_iter() {
                match ph.ph_type() {
                    elf_rs::ProgramType::LOAD => {
                        debug!("Loading a program header --> {:?}", ph);
                        let pages = count_pages_needed(ph.memsz());

                        debug!("Allocating {} pages", pages);

                        let mut mem_ty = KERNEL_RO_MEM_TYPE;

                        let mut page_flags = PageTableFlags::empty()
                            | PageTableFlags::PRESENT
                            | PageTableFlags::WRITABLE;

                        // if !ph.flags().contains(ProgramHeaderFlags::EXECUTE) {
                        //     page_flags |= PageTableFlags::NO_EXECUTE;
                        // } else {
                        //     mem_ty = KERNEL_RX_MEM_TYPE;
                        // }

                        if ph.flags().contains(ProgramHeaderFlags::WRITE) {
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

                        debug!("Zeroing allocated pages");
                        unsafe {
                            pages_addr.write_bytes(0, ph.memsz() as usize);
                        }
                        debug!("Copying the header");
                        let raw_ptr = raw.as_ptr();
                        unsafe {
                            raw_ptr
                                .offset(ph.offset() as isize)
                                .copy_to_nonoverlapping(pages_addr, ph.filesz() as usize)
                        }

                        for i in 0..pages {
                            unsafe {
                                let vaddr = VirtAddr::new(ph.vaddr() + i * Size4KiB::SIZE);
                                let paddr = PhysAddr::new(
                                    pages_addr.offset((i * Size4KiB::SIZE) as isize) as u64,
                                );
                                mapper
                                    .map_to_with_table_flags(
                                        Page::from_start_address(vaddr)
                                            .expect("Allocated page not aligned"),
                                        PhysFrame::from_start_address(paddr)
                                            .expect("Allocated page not aligned"),
                                        page_flags,
                                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                                        &mut alloc,
                                    )
                                    .unwrap()
                                    .flush()
                            }
                        }
                    }
                    _ => {}
                }
            }

            info!("Kernel entry point at {:#x}", elf.entry_point());
            unsafe { transmute(elf.entry_point() as *const ()) }
        }
        Err(e) => panic!("Kernel image is not a valid ELF file: {:?}", e),
    }
}
