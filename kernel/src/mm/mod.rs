use crate::arch::PhysAddr;

pub mod alloc;

pub const PAGE_SIZE: usize = 0x1000;

pub enum MemoryMapType {
    /// UEFI, etc
    SystemReserved,
    MMIO,
    Available
}

pub struct PhysMemRange {
    pub start: PhysAddr,
    pub end: PhysAddr
}

pub struct MemoryMapDesc {
    ty: MemoryMapType,
    range: PhysMemRange,
}