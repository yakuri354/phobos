use crate::arch::PhysAddr;

pub mod alloc;

pub const PAGE_SIZE: usize = 0x1000;

pub enum SystemMemoryType {
    /// UEFI Runtime Services, PAL, etc.
    SystemReserved,
    /// Memory that holds ACPI tables
    ACPITables,
    /// Memory available for allocations
    Available,
    /// Faulty or otherwise unusable memory
    Unusable,
}

pub struct SystemMemoryDescriptor {
    pub ty: SystemMemoryType,
    pub range: PageFrameRange,
}

pub struct PageFrameRange {
    // TODO Struct PageFrame
    pub start: PhysAddr,
    pub pages: usize,
}

#[repr(transparent)]
pub struct SystemMemoryInfo(pub [SystemMemoryDescriptor; 512]);
