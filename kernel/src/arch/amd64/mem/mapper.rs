use x86_64::structures::paging::{Page, PageTable, PhysFrame};
use x86_64::VirtAddr;

// TODO: Efficient mapping of virtual address space using huge- and super-pages

pub fn map_region(table: &mut PageTable, phys: PhysFrame, virt: Page, count: usize) {
    todo!()
}

fn log_512_floor(x: usize) -> usize {
    todo!()
}
