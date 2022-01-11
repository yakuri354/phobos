use core::{
    mem::{size_of, MaybeUninit},
    ptr::NonNull,
};
use log::info;

/// When a movable head is needed
pub struct CDLListHead {
    node: Option<NonNull<CDLListNode>>,
}

impl CDLListHead {
    pub fn new() -> Self {
        Self { node: None }
    }

    pub unsafe fn push(&mut self, new: NonNull<CDLListNode>) {
        match self.node {
            Some(mut node) => {
                node.as_mut().push_back(new);
            }
            None => {
                CDLListNode::init(new.cast());
            }
        }
        self.node = Some(new)
    }

    pub fn pop(&mut self) -> Option<NonNull<()>> {
        match self.node {
            None => None,
            Some(mut a) => {
                unsafe { a.as_mut().remove() }
                Some(a.cast())
            }
        }
    }

    pub fn peek(&self) -> Option<NonNull<CDLListNode>> {
        self.node
    }
}

/// A circular doubly linked list node
pub struct CDLListNode {
    next: NonNull<CDLListNode>,
    prev: NonNull<CDLListNode>,
}

// FIXME: Should `self` be a reference or a NonNull?

impl CDLListNode {
    /// Create an empty ListNode
    #[inline]
    pub fn new() -> MaybeUninit<Self> {
        MaybeUninit::uninit()
    }

    /// Initialize the list head
    #[inline]
    pub unsafe fn init(mut new: NonNull<MaybeUninit<Self>>) {
        unsafe {
            new.as_mut().write(CDLListNode {
                next: new.cast(),
                prev: new.cast(),
            });
        }
    }

    /// Push an entry next to itself
    #[inline]
    pub unsafe fn push_next(&mut self, mut ptr: NonNull<CDLListNode>) {
        *ptr.as_mut() = CDLListNode {
            next: self.next,
            prev: NonNull::from(&*(self as *const _)),
        };
        self.next = ptr;
    }

    /// Push an entry before itself
    #[inline]
    pub unsafe fn push_back(&mut self, mut ptr: NonNull<CDLListNode>) {
        *ptr.as_mut() = CDLListNode {
            next: NonNull::from(&*(self as *const _)),
            prev: self.prev,
        };
        self.prev = ptr;
    }

    /// Pop the next list entry (no-op if the list contains only 1 entry)
    #[inline]
    pub fn pop_next_unchecked(&mut self) -> NonNull<()> {
        let popped = self.next;
        unsafe {
            self.next = self.next.as_ref().next;
        }
        popped.cast()
    }

    #[inline]
    pub fn pop_next(&mut self) -> Option<NonNull<()>> {
        if self.next == unsafe { NonNull::from(&*(self as *const _)) } {
            None
        } else {
            Some(self.pop_next_unchecked())
        }
    }

    /// Pop the previous entry (no-op if the list contains only 1 entry)
    #[inline]
    pub fn pop_back_unchecked(&mut self) -> NonNull<()> {
        let popped = self.prev;
        unsafe {
            self.prev = self.prev.as_ref().prev;
        }
        popped.cast()
    }

    #[inline]
    pub fn pop_back(&mut self) -> Option<NonNull<()>> {
        if self.prev == unsafe { NonNull::from(&*(self as *const _)) } {
            None
        } else {
            Some(self.pop_back_unchecked())
        }
    }

    #[inline]
    pub fn remove(&mut self) {
        unsafe {
            self.next.as_mut().prev = self.prev;
            self.prev.as_mut().next = self.next;
        }

        unsafe {
            *self = core::mem::zeroed();
        }
    }

    #[inline]
    pub fn peek_unchecked(&self) -> NonNull<CDLListNode> {
        self.next
    }

    #[inline]
    pub fn peek(&self) -> Option<NonNull<CDLListNode>> {
        if self.next == NonNull::from(self) {
            None
        } else {
            Some(self.peek_unchecked())
        }
    }

    #[inline]
    pub fn peek_prev_unchecked(&self) -> NonNull<CDLListNode> {
        self.prev
    }

    #[inline]
    pub fn peek_prev(&self) -> Option<NonNull<CDLListNode>> {
        if self.prev == NonNull::from(self) {
            None
        } else {
            Some(self.peek_prev_unchecked())
        }
    }

    // #[inline]
    // pub fn iter(&self) -> ListIterator {
    //     ListIterator::new(unsafe { NonNull::new_unchecked(self as _) })
    // }
}

pub struct SLListNode {
    pub next: Option<NonNull<Self>>,
}

impl SLListNode {
    #[inline]
    pub unsafe fn push(&mut self, mem: NonNull<u8>) {
        let mut item = mem.cast::<SLListNode>().as_mut();
        item.next = self.next;
        self.next = Some(mem.cast());
    }

    #[inline]
    pub fn pop(&mut self) -> Option<NonNull<u8>> {
        if let Some(mut new) = self.next {
            unsafe {
                self.next = new.as_mut().next;
                info!("Popped {:?}", self.next);
                new.as_ptr().write_bytes(0, size_of::<Self>())
            }
            Some(new.cast())
        } else {
            None
        }
    }
}

//
// pub struct ListIterator {
//     curr: Option<NonNull<DCListNode>>,
// }
// // TODO Implement a `DoubleEndedIterator`
// impl Iterator for ListIterator {
//     type Item = NonNull<DCListNode>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.curr {
//             Some(ptr) => {
//                 self.curr = unsafe { (*ptr.as_ptr()).next };
//                 self.curr
//             }
//             None => None,
//         }
//     }
// }
//
// impl ListIterator {
//     pub fn new(head: NonNull<DCListNode>) -> ListIterator {
//         ListIterator { curr: Some(head) }
//     }
// }
