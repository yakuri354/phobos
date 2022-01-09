use core::ptr::NonNull;
use x86_64::structures::paging::{Page, PhysFrame};
use x86_64::VirtAddr;
use crate::arch::mem::PHYS_MAP_OFFSET;
use crate::arch::PhysAddr;
use crate::arch::PAGE_SIZE;

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
        if start.is_aligned(PAGE_SIZE as u64) {
            Some(PageFrameRange { start, pages })
        } else {
            None
        }
    }
    pub fn start(&self) -> PhysAddr {
        self.start
    }
    pub fn page_count(&self) -> usize {
        self.pages
    }
    pub fn end(&self) -> PhysAddr {
        self.start + self.pages * PAGE_SIZE
    }
}

#[repr(transparent)]
pub struct SystemMemoryInfo(pub [SystemMemoryDescriptor; 512]);

pub trait Pointable {
    fn pointer(&self) -> NonNull<u8>;
    fn from_pointer(ptr: NonNull<u8>) -> Self;
}
impl Pointable for PhysAddr {
    fn pointer(&self) -> NonNull<u8> {
        NonNull::new((self.as_u64() + PHYS_MAP_OFFSET) as _).unwrap()
    }

    fn from_pointer(ptr: NonNull<u8>) -> Self {
        PhysAddr::new((ptr.as_ptr() as _) - PHYS_MAP_OFFSET)
    }
}

impl Pointable for PhysFrame {
    fn pointer(&self) -> NonNull<u8> {
        self.start_address().pointer()
    }

    fn from_pointer(ptr: NonNull<u8>) -> Self {
        PhysFrame::containing_address(PhysAddr::from_pointer(ptr))
    }
}

impl Pointable for VirtAddr {
    fn pointer(&self) -> NonNull<u8> {
        NonNull::new(self.as_ptr() as _).unwrap()
    }

    fn from_pointer(ptr: NonNull<u8>) -> Self {
        VirtAddr::from_ptr(ptr.as_ptr())
    }
}

impl Pointable for Page {
    fn pointer(&self) -> NonNull<u8> {
        self.start_address().pointer()
    }

    fn from_pointer(ptr: NonNull<u8>) -> Self {
        Page::containing_address(VirtAddr::from_pointer(ptr))
    }
}