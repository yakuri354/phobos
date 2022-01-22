use crate::arch::interrupt::*;
use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};
use log::info;
// We don't need a real spinlock since the kernel does not support SMP yet

pub struct IRQLocked<T> {
    // inner: Mutex<T>,
    locked: AtomicBool,
    val: UnsafeCell<T>,
}

impl<T> IRQLocked<T> {
    pub const fn new(val: T) -> IRQLocked<T> {
        IRQLocked {
            // inner: Mutex::new(val),
            locked: AtomicBool::new(false),
            val: UnsafeCell::new(val),
        }
    }
    pub fn lock(&self) -> InterruptGuard<T> {
        // let guard = self.inner.lock();
        if self
            .locked
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            panic!("IRQLocked locked twice");
        }
        let flag = are_enabled();
        disable();
        unsafe { InterruptGuard::new(unsafe { &mut *self.val.get() }, flag, &self.locked) }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }
}

unsafe impl<T> Sync for IRQLocked<T> {}
unsafe impl<T> Send for IRQLocked<T> {}

pub struct InterruptGuard<'a, T> {
    // inner: MutexGuard<'a, T>,
    val: &'a mut T,
    locked: &'a AtomicBool,
    int_flag: bool,
}

impl<'a, T> InterruptGuard<'a, T> {
    // fn new(inner: MutexGuard<'a, T>, int_flag: bool) -> Self {
    //     InterruptGuard { inner, int_flag }
    // }
    fn new(val: &'a mut T, int_flag: bool, locked: &'a AtomicBool) -> Self {
        InterruptGuard {
            val,
            int_flag,
            locked,
        }
    }
}

impl<'a, T> Drop for InterruptGuard<'a, T> {
    fn drop(&mut self) {
        if self.int_flag {
            enable()
        }
        self.locked.store(false, Ordering::Release);
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
