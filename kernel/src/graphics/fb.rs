use crate::{data::late_init::LateInit, sync::irq_lock::IRQLocked};
use alloc::{vec, vec::Vec};

use core::ptr::NonNull;

use embedded_graphics_core::{
    pixelcolor::Bgr888,
    prelude::{DrawTarget, IntoStorage, Point, PointsIter, RgbColor, Size},
    primitives::Rectangle,
    Pixel,
};

use uefi::proto::console::gop::ModeInfo;

pub static GLOBAL_FB: IRQLocked<LateInit<FbDisplay>> = IRQLocked::new(LateInit::new());

pub struct FbDisplay {
    pub mode: ModeInfo,
    pub buffer: Vec<u32>,
    pub base: NonNull<u32>,
    pub size: u64,
}

impl FbDisplay {
    pub fn new(base: NonNull<u32>, mode: ModeInfo) -> Self {
        let size = mode.resolution().1 * mode.stride();
        Self {
            size: size as u64,
            base,
            buffer: vec![0; size],
            mode,
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            self.base
                .as_ptr()
                .copy_from_nonoverlapping(self.buffer.as_slice().as_ptr(), self.size as _);
        }
    }

    pub fn scroll_up(&mut self, height: usize, bg: Bgr888) {
        let high = self.mode.stride() * height;
        let low = self.mode.stride() * self.mode.resolution().1;
        self.buffer[0..(high - 1)].fill(bg.into_storage());
        self.buffer.copy_within(high..low, 0)
    }

    pub fn fill(&mut self, color: Bgr888) {
        self.buffer.fill(color.into_storage());
    }

    pub fn write(&mut self, pos: usize, color: Bgr888) {
        self.buffer[pos] = color.into_storage();
    }
}
