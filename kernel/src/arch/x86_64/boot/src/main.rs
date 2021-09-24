#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(asm)]

use core::mem::size_of;
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

use log::{debug, error, info};
use uefi::Event;
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};
use uefi_services::*;

pub mod paging;

#[entry]
fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table)?;
    system_table.stdout().reset(false)?;

    info!("phobos x86_64 UEFI bootloader v{}", env!("CARGO_PKG_VERSION"));
    let rev = system_table.uefi_revision();
    info!("UEFI v{}.{}", rev.major(), rev.minor());
    info!("Enumerating files");
    let fs = unsafe {
        &mut *system_table
            .boot_services()
            .get_image_file_system(handle.clone())
            .expect_success("Failed to open FS").get()
    };

    let mut dir = fs
        .open_volume()
        .expect_success("Failed to open root directory");
    let mut buf = [0; 128];

    while let Some(file) = dir
        .read_entry(&mut buf)
        .expect_success("Failed to read directory") {
        info!("--> {}", file.file_name())
    }

    let console = unsafe {
        &*system_table
            .boot_services()
            .locate_protocol::<uefi::proto::console::text::Input>()
            .expect_success("Could not locate Input").get()
    };

    info!("Press any key to continue...");

    system_table
        .boot_services()
        .wait_for_event(&mut [console.wait_for_key_event()]);

    // TODO Un-hardcode `kernel`
    let mut k_fd = dir
        .open("kernel", FileMode::Read, FileAttribute::empty())
        .expect_success("Kernel executable not found");

    let page_tables = system_table.boot_services().allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 4);

    let mmap_size = system_table
        .boot_services()
        .memory_map_size();
    let mmap_buf = unsafe {
        &mut *slice_from_raw_parts_mut(
            system_table
                .boot_services()
                .allocate_pool(MemoryType::LOADER_DATA, mmap_size)
                .expect_success("Failed to allocate memory map buffer"),
            mmap_size,
        )
    };

    //info!("Reading memory map");
    // let (mmap_key, mut mmap_it) = system_table
    //     .boot_services()
    //     .memory_map(mmap_buf)
    //     .expect_success("Failed to get memory map");
    // for desc in mmap_it {
    //     info!("--> {:?}", desc)
    // }

    let (rt, mmap_it) = system_table
        .exit_boot_services(handle, mmap_buf)
        .expect_success("Failed to exit EFI boot services");



    unsafe { asm!("hlt"); };

    Status::SUCCESS
}
