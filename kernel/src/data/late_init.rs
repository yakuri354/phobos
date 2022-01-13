use core::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};
use log::info;

pub struct LateInit<T> {
    init: bool,
    data: MaybeUninit<T>,
}

impl<T> LateInit<T> {
    /// Create a new, uninitialized LateInit
    pub const fn new() -> Self {
        LateInit {
            init: false,
            data: MaybeUninit::uninit(),
        }
    }

    /// Initialize the value in this LateInit with the provided instance
    pub fn init(&self, data: T) -> bool {
        if self.init {
            false
        } else {
            let mut_ref = unsafe { &mut *(self as *const _ as *mut Self) };
            mut_ref.data.write(data);
            mut_ref.init = true;
            true
        }
    }
}

impl<T> Deref for LateInit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(self.init, "LateInit dereferenced but uninitialized");
        unsafe { self.data.assume_init_ref() }
    }
}

impl<T> DerefMut for LateInit<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(self.init, "LateInit dereferenced but uninitialized");
        unsafe { self.data.assume_init_mut() }
    }
}
