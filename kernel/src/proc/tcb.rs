use alloc::{boxed::Box, string::String, vec::Vec};
use x86_64::structures::paging::PageTable;

pub struct TaskControlBlock {
    name: String,
    cr3: Box<PageTable>,
    k_stack: Vec<u16>,
    pid: u64,
}
