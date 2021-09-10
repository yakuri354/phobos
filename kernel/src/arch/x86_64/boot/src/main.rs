#![no_std]
#![no_main]
#![feature(abi_efiapi)]
use uefi::prelude::*;
use uefi_services::*;
use log::{info, error, debug};

#[entry]
fn efi_main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&system_table)?;
    system_table.stdout().reset(false)?;

    info!("--> phobos x86_64 UEFI bootloader v{}", env!("CARGO_PKG_VERSION"));
    let rev = system_table.uefi_revision();
    info!("--> UEFI v{}.{}", rev.major(), rev.minor());

    Status::SUCCESS
}