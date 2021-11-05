use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::UnsafeCell;
use core::hint::spin_loop;
use core::ops::{Deref, DerefMut, Drop};

/// A TaS spinlock implemented using atomics
pub struct Spinlock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>
}

pub struct SpinlockGuard<'a, T: 'a> {
    inner: &'a Spinlock<T>
}

impl<T> Spinlock<T> {
    pub fn new(data: T) -> Self {
        Spinlock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data)
        }
    }
    /// Locks the spinlock and returns the guard
    pub fn lock(&self) -> SpinlockGuard<T> {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire).is_err() {
            spin_loop()
        }
        SpinlockGuard {
            inner: &self
        }
    }
    fn release(&self) {
        self.lock.store(false, Ordering::Release)
    }
}

impl<T> Deref for SpinlockGuard<'_, T> {
    type Target = T;fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.data.get() }
    }
}

impl<T> DerefMut for SpinlockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner.data.get() }
    }
}

impl<T> Drop for SpinlockGuard<'_, T> {
    fn drop(&mut self) {
        self.inner.release()
    }
}

unsafe impl<T> Send for Spinlock<T> where T: Send {}
unsafe impl<T> Sync for Spinlock<T> where T: Send {}
unsafe impl<T> Send for SpinlockGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for SpinlockGuard<'_, T> where T: Send + Sync {}