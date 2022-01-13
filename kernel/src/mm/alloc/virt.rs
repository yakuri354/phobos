//! The virtual memory manager is responsible for managing pages, etc.

use crate::{
    arch::{mem::get_pt, PAGE_SIZE},
    data::misc::Pointable,
    mm::{
        alloc::{
            phys::{clear_page, GlobalFrameAllocator, GLOBAL_PHYS_ALLOC},
            stack::StackAllocator,
            virt::VAllocError::CannotCommit,
            SinglePageAllocator,
        },
        aux::_1_GiB_PAGE,
    },
    sync::irq_lock::IRQLocked,
};
use bitflags::bitflags;
use boot_lib::PHYS_MAP_OFFSET;
use core::{cmp::Ordering, ops::Add, ptr::NonNull};
use log::info;
use memrange::Range;
use theban_interval_tree::IntervalTree;
use x86_64::{
    align_down, align_up,
    structures::paging::{
        page::{PageRange, PageRangeInclusive},
        Mapper, Page, PageSize, PageTableFlags, PhysFrame, Size1GiB, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

pub const VAD_ALIGN: u64 = 0x10000;
pub const USER_VIRT_SPACE_START: u64 = 0x0000000000010000;
pub const KERNEL_VIRT_SPACE_START: u64 = 0xFFFF800000000000;
pub const USER_VIRT_SPACE_END: u64 = 0x00007FFFFFFF0000;
pub const KERNEL_VIRT_SPACE_END: u64 = 0xFFFFFFFFFFFFF000;
pub const KERNEL_MAP_OFFSET: u64 = 0xFFFFFFE000000000;

// pub const GLOBAL_VM_ALLOC: Locked<KernelVASpace> = Locked::new(KernelVASpace::new());
pub const GLOBAL_VM_ALLOC: IRQLocked<SimpleVaSpace> =
    IRQLocked::new(SimpleVaSpace::new(1, KERNEL_MAP_OFFSET / Size4KiB::SIZE));

enum VAllocError {
    NotEnoughSpace,
    CannotCommit,
    BadParameter,
}

bitflags! {
    pub struct VAllocFlags: u32 {
        const RESERVE = 1;
        const COMMIT = 2;
    }
}

struct VAddrDescriptor();

struct KernelVASpace {
    // TODO Non-paged & Paged kernel pools
    tree: IntervalTree<VAddrDescriptor>,
    // TODO This should have a non-locking allocator
    hint: u64,
}

impl KernelVASpace {
    pub const fn new() -> Self {
        Self {
            tree: IntervalTree::new(),
            hint: 0,
        }
    }

    pub fn alloc(
        &mut self,
        pages: u64,
        flags: VAllocFlags,
        prot: PageTableFlags,
    ) -> Result<PageRange, VAllocError> {
        if let Some(space) = self.find_free_space(pages) {
            let vad = VAddrDescriptor();

            let range = PageRange {
                start: Page::containing_address(VirtAddr::new(
                    space.min * Size4KiB::SIZE + KERNEL_VIRT_SPACE_START,
                )),
                end: Page::containing_address(VirtAddr::new(
                    space.max * Size4KiB::SIZE + KERNEL_VIRT_SPACE_START,
                )),
            };

            if flags.contains(VAllocFlags::COMMIT) {
                // TODO Demand paging
                alloc_and_map_at_range(range, prot)
            }

            self.tree.insert(space, vad);

            Ok(range)
        } else {
            Err(VAllocError::NotEnoughSpace)
        }
    }

    pub fn alloc_at(&mut self, mut base: NonNull<u8>, mut pages: u64) -> Option<PageRange> {
        base = NonNull::new(align_down(base.as_ptr() as _, VAD_ALIGN) as _)?;
        pages = align_up(pages, 16);

        todo!()
    }

    fn find_free_space(&mut self, pages: u64) -> Option<memrange::Range> {
        let size = align_up(pages, 16); // Align to 64K boundary
        if self.tree.empty() {
            if KERNEL_VIRT_SPACE_START + size * Size4KiB::SIZE > KERNEL_VIRT_SPACE_END {
                return None;
            }
            self.hint = size;
            Some(Range::new(0, size - 1))
        } else {
            let mut latest = self.hint;
            for (r, _) in self.tree.range(self.hint, u64::MAX) {
                if r.min - latest >= size {
                    self.hint = latest + size;
                    return Some(Range::new(latest, latest + size - 1));
                } else {
                    latest = r.max + 1;
                }
            }
            if KERNEL_VIRT_SPACE_END - KERNEL_VIRT_SPACE_START - latest >= size {
                self.hint = latest + size;
                return Some(Range::new(latest, latest + size - 1));
            }

            None
        }
    }
}

pub fn alloc_and_map_at_range(range: PageRange, flags: PageTableFlags) {
    alloc_and_map_at(
        range.start.start_address(),
        (range.end.start_address() - range.start.start_address()) / Size4KiB::SIZE,
        flags,
    )
}

pub fn alloc_and_map_at(virt: VirtAddr, pages: u64, flags: PageTableFlags) {
    assert!(virt.is_aligned(Size4KiB::SIZE));
    for page in 0..pages {
        if let Some(frame) = GLOBAL_PHYS_ALLOC.lock().get_clean() {
            unsafe {
                get_pt()
                    .map_to_with_table_flags(
                        Page::<Size4KiB>::containing_address(virt + page * Size4KiB::SIZE),
                        PhysFrame::containing_address(PhysAddr::new(
                            frame.as_ptr() as u64 - PHYS_MAP_OFFSET,
                        )),
                        flags,
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                        &mut GlobalFrameAllocator,
                    )
                    .ok()
                    .expect("Mapping failed")
                    .flush()
            };
        } else {
            panic!("Physical OOM"); // TODO Swap
        }
    }
}

unsafe impl Send for KernelVASpace {}

pub struct SimpleVaSpace {
    curr: u64,
    end: u64,
}

impl SimpleVaSpace {
    // TODO Rewrite this abomination
    pub const fn new(curr: u64, end: u64) -> Self {
        Self { curr, end }
    }

    pub fn alloc(
        &mut self,
        pages: u64,
        flags: VAllocFlags,
        prot: PageTableFlags,
    ) -> Option<PageRange> {
        self.alloc_aligned(pages, 1, flags, prot)
    }

    pub fn alloc_aligned(
        &mut self,
        pages: u64,
        align_pages: u64,
        flags: VAllocFlags,
        prot: PageTableFlags,
    ) -> Option<PageRange> {
        self.curr = align_up(self.curr, align_pages);
        if self.curr + pages >= self.end {
            None
        } else {
            let old = self.curr;
            self.curr += pages;
            let range = PageRange {
                start: Page::containing_address(VirtAddr::new(old * Size4KiB::SIZE)),
                end: Page::containing_address(VirtAddr::new(self.curr * Size4KiB::SIZE)),
            };
            if flags.contains(VAllocFlags::COMMIT) {
                // TODO Demand paging
                alloc_and_map_at_range(range, prot)
            }
            Some(range)
        }
    }

    pub unsafe fn free(&mut self, addr: VirtAddr, count: u64) {
        self.free_range(PageRange {
            start: Page::containing_address(addr),
            end: Page::containing_address(addr + Size4KiB::SIZE * count),
        })
    }

    pub unsafe fn free_range(&mut self, range: PageRange) {
        let pt = get_pt();
        for page in range {
            GLOBAL_PHYS_ALLOC.lock().dirty.push(
                pt.translate_page(page)
                    .expect("Bad virtual address freed")
                    .start_address()
                    .pointer(),
            )
        }
    }
}
