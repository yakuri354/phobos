use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct LateInit<T> {
    init: AtomicBool,
    data: MaybeUninit<T>,
}

impl<T> LateInit<T> {
    /// Create a new, uninitialized LateInit
    pub const fn new() -> Self {
        LateInit {
            init: AtomicBool::new(false),
            data: MaybeUninit::uninit(),
        }
    }

    /// Initialize the value in this LateInit with the provided instance
    pub unsafe fn init(&self, data: T) -> bool {
        let mut_ref = &mut *(self as *const _ as *mut Self);

        if mut_ref
            .init
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            false
        } else {
            mut_ref.data.write(data);
            true
        }
    }
}

impl<T> Deref for LateInit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(
            self.init.load(Ordering::SeqCst),
            "LateInit dereferenced but uninitialized"
        );
        unsafe { self.data.assume_init_ref() }
    }
}
