#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(asm)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::fmt::format;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::mem::size_of;
use core::ops::Not;
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

use log::{debug, error, info};
use uefi::prelude::*;
use uefi::proto::console::gop::{FrameBuffer, GraphicsOutput};
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode, RegularFile};
use uefi::table::boot::{AllocateType, MemoryAttribute, MemoryDescriptor, MemoryType};
use uefi::Event;
use uefi_services::*;
use x86_64::instructions::port::PortWriteOnly;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr3, Cr4, Efer, EferFlags};
use x86_64::registers::read_rip;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::paging::mapper::PageTableFrameMapping;
use x86_64::structures::paging::page_table::PageTableEntry;
use x86_64::structures::paging::{
    FrameAllocator, MappedPageTable, Mapper, Page, PageTable, PageTableFlags, PhysFrame,
    RecursivePageTable, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};

use alloc::vec;
use boot_lib::{KernelArgs, KERNEL_ARGS_MDL_SIZE};
use core::fmt::Write;
use uart_16550::SerialPort;

mod elf;

static K_FILE: &'static str = "kernel";

struct UefiAlloc();

unsafe impl FrameAllocator<Size4KiB> for UefiAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let addr = unsafe { uefi_services::system_table().as_mut() }
            .boot_services()
            .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1)
            .expect_success("Failed to allocate a page");
        Some(PhysFrame::from_start_address(PhysAddr::new(addr)).unwrap())
    }
}

#[entry]
fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table).expect_success("Failed to setup UEFI services");
    system_table
        .stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");

    info!(
        "phobos x86_64 UEFI bootloader v{}",
        env!("CARGO_PKG_VERSION")
    );
    let rev = system_table.uefi_revision();
    info!("UEFI v{}.{}", rev.major(), rev.minor());
    info!("CR0 -> {:?}", Cr0::read());
    info!("CR4 -> {:?}", Cr4::read());
    info!("EFER -> {:?}", Efer::read());
    let (pml4_frame, cr3_flags) = Cr3::read();
    info!("PML4 -> {:#x}", pml4_frame.start_address().as_u64());
    let fs = unsafe {
        &mut *system_table
            .boot_services()
            .get_image_file_system(handle.clone())
            .expect_success("Failed to open FS")
            .get()
    };

    let mut dir = fs
        .open_volume()
        .expect_success("Failed to open root directory");
    let mut buf = [0; 128];

    info!("Searching for kernel");

    let k_size = {
        let mut ks = None;
        while let Some(file) = dir
            .read_entry(&mut buf)
            .expect_success("Failed to read directory")
        {
            if file.file_name().to_string() == K_FILE {
                ks = Some(file.file_size());
                break;
            }
        }
        if ks.is_none() {
            panic!("Kernel executable not found");
        }
        ks.unwrap() as usize
    };

    info!("Reading the kernel image into a temporary pool");

    let mut k_fd = unsafe {
        RegularFile::new(
            dir.open(K_FILE, FileMode::Read, FileAttribute::empty())
                .expect_success("Kernel executable not found"),
        )
    };

    let mut k_buf = Vec::with_capacity(k_size);

    k_fd.read(&mut k_buf)
        .expect_success("Failed to read kernel file");

    k_fd.close();

    info!("Setting up recursive paging");

    let pml4 = unsafe { &mut *(pml4_frame.start_address().as_u64() as *mut PageTable) };

    const REC_IDX: usize = 511;
    const REC_ADDRESS: u64 = 0xfffffffffffff000;

    let cr0 = Cr0::read();
    unsafe { Cr0::write(cr0 & !Cr0Flags::WRITE_PROTECT) };

    pml4[REC_IDX].set_addr(
        PhysAddr::new(pml4_frame.start_address().as_u64()),
        PageTableFlags::empty()
            | PageTableFlags::PRESENT
            | PageTableFlags::WRITABLE
            | PageTableFlags::NO_EXECUTE
            | PageTableFlags::NO_CACHE,
    );

    unsafe { Cr0::write(cr0) };

    let mut rpt = RecursivePageTable::new(unsafe { &mut *(REC_ADDRESS as *mut _) })
        .expect("Recursive page table not recognized");

    info!("Mapping kernel image");

    let (entry, k_mdl) = elf::map_elf(&mut k_buf, &mut rpt, &mut system_table);

    // Remove the lowest page to trap NULL pointer dereference bugs
    unsafe {
        rpt.update_flags(
            Page::<Size4KiB>::from_start_address(VirtAddr::new(0)).unwrap(),
            PageTableFlags::empty(),
        )
        .unwrap()
        .flush();
    }

    info!("Press any key to jump into kernel...");

    let console = unsafe {
        &*system_table
            .boot_services()
            .locate_protocol::<uefi::proto::console::text::Input>()
            .expect_success("Could not locate Input")
            .get()
    };

    system_table
        .boot_services()
        .wait_for_event(&mut [console.wait_for_key_event()]);

    info!("Allocating memory map");

    let mmap_size = system_table.boot_services().memory_map_size() + 0x2000;
    let mut mmap_buf = Vec::with_capacity(mmap_size);

    info!("Initializing kernel args struct");

    let p_args = system_table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, size_of::<KernelArgs>())
        .expect_success("Failed to allocate kernel args buffer")
        as *mut KernelArgs;

    unsafe {
        p_args.write_bytes(0, 1);
    }

    let args = unsafe { &mut *p_args };

    let (_, mmap_it) = system_table
        .boot_services()
        .memory_map(&mut mmap_buf)
        .expect_success("Failed to get memory map");

    let mut final_mmap = vec![];

    'outer: for md in mmap_it {
        for kmd in &k_mdl {
            if md.ty == kmd.ty && md.phys_start == kmd.phys_start {
                final_mmap.push(*kmd);
                continue 'outer;
            }
        }
        final_mmap.push(*md);
    }

    if final_mmap.len() > KERNEL_ARGS_MDL_SIZE {
        panic!(&format!(
            "Memory map of len {} does not fit into len {}",
            final_mmap.len(),
            KERNEL_ARGS_MDL_SIZE
        ))
    }

    for i in args.mmap.iter_mut() {
        *i = MemoryDescriptor::default();
    }

    for (i, md) in final_mmap.iter().enumerate() {
        info!("{:?}", md);
        args.mmap[i] = *md;
    }

    info!(
        "Exiting boot services and calling kernel entry point at {:?}",
        entry
    );

    let (rt, _) = system_table
        .exit_boot_services(handle, &mut mmap_buf)
        .expect_success("Failed to exit UEFI boot services");

    args.uefi_rst = rt;

    (entry)(args as _);
}
