use crate::arch::PhysAddr;
use crate::arch::FRAME_SIZE;

pub mod alloc;

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
    // TODO: Struct PageFrame
    start: PhysAddr,
    pages: usize,
}

impl PageFrameRange {
    pub fn new(start: PhysAddr, pages: usize) -> Option<PageFrameRange> {
        if start.is_aligned(FRAME_SIZE as u64) {
            Some(PageFrameRange { start, pages })
        } else {
            None
        }
    }
    pub fn start(&self) -> PhysAddr { self.start }
    pub fn page_count(&self) -> usize { self.pages }
    pub fn end(&self) -> PhysAddr { self.start + self.pages * FRAME_SIZE }
}

#[repr(transparent)]
pub struct SystemMemoryInfo(pub [SystemMemoryDescriptor; 512]);
