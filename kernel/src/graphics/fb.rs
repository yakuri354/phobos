use crate::{data::late_init::LateInit, sync::irq_lock::IRQLocked};
use alloc::{vec, vec::Vec};
use boot_lib::PHYS_MAP_OFFSET;
use core::{
    cmp::min,
    convert::{Infallible, TryInto},
    ops::DerefMut,
    ptr::NonNull,
};
use embedded_graphics::{
    mono_font::mapping::GlyphMapping,
    primitives::{Circle, PrimitiveStyle, StyledDrawable},
};
use embedded_graphics_core::{
    geometry::Dimensions,
    pixelcolor::{raw::ToBytes, Bgr888},
    prelude::{DrawTarget, IntoStorage, Point, RgbColor, Size},
    primitives::Rectangle,
    Pixel,
};
use log::info;
use uefi::proto::console::{
    gop::{ModeInfo, PixelFormat},
    text::Color,
};

pub static GLOBAL_FB: IRQLocked<LateInit<FbDisplay>> = IRQLocked::new(LateInit::new());

pub struct FbDisplay {
    pub mode: ModeInfo,
    pub buffer: Vec<u32>,
    pub base: NonNull<u32>,
    pub size: u64,
}

impl Dimensions for FbDisplay {
    fn bounding_box(&self) -> Rectangle {
        let res = self.mode.resolution();
        Rectangle::new(Point::new(0, 0), Size::new(res.0 as _, res.1 as _))
    }
}

impl DrawTarget for FbDisplay {
    type Color = embedded_graphics::pixelcolor::Bgr888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let Size { height, width } = self.bounding_box().size;
        for Pixel(Point { x, y }, color) in pixels {
            // Check if the pixel coordinates are out of bounds.
            // `DrawTarget` implementation are required to discard any out of bounds
            // pixels without returning an error or causing a panic.
            if x <= width as _ && y <= height as _ && x >= 0 && y >= 0 {
                // Calculate the index in the framebuffer.
                let offset = x + y * self.mode.stride() as i32;
                self.buffer[offset as usize] = color.into_storage();
                // unsafe {
                //     let ptr: *mut u8 = self.base.as_ptr().offset(offset as _).cast();
                //     ptr.offset(0).write(color.b());
                //     ptr.offset(1).write(color.g());
                //     ptr.offset(2).write(color.r());
                // }
            } else {
                info!("Skipped") // FIXME
            }
        }

        Ok(())
    }
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
}

pub fn clear() {
    let mut fb = GLOBAL_FB.lock();
    let rect = fb.bounding_box();
    fb.fill_solid(&rect, Bgr888::BLACK);
    fb.flush();
}
