use crate::{
    diag::terminal::Terminal,
    graphics::fb::{FbDisplay, GLOBAL_FB},
};
use alloc::string::{String, ToString};
use core::{
    default::Default,
    fmt::{Arguments, Write},
    iter::{repeat, FromIterator},
    ops::{Deref, DerefMut},
};
use embedded_graphics::text::{renderer::TextRenderer, Baseline};
use embedded_graphics_core::{
    geometry::Dimensions,
    pixelcolor::{Bgr888, Gray8, RgbColor},
    prelude::{DrawTarget, IntoStorage, Point, Size},
};
use fontdue::{layout::GlyphRasterConfig, Font, FontSettings, Metrics};

const TAB_SIZE: u32 = 4;
const PADDING_PX: u32 = 3;

pub struct FbTextRender {
    font: Font,
    font_size: u32,
    current_pos: (u32, u32),
    bg: Bgr888,
}

// TODO Support ANSI control codes

impl FbTextRender {
    pub fn new<T: Deref<Target = [u8]>>(font_data: T, font_size: u32, bg: Bgr888) -> Self {
        GLOBAL_FB.lock().fill(bg);
        Self {
            font: Font::from_bytes(
                font_data,
                FontSettings {
                    scale: font_size as _,
                    ..Default::default()
                },
            )
            .expect("Failed to parse font"),
            font_size,
            current_pos: (PADDING_PX, PADDING_PX),
            bg,
        }
    }

    pub fn write_fmt_colored(&mut self, args: Arguments<'_>, color: Bgr888) -> core::fmt::Result {
        let str = args.to_string();
        self.write_str_colored(&str, color)
    }

    pub fn write_str_colored(&mut self, string: &str, color: Bgr888) -> core::fmt::Result {
        for ch in string.chars() {
            self.write_char_colored_impl(ch, color)?
        }

        GLOBAL_FB.lock().flush();

        Ok(())
    }

    pub fn write_char_colored(&mut self, ch: char, color: Bgr888) -> core::fmt::Result {
        self.write_char_colored_impl(ch, color)
            .map(|_| GLOBAL_FB.lock().flush())
    }

    fn write_char_colored_impl(&mut self, ch: char, color: Bgr888) -> core::fmt::Result {
        if ch.is_control() {
            match ch {
                '\n' => self.advance_line(&mut **GLOBAL_FB.lock()),
                '\r' => self.current_pos.0 = 0,
                '\t' => {
                    self.write_str_colored(
                        &String::from_iter(repeat(' ').take(TAB_SIZE as usize)),
                        color,
                    )?;
                }
                _ => {}
            }
        } else {
            self.draw_char(ch, color)
        }
        Ok(())
    }

    fn alloc_place_for_char(&mut self, fb: &mut FbDisplay, width: u32) {
        let res = fb.mode.resolution();
        if self.current_pos.0 + width + PADDING_PX >= res.0 as u32 {
            self.advance_line(fb);
        }
    }

    fn advance_line(&mut self, fb: &mut FbDisplay) {
        let res = fb.mode.resolution();
        let amount = self.current_pos.1 + self.font_size + PADDING_PX - (res.1 as u32) + 1;
        fb.scroll_up(amount as _, self.bg);
        self.current_pos.1 = res.1 as u32 - amount;
        self.current_pos.0 = PADDING_PX;
    }

    fn draw_char(&mut self, ch: char, color: Bgr888) {
        let (met, bmp) = self.font.rasterize(ch, self.font_size as f32);

        let mut fb = GLOBAL_FB.lock();
        self.alloc_place_for_char(&mut **fb, met.width as _);

        for i in 0..met.height {
            for j in 0..met.width {
                let stride = fb.mode.stride();
                let fb_idx =
                    self.current_pos.0 as usize + j + (self.current_pos.1 as usize + i) * stride;
                let bmp_idx = i * met.width + j;

                let new_r = (color.r() * bmp[bmp_idx]) / u8::MAX;
                let new_g = (color.g() * bmp[bmp_idx]) / u8::MAX;
                let new_b = (color.b() * bmp[bmp_idx]) / u8::MAX;

                fb.buffer[fb_idx] = Bgr888::new(new_r, new_g, new_b)
            }
        }
    }
}

impl Write for FbTextRender {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str_colored(s, Bgr888::WHITE)
    }

    fn write_char(&mut self, ch: char) -> core::fmt::Result {
        self.write_char_colored(ch, Bgr888::WHITE)
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> core::fmt::Result {
        self.write_fmt_colored(args, Bgr888::WHITE)
    }
}
