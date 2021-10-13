use crate::mm::paging::{RawPML4E, PagingChainRef};

/// The pointer must be aligned and point to a valid page
pub unsafe fn init_page_table(buf: *mut u64) {
    for i in 0..512 {
        let ptr = buf.offset(i);
        *ptr = RawPML4E::new().into();
    }
}

/// Initializes 4 paging layers on consecutive memory
pub fn init_linear_pt_array(buf: &mut [u8]) -> Option<PagingChainRef> {
    let ptr = buf.as_mut_ptr();
    if buf.len() != 0x1000 * 4 || (ptr as u64) % 0x1000 != 0 {
        None
    } else {
        for i in 0..4 {
            unsafe { init_page_table(ptr.offset(i * 0x1000) as *mut u64) }
        }
        Some(PagingChainRef(
            PML4E::
        ))
    }
}