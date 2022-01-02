use crate::arch::bit_ops::*;
use core::mem::{size_of, transmute};

const SHIFT_BITS_OFFSET: usize = USIZE_BITS.log2();
const SHIFT_MASK: usize = (1 << SHIFT_BITS_OFFSET) - 1;

pub struct BitMapPtr {
    data: *mut usize,
    size_bits: usize
}

impl BitMapPtr {
    unsafe fn index2offsets(&self, index: usize) -> (*mut usize, usize) {
        let offset = (index & !SHIFT_MASK) >> SHIFT_BITS_OFFSET;
        let shift = index & SHIFT_MASK;
        (self.data.offset(offset as _), shift)
    }

    #[inline]
    pub unsafe fn flip_at(&mut self, index: usize) {
        let (ent, shift) = self.index2offsets(index);
        *ent = *ent ^ (1 << shift);
    }

    #[inline]
    pub unsafe fn set_at(&mut self, index: usize) {
        let (ent, shift) = self.index2offsets(index);
        *ent = *ent | (1 << shift)
    }

    #[inline]
    pub unsafe fn get_at(&self, index: usize) -> bool {
        let (ent, shift) = self.index2offsets(index);
        transmute(((*ent >> shift) & 1) as u8)
    }
}