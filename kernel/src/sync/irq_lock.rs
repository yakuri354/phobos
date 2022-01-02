use crate::arch::interrupt::*;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};
use spin::{Mutex, MutexGuard};

pub struct IRQSpinlock<T> {
    inner: Mutex<T>,
}

impl<T> IRQSpinlock<T> {
    pub const fn new(val: T) -> IRQSpinlock<T> {
        IRQSpinlock {
            inner: Mutex::new(val),
        }
    }
    pub fn lock(&self) -> InterruptGuard<T> {
        let guard = self.inner.lock();
        let flag = are_enabled();
        disable();
        InterruptGuard::new(guard, flag)
    }
    pub fn is_locked(&self) -> bool {
        self.inner.is_locked()
    }
}

struct InterruptGuard<'a, T> {
    inner: MutexGuard<'a, T>,
    int_flag: bool,
}

impl<'a, T> InterruptGuard<'a, T> {
    fn new(inner: MutexGuard<'a, T>, int_flag: bool) -> Self {
        InterruptGuard { inner, int_flag }
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
        self.inner.deref()
    }
}

impl<'a, T> DerefMut for InterruptGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}
