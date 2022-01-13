use crate::{arch::interrupt::*, mm::alloc::GlobalAllocator};
use core::{
    alloc::{GlobalAlloc, Layout},
    ops::{Deref, DerefMut},
};
use liballoc::LiballocAllocator;
use spin::{Mutex, MutexGuard};

// We don't need a real spinlock since the kernel does not support SMP yet

pub struct IRQLocked<T> {
    // inner: Mutex<T>,
    val: T,
}

impl<T> IRQLocked<T> {
    pub const fn new(val: T) -> IRQLocked<T> {
        IRQLocked {
            // inner: Mutex::new(val),
            val,
        }
    }
    pub fn lock(&self) -> InterruptGuard<T> {
        // let guard = self.inner.lock();
        let flag = are_enabled();
        disable();
        unsafe { InterruptGuard::new(&mut *(&self.val as *const _ as *mut _), flag) }
    }
    pub fn is_locked(&self) -> bool {
        false
    }
}

unsafe impl<T> Sync for IRQLocked<T> {}

pub struct InterruptGuard<'a, T> {
    // inner: MutexGuard<'a, T>,
    val: &'a mut T,
    int_flag: bool,
}

impl<'a, T> InterruptGuard<'a, T> {
    // fn new(inner: MutexGuard<'a, T>, int_flag: bool) -> Self {
    //     InterruptGuard { inner, int_flag }
    // }
    fn new(val: &'a mut T, int_flag: bool) -> Self {
        InterruptGuard { val, int_flag }
    }
}

impl<'a, T> Drop for InterruptGuard<'a, T> {
    fn drop(&mut self) {
        if self.int_flag {
            enable()
        }
    }
}

impl<'a, T> Deref for InterruptGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.val
    }
}

impl<'a, T> DerefMut for InterruptGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.val
    }
}
