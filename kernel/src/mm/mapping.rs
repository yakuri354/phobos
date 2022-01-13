use crate::{arch::mem::get_pt, data::misc::Pointable};
use core::convert::TryFrom;
use log::info;
use x86_64::{
    align_down, align_up,
    instructions::tlb::flush_all,
    structures::paging::{
        mapper::{FlagUpdateError, MapperFlushAll},
        page::PageRange,
        page_table::{PageTableEntry, PageTableLevel},
        Mapper, Page, PageSize, PageTable, PageTableFlags, PageTableIndex, Size1GiB, Size4KiB,
    },
    VirtAddr,
};

const V_ADDR_MASK: u64 = 0x0000FFFFFFFFFFFF;

fn int_to_ptl(i: u64) -> Option<PageTableLevel> {
    match i {
        1 => Some(PageTableLevel::One),
        2 => Some(PageTableLevel::Two),
        3 => Some(PageTableLevel::Three),
        4 => Some(PageTableLevel::Four),
        _ => None,
    }
}

unsafe fn unmap_level(level: u64, range: PageRange) {
    let step = Size4KiB::SIZE * 512_u64.pow(level as u32 - 1);
    let start_al = align_up(range.start.start_address().as_u64() & V_ADDR_MASK, step);
    let end_al = align_down(range.end.start_address().as_u64() & V_ADDR_MASK, step);
    let mut pt = get_pt();
    if start_al < end_al {
        for ent in (start_al..end_al).step_by(step as _) {
            let addr = VirtAddr::new(ent);
            let mut pt_entry = &mut pt.level_4_table()[addr.p4_index()];
            for i in 1..=(4 - level) {
                pt_entry = &mut pt_entry.addr().pointer().cast::<PageTable>().as_mut()
                    [addr.page_table_index(int_to_ptl(4 - i).unwrap())]
            }
            pt_entry.set_unused();
        }
    }

    if start_al > range.start.start_address().as_u64() & V_ADDR_MASK {
        unmap_level(
            level - 1,
            PageRange {
                start: range.start,
                end: Page::containing_address(VirtAddr::new(start_al)),
            },
        )
    }

    if end_al < range.end.start_address().as_u64() & V_ADDR_MASK {
        unmap_level(
            level - 1,
            PageRange {
                start: Page::containing_address(VirtAddr::new(end_al)),
                end: range.end,
            },
        )
    }
}

pub unsafe fn unmap_range(range: PageRange) {
    unmap_level(4, range);
    flush_all();
}
