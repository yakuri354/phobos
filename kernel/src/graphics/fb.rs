use crate::{data::late_init::LateInit, sync::irq_lock::IRQLocked};
use core::ptr::NonNull;

pub const GLOBAL_FB: IRQLocked<LateInit<FrameBuffer>> = IRQLocked::new(LateInit::new());

pub struct FrameBuffer {
    base: NonNull<u8>,
    size: u64,
}

impl FrameBuffer {
    pub fn new(base: NonNull<u8>, size: u64) -> Self {
        Self { size, base }
    }
}
