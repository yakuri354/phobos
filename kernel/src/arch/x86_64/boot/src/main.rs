#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(asm)]

mod elf;

extern crate alloc;

use core::mem::size_of;
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

use log::{debug, error, info};
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode, RegularFile};
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};
use uefi::Event;
use uefi_services::*;
use x86_64::config_reg::*;
use x86_64::mm::paging::setup::init_page_table;
use alloc::string::ToString;

static K_FILE: &'static str = "kernel";

fn count_pages_needed(size: usize) -> usize {
    let rem = size % 0x1000;
    if rem == 0 {
        size / 0x1000
    } else {
        ((size - rem) / 0x1000) + 1
    }
}

#[entry]
fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table)?;
    system_table.stdout().reset(false)?;

    info!(
        "phobos x86_64 UEFI bootloader v{}",
        env!("CARGO_PKG_VERSION")
    );
    let rev = system_table.uefi_revision();
    info!("UEFI v{}.{}", rev.major(), rev.minor());
    info!("CR0 -> {:#b}", read_cr0());
    info!("CR4 -> {:#b}", read_cr4());
    info!("CR3 -> {:#x}", read_cr3());
    info!("Enumerating files");
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

    info!("Reading the kernel into memory");

    let mut k_fd = unsafe {
        RegularFile::new(
            dir.open(K_FILE, FileMode::Read, FileAttribute::empty())
                .expect_success("Kernel executable not found"),
        )
    };

    let k_buf = unsafe {
        &mut *slice_from_raw_parts_mut(
            system_table
                .boot_services()
                .allocate_pool(MemoryType::LOADER_DATA, k_size)
                .expect_success("Failed to allocate temporary kernel pool") as *mut u8,
            k_size,
        )
    };

    k_fd.read(k_buf).expect_success("Failed to read kernel file");

    k_fd.close();


    info!("Initializing page tables");

    let page_tables = system_table
        .boot_services()
        .allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 4)
        .expect_success("Failed to allocate page tables");



    info!("Allocating memory map");

    let mmap_size = system_table.boot_services().memory_map_size();
    let mmap_buf = unsafe {
        &mut *slice_from_raw_parts_mut(
            system_table
                .boot_services()
                .allocate_pool(MemoryType::LOADER_DATA, mmap_size)
                .expect_success("Failed to allocate memory map buffer"),
            mmap_size,
        )
    };

    let (mmap_key, mmap_it) = system_table
        .boot_services()
        .memory_map(mmap_buf)
        .expect_success("Failed to retrieve memory map");

    for map in mmap_it {
        let map: &MemoryDescriptor = map; // For the IDE autocomplete to work
        match map.ty {
            MemoryType::LOADER_DATA => {}
            _ => {}
        }
    }

    //info!("Reading memory map");
    // let (mmap_key, mut mmap_it) = system_table
    //     .boot_services()
    //     .memory_map(mmap_buf)
    //     .expect_success("Failed to get memory map");
    // for desc in mmap_it {
    //     info!("--> {:?}", desc)
    // }

    // let (rt, mmap_it) = system_table
    //     .exit_boot_services(handle, mmap_buf)
    //     .expect_success("Failed to exit EFI boot services");

    info!("Press any key to continue...");

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

    Status::SUCCESS
}
