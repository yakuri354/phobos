//! The virtual memory manager is responsible managing pages, etc.

use crate::mm::alloc::buddy::BuddyAlloc;

pub struct VirtualMemoryAllocator {
    phys_alloc: BuddyAlloc
}

impl VirtualMemoryAllocator {

}