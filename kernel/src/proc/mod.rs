use alloc::collections::LinkedList;
use alloc::string::String;
use alloc::vec::Vec;
use crate::sync::irq_lock::IRQLocked;

mod tcb;

static TASKS: IRQLocked<LinkedList<Task>> = IRQLocked::new(LinkedList::new());

struct Task {
    stack: Vec<u8>,
    name: String,
    cpu_time: u64,
}

pub fn schedule() {
    
}