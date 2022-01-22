use bitflags::bitflags;
use core::mem::size_of;
use static_assertions::const_assert;
bitflags! {
    pub struct VolumeFlags: u16 {
        const ActiveFAT = 1;
        const VolumeDirty = 2;
        const MediaFailure = 4;
        const ClearToZero = 8;
    }
}

#[repr(packed)]
pub struct BootSector {
    jump_boot: [u8; 3],
    file_system_name: [u8; 8],
    must_be_zero: [u8; 53],
    partition_offset: u64,
    volume_length: u64,
    fat_offset: u32,
    fat_length: u32,
    cluster_heap_offset: u32,
    cluster_count: u32,
    first_cluster_of_root_directory: u32,
    volume_serial_number: u32,
    file_system_revision: u16,
    volume_flags: VolumeFlags,
    bytes_per_sector_shift: u8,
    sectors_per_cluster_shift: u8,
    number_of_fats: u8,
    drive_select: u8,
    percent_in_use: u8,
    _reserved: [u8; 7],
    boot_code: [u8; 390],
    boot_signature: u16,
}

const_assert!(size_of::<BootSector>() == 512);
