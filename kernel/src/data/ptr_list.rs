use core::ptr::null_mut;

/// A raw intrusive linked list implementation
pub struct PointerList {
    head: *mut usize,
}

impl PointerList {
    /// Create an empty PointerList
    pub fn new() -> PointerList {
        PointerList {
            head: null_mut(),
        }
    }
    /// Push a pointer to the head of the list
    pub unsafe fn push(&mut self, ptr: *mut usize) {
        *ptr = self.head as _;
        self.head = ptr;
    }
    /// Try to pop the head of the list
    pub fn pop(&mut self) -> Option<*mut usize> {
        if self.head.is_null() {
            None
        } else {
            let prev = self.head;
            self.head = unsafe { *prev as _ };
            Some(prev)
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }
}

struct PointerListIterator {
    
}