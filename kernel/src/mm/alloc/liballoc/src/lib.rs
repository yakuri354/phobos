#![no_std]
#![feature(c_variadic)]

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod bindings;

extern crate alloc;

use alloc::string::String;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};
use cty::*;
use log::debug;
use printf_compat::{format, output};

pub type FnLock = fn() -> bool;
pub type FnUnlock = fn() -> bool;
pub type FnAllocPages = fn(i32) -> Option<NonNull<u8>>;
pub type FnFreePages = fn(NonNull<u8>, i32) -> bool;

static GLOBAL_LIB_ALLOC_INIT: AtomicBool = AtomicBool::new(false);
static mut GLOBAL_LIB_ALLOC: Option<LibAllocAllocatorData> = None;

struct LibAllocAllocatorData {
    lock: FnLock,
    unlock: FnUnlock,
    alloc_pages: FnAllocPages,
    free_pages: FnFreePages,
}

pub unsafe fn init(
    lock: FnLock,
    unlock: FnUnlock,
    alloc_pages: FnAllocPages,
    free_pages: FnFreePages,
) {
    GLOBAL_LIB_ALLOC_INIT
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .unwrap();
    GLOBAL_LIB_ALLOC = Some(LibAllocAllocatorData {
        lock,
        unlock,
        alloc_pages,
        free_pages,
    })
}

#[no_mangle]
unsafe extern "C" fn liballoc_lock() -> c_int {
    if (GLOBAL_LIB_ALLOC
        .as_ref()
        .expect("liballoc not initialized")
        .lock)()
    {
        0
    } else {
        1
    }
}

#[no_mangle]
unsafe extern "C" fn liballoc_unlock() -> c_int {
    if (GLOBAL_LIB_ALLOC
        .as_ref()
        .expect("liballoc not initialized")
        .unlock)()
    {
        0
    } else {
        1
    }
}

#[no_mangle]
unsafe extern "C" fn liballoc_alloc(count: c_int) -> *mut c_void {
    (GLOBAL_LIB_ALLOC
        .as_ref()
        .expect("liballoc not initialized")
        .alloc_pages)(count)
    .map(NonNull::as_ptr)
    .unwrap_or(0_usize as _) as _
}

#[no_mangle]
unsafe extern "C" fn liballoc_free(addr: *mut c_void, count: c_int) -> c_int {
    if (GLOBAL_LIB_ALLOC
        .as_ref()
        .expect("liballoc not initialized")
        .free_pages)(NonNull::new_unchecked(addr as _), count)
    {
        0
    } else {
        1
    }
}

#[no_mangle]
unsafe extern "C" fn liballoc_printf(str: *const c_char, mut args: ...) -> c_int {
    let mut s = String::new();
    let bytes_written = format(str, args.as_va_list(), output::fmt_write(&mut s));
    debug!("{}", s);
    bytes_written
}

pub struct LiballocAllocator;

unsafe impl GlobalAlloc for LiballocAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        debug_assert!(GLOBAL_LIB_ALLOC_INIT.load(Ordering::Relaxed));
        bindings::__kmalloc(layout.size()) as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        debug_assert!(GLOBAL_LIB_ALLOC_INIT.load(Ordering::Relaxed));
        bindings::__kfree(ptr as _)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _: Layout, new_size: usize) -> *mut u8 {
        debug_assert!(GLOBAL_LIB_ALLOC_INIT.load(Ordering::Relaxed));
        bindings::__krealloc(ptr as _, new_size) as _
    }
}
