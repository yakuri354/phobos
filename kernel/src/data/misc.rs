use boot_lib::PHYS_MAP_OFFSET;
use core::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};
use x86_64::{
    structures::paging::{Page, PhysFrame},
    PhysAddr, VirtAddr,
};

pub trait Pointable {
    fn pointer(&self) -> NonNull<u8>;
    fn from_pointer(ptr: NonNull<u8>) -> Self;
}

impl Pointable for PhysAddr {
    fn pointer(&self) -> NonNull<u8> {
        NonNull::new((self.as_u64() + PHYS_MAP_OFFSET) as *mut _).unwrap()
    }

    fn from_pointer(ptr: NonNull<u8>) -> Self {
        PhysAddr::new(ptr.as_ptr() as u64 - PHYS_MAP_OFFSET)
    }
}

impl Pointable for PhysFrame {
    fn pointer(&self) -> NonNull<u8> {
        self.start_address().pointer()
    }

    fn from_pointer(ptr: NonNull<u8>) -> Self {
        PhysFrame::containing_address(PhysAddr::from_pointer(ptr))
    }
}

impl Pointable for VirtAddr {
    fn pointer(&self) -> NonNull<u8> {
        NonNull::new(self.as_ptr::<u8>() as *mut _).unwrap()
    }

    fn from_pointer(ptr: NonNull<u8>) -> Self {
        VirtAddr::from_ptr(ptr.as_ptr())
    }
}

impl Pointable for Page {
    fn pointer(&self) -> NonNull<u8> {
        self.start_address().pointer()
    }

    fn from_pointer(ptr: NonNull<u8>) -> Self {
        Page::containing_address(VirtAddr::from_pointer(ptr))
    }
}

pub struct Global<T>(T);

unsafe impl<T> Send for Global<T> {}

impl<T> Deref for Global<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Global<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Global<T> {
    pub(crate) const fn new(val: T) -> Self {
        Self(val)
    }
}
